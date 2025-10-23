use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{ Utc};

use crate::model::tontine_rounds::{TontineRound, CreateTontineRound, UpdateTontineRound, TontineRoundWithBeneficiary, RoundStatus};
use crate::errors::AppError;

pub struct TontineRoundRepository;

impl TontineRoundRepository {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<TontineRoundWithBeneficiary>, AppError> {
        let rounds = sqlx::query(
            "SELECT tr.*, u.full_name as beneficiary_name, u.email as beneficiary_email
             FROM tontine_rounds tr
             LEFT JOIN users u ON tr.beneficiary_user_id = u.id
             ORDER BY tr.created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        let rounds_with_beneficiary = rounds.into_iter().map(|row| TontineRoundWithBeneficiary {
            id: row.get("id"),
            tontine_id: row.get("tontine_id"),
            round_number: row.get("round_number"),
            beneficiary_user_id: row.get("beneficiary_user_id"),
            amount: row.get("amount"),
            round_date: row.get("round_date"),
            status: row.get("status"),
            created_at: row.get("created_at"),
            beneficiary_name: row.get("beneficiary_name"),
            beneficiary_email: row.get("beneficiary_email"),
        }).collect();

        Ok(rounds_with_beneficiary)
    }

    pub async fn find_by_id(pool: &PgPool, round_id: Uuid) -> Result<TontineRoundWithBeneficiary, AppError> {
        let round = sqlx::query(
            "SELECT tr.*, u.full_name as beneficiary_name, u.email as beneficiary_email
             FROM tontine_rounds tr
             LEFT JOIN users u ON tr.beneficiary_user_id = u.id
             WHERE tr.id = $1"
        )
        .bind(round_id)
        .fetch_optional(pool)
        .await?;

        match round {
            Some(row) => {
                let round_with_beneficiary = TontineRoundWithBeneficiary {
                    id: row.get("id"),
                    tontine_id: row.get("tontine_id"),
                    round_number: row.get("round_number"),
                    beneficiary_user_id: row.get("beneficiary_user_id"),
                    amount: row.get("amount"),
                    round_date: row.get("round_date"),
                    status: row.get("status"),
                    created_at: row.get("created_at"),
                    beneficiary_name: row.get("beneficiary_name"),
                    beneficiary_email: row.get("beneficiary_email"),
                };
                Ok(round_with_beneficiary)
            },
            None => Err(AppError::NotFound(format!("Round avec l'ID {} non trouvé", round_id))),
        }
    }

    pub async fn find_by_tontine(pool: &PgPool, tontine_id: Uuid) -> Result<Vec<TontineRoundWithBeneficiary>, AppError> {
        let rounds = sqlx::query(
            "SELECT tr.*, u.full_name as beneficiary_name, u.email as beneficiary_email
             FROM tontine_rounds tr
             LEFT JOIN users u ON tr.beneficiary_user_id = u.id
             WHERE tr.tontine_id = $1
             ORDER BY tr.round_number ASC"
        )
        .bind(tontine_id)
        .fetch_all(pool)
        .await?;

        let rounds_with_beneficiary = rounds.into_iter().map(|row| TontineRoundWithBeneficiary {
            id: row.get("id"),
            tontine_id: row.get("tontine_id"),
            round_number: row.get("round_number"),
            beneficiary_user_id: row.get("beneficiary_user_id"),
            amount: row.get("amount"),
            round_date: row.get("round_date"),
            status: row.get("status"),
            created_at: row.get("created_at"),
            beneficiary_name: row.get("beneficiary_name"),
            beneficiary_email: row.get("beneficiary_email"),
        }).collect();

        Ok(rounds_with_beneficiary)
    }

