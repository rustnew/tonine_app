use actix_web::{web, HttpResponse};
use uuid::Uuid;
use serde_json::json;

use crate::model::users::{CreateUser, UpdateUser};
use crate::repositories::user_repository::UserRepository;
use crate::errors::AppError;

pub struct UserHandler;

impl UserHandler {
    pub async fn get_users(pool: web::Data<sqlx::PgPool>) -> Result<HttpResponse, AppError> {
        let users = UserRepository::find_all(&pool).await?;
        Ok(HttpResponse::Ok().json(users))
    }

    pub async fn get_user(
        pool: web::Data<sqlx::PgPool>,
        user_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let user = UserRepository::find_by_id(&pool, user_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(user))
    }

    pub async fn create_user(
        pool: web::Data<sqlx::PgPool>,
        user_data: web::Json<CreateUser>,
    ) -> Result<HttpResponse, AppError> {
        let user = UserRepository::create(&pool, &user_data.into_inner()).await?;
        Ok(HttpResponse::Created().json(user))
    }

    pub async fn update_user(
        pool: web::Data<sqlx::PgPool>,
        user_id: web::Path<Uuid>,
        user_data: web::Json<UpdateUser>,
    ) -> Result<HttpResponse, AppError> {
        let user = UserRepository::update(&pool, user_id.into_inner(), &user_data.into_inner()).await?;
        Ok(HttpResponse::Ok().json(user))
    }

    pub async fn delete_user(
        pool: web::Data<sqlx::PgPool>,
        user_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        UserRepository::delete(&pool, user_id.into_inner()).await?;
        Ok(HttpResponse::NoContent().finish())
    }

    pub async fn change_password(
        pool: web::Data<sqlx::PgPool>,
        user_id: web::Path<Uuid>,
        password_data: web::Json<ChangePasswordRequest>,
    ) -> Result<HttpResponse, AppError> {
        UserRepository::change_password(&pool, user_id.into_inner(), &password_data.new_password).await?;
        Ok(HttpResponse::Ok().json(json!({"message": "Mot de passe modifié avec succès"})))
    }
}

#[derive(serde::Deserialize)]
pub struct ChangePasswordRequest {
    pub new_password: String,
}