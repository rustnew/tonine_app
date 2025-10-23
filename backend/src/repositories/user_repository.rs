use sqlx::{PgPool};
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::model::users::{User, CreateUser, UpdateUser, UserResponse};
use crate::errors::AppError;

pub struct UserRepository;

impl UserRepository {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<UserResponse>, AppError> {
        let users = sqlx::query_as::<_, User>(
            "SELECT id, email, phone, full_name, password_hash, is_active, created_at, updated_at FROM users ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        Ok(users.into_iter().map(|user| user.into()).collect())
    }

    pub async fn find_by_id(pool: &PgPool, user_id: Uuid) -> Result<UserResponse, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, phone, full_name, password_hash, is_active, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        match user {
            Some(user) => Ok(user.into()),
            None => Err(AppError::NotFound(format!("Utilisateur avec l'ID {} non trouvé", user_id))),
        }
    }


    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, phone, full_name, password_hash, is_active, created_at, updated_at FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        match user {
            Some(user) => Ok(user),
            None => Err(AppError::NotFound(format!("Utilisateur avec l'email {} non trouvé", email))),
        }
    }

    
    pub async fn create(pool: &PgPool, user_data: &CreateUser) -> Result<UserResponse, AppError> {
        // Vérifier si l'email existe déjà
        let existing_user = sqlx::query("SELECT id FROM users WHERE email = $1 OR phone = $2")
            .bind(&user_data.email)
            .bind(&user_data.phone)
            .fetch_optional(pool)
            .await?;

        if existing_user.is_some() {
            return Err(AppError::ValidationError("Un utilisateur avec cet email ou téléphone existe déjà".to_string()));
        }

        // Hasher le mot de passe
        let password_hash = hash(&user_data.password, DEFAULT_COST)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (email, phone, full_name, password_hash) 
             VALUES ($1, $2, $3, $4) 
             RETURNING id, email, phone, full_name, password_hash, is_active, created_at, updated_at"
        )
        .bind(&user_data.email)
        .bind(&user_data.phone)
        .bind(&user_data.full_name)
        .bind(&password_hash)
        .fetch_one(pool)
        .await?;

        Ok(user.into())
    }

    pub async fn update(pool: &PgPool, user_id: Uuid, user_data: &UpdateUser) -> Result<UserResponse, AppError> {
        // Vérifier si l'utilisateur existe
        let existing_user = Self::find_by_id(pool, user_id).await?;

        // Construire la requête dynamiquement
        let mut query = "UPDATE users SET ".to_string();
        let mut params: Vec<String> = Vec::new();
        let mut counter = 1;

        if let Some(email) = &user_data.email {
            params.push(format!("email = ${}", counter));
            counter += 1;
        }
        if let Some(phone) = &user_data.phone {
            params.push(format!("phone = ${}", counter));
            counter += 1;
        }
        if let Some(full_name) = &user_data.full_name {
            params.push(format!("full_name = ${}", counter));
            counter += 1;
        }
        if let Some(is_active) = user_data.is_active {
            params.push(format!("is_active = ${}", counter));
            counter += 1;
        }

        if params.is_empty() {
            return Ok(existing_user);
        }

        query.push_str(&params.join(", "));
        query.push_str(&format!(" WHERE id = ${} RETURNING id, email, phone, full_name, password_hash, is_active, created_at, updated_at", counter));

        let mut query_builder = sqlx::query_as::<_, User>(&query);

        if let Some(email) = &user_data.email {
            query_builder = query_builder.bind(email);
        }
        if let Some(phone) = &user_data.phone {
            query_builder = query_builder.bind(phone);
        }
        if let Some(full_name) = &user_data.full_name {
            query_builder = query_builder.bind(full_name);
        }
        if let Some(is_active) = user_data.is_active {
            query_builder = query_builder.bind(is_active);
        }

        query_builder = query_builder.bind(user_id);

        let user = query_builder.fetch_one(pool).await?;
        Ok(user.into())
    }

    pub async fn delete(pool: &PgPool, user_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Utilisateur avec l'ID {} non trouvé", user_id)));
        }

        Ok(())
    }

    pub async fn verify_password(user: &User, password: &str) -> Result<bool, AppError> {
        verify(password, &user.password_hash)
            .map_err(|e| AppError::AuthenticationError(e.to_string()))
    }

    pub async fn change_password(pool: &PgPool, user_id: Uuid, new_password: &str) -> Result<(), AppError> {
        let password_hash = hash(new_password, DEFAULT_COST)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let result = sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2")
            .bind(&password_hash)
            .bind(user_id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Utilisateur avec l'ID {} non trouvé", user_id)));
        }

        Ok(())
    }


    pub async fn find_by_id_with_password(pool: &PgPool, user_id: Uuid) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, phone, full_name, password_hash, is_active, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        match user {
            Some(user) => Ok(user),
            None => Err(AppError::NotFound(format!("Utilisateur avec l'ID {} non trouvé", user_id))),
        }
    }

}