    pub async fn find_current_round(pool: &PgPool, tontine_id: Uuid) -> Result<Option<TontineRoundWithBeneficiary>, AppError> {
        let round = sqlx::query(
            "SELECT tr.*, u.full_name as beneficiary_name, u.email as beneficiary_email
             FROM tontine_rounds tr
             LEFT JOIN users u ON tr.beneficiary_user_id = u.id
             WHERE tr.tontine_id = $1 AND tr.status = 'pending'
             ORDER BY tr.round_number ASC
             LIMIT 1"
        )
        .bind(tontine_id)
        .fetch_optional(pool)
        .await?;

        match round {
            Some(row) => {
                let round_with_beneficiary = TontineRoundWithBeneficiary {
                    id: row.get("id"),
                    tontine_id: row.get("tontine_id"),
                    round_number: row.get("round_number"),
                    beneficiary_user_id: row.get("beneficiary_user_id"),
                    amount: row.get("amount"),
                    round_date: row.get("round_date"),
                    status: row.get("status"),
                    created_at: row.get("created_at"),
                    beneficiary_name: row.get("beneficiary_name"),
                    beneficiary_email: row.get("beneficiary_email"),
                };
                Ok(Some(round_with_beneficiary))
            },
            None => Ok(None),
        }
    }

    pub async fn create(pool: &PgPool, round_data: &CreateTontineRound) -> Result<TontineRound, AppError> {
        // Vérifier si la tontine existe
        let tontine_exists = sqlx::query("SELECT id, max_members FROM tontines WHERE id = $1")
            .bind(&round_data.tontine_id)
            .fetch_optional(pool)
            .await?;

        if tontine_exists.is_none() {
            return Err(AppError::ValidationError("La tontine spécifiée n'existe pas".to_string()));
        }

        // Vérifier si le bénéficiaire existe
        let beneficiary_exists = sqlx::query("SELECT id FROM users WHERE id = $1")
            .bind(&round_data.beneficiary_user_id)
            .fetch_optional(pool)
            .await?;

        if beneficiary_exists.is_none() {
            return Err(AppError::ValidationError("Le bénéficiaire spécifié n'existe pas".to_string()));
        }

        // Vérifier si le bénéficiaire est membre de la tontine
        let is_member = sqlx::query(
            "SELECT id FROM tontine_members WHERE tontine_id = $1 AND user_id = $2 AND is_active = true"
        )
        .bind(&round_data.tontine_id)
        .bind(&round_data.beneficiary_user_id)
        .fetch_optional(pool)
        .await?;

        if is_member.is_none() {
            return Err(AppError::ValidationError("Le bénéficiaire n'est pas membre de cette tontine".to_string()));
        }

        // Vérifier si le round number est unique pour cette tontine
        let existing_round = sqlx::query(
            "SELECT id FROM tontine_rounds WHERE tontine_id = $1 AND round_number = $2"
        )
        .bind(&round_data.tontine_id)
        .bind(&round_data.round_number)
        .fetch_optional(pool)
        .await?;

        if existing_round.is_some() {
            return Err(AppError::ValidationError("Un round avec ce numéro existe déjà pour cette tontine".to_string()));
        }

        let round = sqlx::query_as::<_, TontineRound>(
            "INSERT INTO tontine_rounds (tontine_id, round_number, beneficiary_user_id, amount, round_date) 
             VALUES ($1, $2, $3, $4, $5) 
             RETURNING id, tontine_id, round_number, beneficiary_user_id, amount, round_date, status, created_at"
        )
        .bind(&round_data.tontine_id)
        .bind(&round_data.round_number)
        .bind(&round_data.beneficiary_user_id)
        .bind(&round_data.amount)
        .bind(&round_data.round_date)
        .fetch_one(pool)
        .await?;

        Ok(round)
    }



