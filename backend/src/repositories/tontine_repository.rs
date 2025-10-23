use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;

use crate::model::tontine::{Tontine, CreateTontine, UpdateTontine};
use crate::errors::AppError;

pub struct TontineRepository;

impl TontineRepository {
    
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Tontine>, AppError> {
        let tontines = sqlx::query_as::<_, Tontine>(
            "SELECT id, name, description, amount_per_member, frequency, max_members, current_round, status, created_by, created_at, updated_at 
             FROM tontines 
             ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        Ok(tontines)
    }

    pub async fn find_by_id(pool: &PgPool, tontine_id: Uuid) -> Result<Tontine, AppError> {
        let tontine = sqlx::query_as::<_, Tontine>(
            "SELECT id, name, description, amount_per_member, frequency, max_members, current_round, status, created_by, created_at, updated_at 
             FROM tontines 
             WHERE id = $1"
        )
        .bind(tontine_id)
        .fetch_optional(pool)
        .await?;

        match tontine {
            Some(tontine) => Ok(tontine),
            None => Err(AppError::NotFound(format!("Tontine avec l'ID {} non trouvée", tontine_id))),
        }
    }

    pub async fn find_by_creator(pool: &PgPool, user_id: Uuid) -> Result<Vec<Tontine>, AppError> {
        let tontines = sqlx::query_as::<_, Tontine>(
            "SELECT id, name, description, amount_per_member, frequency, max_members, current_round, status, created_by, created_at, updated_at 
             FROM tontines 
             WHERE created_by = $1 
             ORDER BY created_at DESC"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(tontines)
    }

    pub async fn create(pool: &PgPool, tontine_data: &CreateTontine, user_id: String) -> Result<Tontine, AppError> {
    // Convertir la String en UUID
    let user_uuid = uuid::Uuid::parse_str(&user_id)
        .map_err(|_| AppError::ValidationError("ID utilisateur invalide".to_string()))?;

    // Vérifier si l'utilisateur créateur existe
    let user_exists = sqlx::query("SELECT id FROM users WHERE id = $1")
        .bind(&user_uuid) // Utiliser l'UUID converti
        .fetch_optional(pool)
        .await?;

    if user_exists.is_none() {
        return Err(AppError::ValidationError("L'utilisateur créateur n'existe pas".to_string()));
    }

    let frequency_str: String = tontine_data.frequency.clone().into();

    let tontine = sqlx::query_as::<_, Tontine>(
        "INSERT INTO tontines (name, description, amount_per_member, frequency, max_members, created_by) 
         VALUES ($1, $2, $3, $4, $5, $6) 
         RETURNING id, name, description, amount_per_member, frequency, max_members, current_round, status, created_by, created_at, updated_at"
    )
    .bind(&tontine_data.name)
    .bind(&tontine_data.description)
    .bind(&tontine_data.amount_per_member)
    .bind(&frequency_str)
    .bind(&tontine_data.max_members)
    .bind(&user_uuid) // ← ICI : Utiliser l'UUID converti
    .fetch_one(pool)
    .await?;

    Ok(tontine)
}

    pub async fn update(pool: &PgPool, tontine_id: Uuid, tontine_data: &UpdateTontine) -> Result<Tontine, AppError> {
        // Vérifier si la tontine existe
        let existing = Self::find_by_id(pool, tontine_id).await?;

        // Mettre à jour seulement les champs fournis
        let name = tontine_data.name.as_ref().unwrap_or(&existing.name);
        let description = tontine_data.description.as_ref().or(existing.description.as_ref());
        let amount_per_member = tontine_data.amount_per_member.unwrap_or(existing.amount_per_member);
        let frequency = if let Some(freq) = &tontine_data.frequency {
            freq.clone().into()
        } else {
            existing.frequency
        };
        let max_members = tontine_data.max_members.unwrap_or(existing.max_members);
        let status = if let Some(stat) = &tontine_data.status {
            stat.clone().into()
        } else {
            existing.status
        };

        let tontine = sqlx::query_as::<_, Tontine>(
            "UPDATE tontines SET name = $1, description = $2, amount_per_member = $3, frequency = $4, max_members = $5, status = $6, updated_at = $7 
            WHERE id = $8 
            RETURNING id, name, description, amount_per_member, frequency, max_members, current_round, status, created_by, created_at, updated_at"
        )
        .bind(name)
        .bind(description)
        .bind(amount_per_member)
        .bind(frequency)
        .bind(max_members)
        .bind(status)
        .bind(Utc::now())
        .bind(tontine_id)
        .fetch_one(pool)
        .await?;

        Ok(tontine)
    }
   


    pub async fn delete(pool: &PgPool, tontine_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM tontines WHERE id = $1")
            .bind(tontine_id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Tontine avec l'ID {} non trouvée", tontine_id)));
        }

        Ok(())
    }

    pub async fn get_active_tontines(pool: &PgPool) -> Result<Vec<Tontine>, AppError> {
        let tontines = sqlx::query_as::<_, Tontine>(
            "SELECT id, name, description, amount_per_member, frequency, max_members, current_round, status, created_by, created_at, updated_at 
             FROM tontines 
             WHERE status = 'active' 
             ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        Ok(tontines)
    }

    pub async fn increment_round(pool: &PgPool, tontine_id: Uuid) -> Result<Tontine, AppError> {
        let tontine = sqlx::query_as::<_, Tontine>(
            "UPDATE tontines 
             SET current_round = current_round + 1, updated_at = $1 
             WHERE id = $2 
             RETURNING id, name, description, amount_per_member, frequency, max_members, current_round, status, created_by, created_at, updated_at"
        )
        .bind(Utc::now())
        .bind(tontine_id)
        .fetch_one(pool)
        .await?;

        Ok(tontine)
    }

    pub async fn get_tontine_with_creator(pool: &PgPool, tontine_id: Uuid) -> Result<TontineWithCreator, AppError> {
        let tontine = sqlx::query(
            "SELECT t.*, u.full_name as creator_name, u.email as creator_email 
             FROM tontines t 
             JOIN users u ON t.created_by = u.id 
             WHERE t.id = $1"
        )
        .bind(tontine_id)
        .fetch_optional(pool)
        .await?;

        match tontine {
            Some(row) => {
                let tontine_with_creator = TontineWithCreator {
                    id: row.get("id"),
                    name: row.get("name"),
                    description: row.get("description"),
                    amount_per_member: row.get("amount_per_member"),
                    frequency: row.get("frequency"),
                    max_members: row.get("max_members"),
                    current_round: row.get("current_round"),
                    status: row.get("status"),
                    created_by: row.get("created_by"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    creator_name: row.get("creator_name"),
                    creator_email: row.get("creator_email"),
                };
                Ok(tontine_with_creator)
            },
            None => Err(AppError::NotFound(format!("Tontine avec l'ID {} non trouvée", tontine_id))),
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct TontineWithCreator {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub amount_per_member: Decimal,
    pub frequency: String,
    pub max_members: i32,
    pub current_round: i32,
    pub status: String,
    pub created_by: Uuid,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub creator_name: String,
    pub creator_email: String,
}