    pub async fn update(pool: &PgPool, round_id: Uuid, round_data: &UpdateTontineRound) -> Result<TontineRound, AppError> {
    // Vérifier si le round existe
    let existing_round = sqlx::query_as::<_, TontineRound>(
        "SELECT id, tontine_id, round_number, beneficiary_user_id, amount, round_date, status, created_at 
         FROM tontine_rounds WHERE id = $1"
    )
    .bind(round_id)
    .fetch_optional(pool)
    .await?;

    if existing_round.is_none() {
        return Err(AppError::NotFound(format!("Round avec l'ID {} non trouvé", round_id)));
    }

    // Construire la requête dynamiquement avec des types concrets
    let mut query = "UPDATE tontine_rounds SET ".to_string();
    let mut bind_values: Vec<String> = Vec::new();
    let mut counter = 1;

    if round_data.beneficiary_user_id.is_some() {
        bind_values.push(format!("beneficiary_user_id = ${}", counter));
        counter += 1;
    }
    if round_data.amount.is_some() {
        bind_values.push(format!("amount = ${}", counter));
        counter += 1;
    }
    if round_data.round_date.is_some() {
        bind_values.push(format!("round_date = ${}", counter));
        counter += 1;
    }
    if round_data.status.is_some() {
        bind_values.push(format!("status = ${}", counter));
        counter += 1;
    }

    if bind_values.is_empty() {
        return Ok(existing_round.unwrap());
    }

    query.push_str(&bind_values.join(", "));
    query.push_str(&format!(" WHERE id = ${} RETURNING id, tontine_id, round_number, beneficiary_user_id, amount, round_date, status, created_at", counter));

    // Construire la requête avec les valeurs réelles
    let mut query_builder = sqlx::query_as::<_, TontineRound>(&query);
    
    if let Some(beneficiary_user_id) = &round_data.beneficiary_user_id {
        query_builder = query_builder.bind(beneficiary_user_id);
    }
    if let Some(amount) = &round_data.amount {
        query_builder = query_builder.bind(amount);
    }
    if let Some(round_date) = &round_data.round_date {
        query_builder = query_builder.bind(round_date);
    }
    if let Some(status) = &round_data.status {
        let status_str: String = status.clone().into();
        query_builder = query_builder.bind(status_str);
    }
    
    query_builder = query_builder.bind(round_id);

    let round = query_builder.fetch_one(pool).await?;
    Ok(round)
}



    pub async fn delete(pool: &PgPool, round_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM tontine_rounds WHERE id = $1")
            .bind(round_id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Round avec l'ID {} non trouvé", round_id)));
        }

        Ok(())
    }

    pub async fn complete_round(pool: &PgPool, round_id: Uuid) -> Result<TontineRound, AppError> {
        let round = sqlx::query_as::<_, TontineRound>(
            "UPDATE tontine_rounds 
             SET status = 'completed', round_date = $1 
             WHERE id = $2 
             RETURNING id, tontine_id, round_number, beneficiary_user_id, amount, round_date, status, created_at"
        )
        .bind(Utc::now())
        .bind(round_id)
        .fetch_one(pool)
        .await?;

        Ok(round)
    }

    pub async fn cancel_round(pool: &PgPool, round_id: Uuid) -> Result<TontineRound, AppError> {
        let round = sqlx::query_as::<_, TontineRound>(
            "UPDATE tontine_rounds 
             SET status = 'cancelled'
             WHERE id = $1 
             RETURNING id, tontine_id, round_number, beneficiary_user_id, amount, round_date, status, created_at"
        )
        .bind(round_id)
        .fetch_one(pool)
        .await?;

        Ok(round)
    }

    pub async fn get_rounds_by_status(pool: &PgPool, status: RoundStatus) -> Result<Vec<TontineRoundWithBeneficiary>, AppError> {
        let status_str: String = status.into();
        
        let rounds = sqlx::query(
            "SELECT tr.*, u.full_name as beneficiary_name, u.email as beneficiary_email
             FROM tontine_rounds tr
             LEFT JOIN users u ON tr.beneficiary_user_id = u.id
             WHERE tr.status = $1
             ORDER BY tr.created_at DESC"
        )
        .bind(&status_str)
        .fetch_all(pool)
        .await?;

        let rounds_with_beneficiary = rounds.into_iter().map(|row| TontineRoundWithBeneficiary {
            id: row.get("id"),
            tontine_id: row.get("tontine_id"),
            round_number: row.get("round_number"),
            beneficiary_user_id: row.get("beneficiary_user_id"),
            amount: row.get("amount"),
            round_date: row.get("round_date"),
            status: row.get("status"),
            created_at: row.get("created_at"),
            beneficiary_name: row.get("beneficiary_name"),
            beneficiary_email: row.get("beneficiary_email"),
        }).collect();

        Ok(rounds_with_beneficiary)
    }

    pub async fn get_next_round_number(pool: &PgPool, tontine_id: Uuid) -> Result<i32, AppError> {
        let max_round: Option<i32> = sqlx::query_scalar(
            "SELECT MAX(round_number) FROM tontine_rounds WHERE tontine_id = $1"
        )
        .bind(tontine_id)
        .fetch_one(pool)
        .await?;

        Ok(max_round.unwrap_or(0) + 1)
    }
}