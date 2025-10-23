use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{ Utc};

use crate::model::contributions::{Contribution, CreateContribution, UpdateContribution, ContributionWithDetails};
use crate::errors::AppError;

pub struct ContributionRepository;

impl ContributionRepository {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<ContributionWithDetails>, AppError> {
        let contributions = sqlx::query(
            "SELECT c.*, u.full_name as member_name, tr.round_number, t.name as tontine_name
             FROM contributions c
             JOIN tontine_members tm ON c.member_id = tm.id
             JOIN users u ON tm.user_id = u.id
             JOIN tontine_rounds tr ON c.tontine_round_id = tr.id
             JOIN tontines t ON tr.tontine_id = t.id
             ORDER BY c.created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        let contributions_with_details = contributions.into_iter().map(|row| ContributionWithDetails {
            id: row.get("id"),
            tontine_round_id: row.get("tontine_round_id"),
            member_id: row.get("member_id"),
            amount: row.get("amount"),
            payment_date: row.get("payment_date"),
            payment_method: row.get("payment_method"),
            payment_status: row.get("payment_status"),
            created_at: row.get("created_at"),
            member_name: row.get("member_name"),
            round_number: row.get("round_number"),
            tontine_name: row.get("tontine_name"),
        }).collect();

        Ok(contributions_with_details)
    }

    pub async fn find_by_id(pool: &PgPool, contribution_id: Uuid) -> Result<ContributionWithDetails, AppError> {
        let contribution = sqlx::query(
            "SELECT c.*, u.full_name as member_name, tr.round_number, t.name as tontine_name
             FROM contributions c
             JOIN tontine_members tm ON c.member_id = tm.id
             JOIN users u ON tm.user_id = u.id
             JOIN tontine_rounds tr ON c.tontine_round_id = tr.id
             JOIN tontines t ON tr.tontine_id = t.id
             WHERE c.id = $1"
        )
        .bind(contribution_id)
        .fetch_optional(pool)
        .await?;

        match contribution {
            Some(row) => {
                let contribution_with_details = ContributionWithDetails {
                    id: row.get("id"),
                    tontine_round_id: row.get("tontine_round_id"),
                    member_id: row.get("member_id"),
                    amount: row.get("amount"),
                    payment_date: row.get("payment_date"),
                    payment_method: row.get("payment_method"),
                    payment_status: row.get("payment_status"),
                    created_at: row.get("created_at"),
                    member_name: row.get("member_name"),
                    round_number: row.get("round_number"),
                    tontine_name: row.get("tontine_name"),
                };
                Ok(contribution_with_details)
            },
            None => Err(AppError::NotFound(format!("Contribution avec l'ID {} non trouvée", contribution_id))),
        }
    }

    pub async fn find_by_round(pool: &PgPool, round_id: Uuid) -> Result<Vec<ContributionWithDetails>, AppError> {
        let contributions = sqlx::query(
            "SELECT c.*, u.full_name as member_name, tr.round_number, t.name as tontine_name
             FROM contributions c
             JOIN tontine_members tm ON c.member_id = tm.id
             JOIN users u ON tm.user_id = u.id
             JOIN tontine_rounds tr ON c.tontine_round_id = tr.id
             JOIN tontines t ON tr.tontine_id = t.id
             WHERE c.tontine_round_id = $1
             ORDER BY c.payment_date DESC"
        )
        .bind(round_id)
        .fetch_all(pool)
        .await?;

        let contributions_with_details = contributions.into_iter().map(|row| ContributionWithDetails {
            id: row.get("id"),
            tontine_round_id: row.get("tontine_round_id"),
            member_id: row.get("member_id"),
            amount: row.get("amount"),
            payment_date: row.get("payment_date"),
            payment_method: row.get("payment_method"),
            payment_status: row.get("payment_status"),
            created_at: row.get("created_at"),
            member_name: row.get("member_name"),
            round_number: row.get("round_number"),
            tontine_name: row.get("tontine_name"),
        }).collect();

        Ok(contributions_with_details)
    }

    pub async fn find_by_member(pool: &PgPool, member_id: Uuid) -> Result<Vec<ContributionWithDetails>, AppError> {
        let contributions = sqlx::query(
            "SELECT c.*, u.full_name as member_name, tr.round_number, t.name as tontine_name
             FROM contributions c
             JOIN tontine_members tm ON c.member_id = tm.id
             JOIN users u ON tm.user_id = u.id
             JOIN tontine_rounds tr ON c.tontine_round_id = tr.id
             JOIN tontines t ON tr.tontine_id = t.id
             WHERE c.member_id = $1
             ORDER BY c.created_at DESC"
        )
        .bind(member_id)
        .fetch_all(pool)
        .await?;

        let contributions_with_details = contributions.into_iter().map(|row| ContributionWithDetails {
            id: row.get("id"),
            tontine_round_id: row.get("tontine_round_id"),
            member_id: row.get("member_id"),
            amount: row.get("amount"),
            payment_date: row.get("payment_date"),
            payment_method: row.get("payment_method"),
            payment_status: row.get("payment_status"),
            created_at: row.get("created_at"),
            member_name: row.get("member_name"),
            round_number: row.get("round_number"),
            tontine_name: row.get("tontine_name"),
        }).collect();

        Ok(contributions_with_details)
    }

    pub async fn create(pool: &PgPool, contribution_data: &CreateContribution) -> Result<Contribution, AppError> {
        // Vérifier si le round existe
        let round_exists = sqlx::query(
            "SELECT id, amount FROM tontine_rounds WHERE id = $1"
        )
        .bind(&contribution_data.tontine_round_id)
        .fetch_optional(pool)
        .await?;

        if round_exists.is_none() {
            return Err(AppError::ValidationError("Le round spécifié n'existe pas".to_string()));
        }

        // Vérifier si le membre existe
        let member_exists = sqlx::query(
            "SELECT id, tontine_id FROM tontine_members WHERE id = $1 AND is_active = true"
        )
        .bind(&contribution_data.member_id)
        .fetch_optional(pool)
        .await?;

        if member_exists.is_none() {
            return Err(AppError::ValidationError("Le membre spécifié n'existe pas ou n'est pas actif".to_string()));
        }

        // Vérifier si le membre appartient au round
        let member_belongs_to_round = sqlx::query(
            "SELECT tm.id 
             FROM tontine_members tm
             JOIN tontine_rounds tr ON tm.tontine_id = tr.tontine_id
             WHERE tm.id = $1 AND tr.id = $2"
        )
        .bind(&contribution_data.member_id)
        .bind(&contribution_data.tontine_round_id)
        .fetch_optional(pool)
        .await?;

        if member_belongs_to_round.is_none() {
            return Err(AppError::ValidationError("Le membre n'appartient pas à cette tontine/round".to_string()));
        }

        // Vérifier si le membre a déjà cotisé pour ce round
        let existing_contribution = sqlx::query(
            "SELECT id FROM contributions WHERE tontine_round_id = $1 AND member_id = $2"
        )
        .bind(&contribution_data.tontine_round_id)
        .bind(&contribution_data.member_id)
        .fetch_optional(pool)
        .await?;

        if existing_contribution.is_some() {
            return Err(AppError::ValidationError("Le membre a déjà cotisé pour ce round".to_string()));
        }

        let payment_method_str: String = contribution_data.payment_method.clone().into();

        let contribution = sqlx::query_as::<_, Contribution>(
            "INSERT INTO contributions (tontine_round_id, member_id, amount, payment_method) 
             VALUES ($1, $2, $3, $4) 
             RETURNING id, tontine_round_id, member_id, amount, payment_date, payment_method, payment_status, created_at"
        )
        .bind(&contribution_data.tontine_round_id)
        .bind(&contribution_data.member_id)
        .bind(&contribution_data.amount)
        .bind(&payment_method_str)
        .fetch_one(pool)
        .await?;

        Ok(contribution)
    }


    pub async fn update(pool: &PgPool, contribution_id: Uuid, contribution_data: &UpdateContribution) -> Result<Contribution, AppError> {
    // Vérifier si la contribution existe
    let existing_contribution = sqlx::query_as::<_, Contribution>(
        "SELECT id, tontine_round_id, member_id, amount, payment_date, payment_method, payment_status, created_at 
         FROM contributions WHERE id = $1"
    )
    .bind(contribution_id)
    .fetch_optional(pool)
    .await?;

    let existing = match existing_contribution {
        Some(contribution) => contribution,
        None => return Err(AppError::NotFound(format!("Contribution avec l'ID {} non trouvée", contribution_id))),
    };

    // Utiliser les nouvelles valeurs ou conserver les anciennes
    let amount = contribution_data.amount.unwrap_or(existing.amount);
    let payment_method = match &contribution_data.payment_method {
        Some(method) => {
            let method_str: String = method.clone().into();
            method_str
        },
        None => existing.payment_method.unwrap_or_default(),
    };
    let payment_status = match &contribution_data.payment_status {
        Some(status) => {
            let status_str: String = status.clone().into();
            status_str
        },
        None => existing.payment_status,
    };

    let contribution = sqlx::query_as::<_, Contribution>(
        "UPDATE contributions 
         SET amount = $1, payment_method = $2, payment_status = $3
         WHERE id = $4 
         RETURNING id, tontine_round_id, member_id, amount, payment_date, payment_method, payment_status, created_at"
    )
    .bind(amount)
    .bind(payment_method)
    .bind(payment_status)
    .bind(contribution_id)
    .fetch_one(pool)
    .await?;

    Ok(contribution)
}


    pub async fn delete(pool: &PgPool, contribution_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM contributions WHERE id = $1")
            .bind(contribution_id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Contribution avec l'ID {} non trouvée", contribution_id)));
        }

        Ok(())
    }

    pub async fn mark_as_paid(pool: &PgPool, contribution_id: Uuid) -> Result<Contribution, AppError> {
        let contribution = sqlx::query_as::<_, Contribution>(
            "UPDATE contributions 
             SET payment_status = 'paid', payment_date = $1 
             WHERE id = $2 
             RETURNING id, tontine_round_id, member_id, amount, payment_date, payment_method, payment_status, created_at"
        )
        .bind(Utc::now())
        .bind(contribution_id)
        .fetch_one(pool)
        .await?;

        Ok(contribution)
    }

    pub async fn mark_as_failed(pool: &PgPool, contribution_id: Uuid) -> Result<Contribution, AppError> {
        let contribution = sqlx::query_as::<_, Contribution>(
            "UPDATE contributions 
             SET payment_status = 'failed'
             WHERE id = $2 
             RETURNING id, tontine_round_id, member_id, amount, payment_date, payment_method, payment_status, created_at"
        )
        .bind(contribution_id)
        .fetch_one(pool)
        .await?;

        Ok(contribution)
    }

    pub async fn get_round_summary(pool: &PgPool, round_id: Uuid) -> Result<RoundSummary, AppError> {
        let summary = sqlx::query(
            "SELECT 
                COUNT(*) as total_members,
                COUNT(c.id) as contributions_count,
                COALESCE(SUM(c.amount), 0) as total_collected,
                tr.amount as round_amount
             FROM tontine_rounds tr
             JOIN tontines t ON tr.tontine_id = t.id
             JOIN tontine_members tm ON t.id = tm.tontine_id AND tm.is_active = true
             LEFT JOIN contributions c ON tr.id = c.tontine_round_id AND c.member_id = tm.id AND c.payment_status = 'paid'
             WHERE tr.id = $1
             GROUP BY tr.id, tr.amount"
        )
        .bind(round_id)
        .fetch_one(pool)
        .await?;

        let round_summary = RoundSummary {
            total_members: summary.get("total_members"),
            contributions_count: summary.get("contributions_count"),
            total_collected: summary.get("total_collected"),
            round_amount: summary.get("round_amount"),
            remaining_amount: summary.get::<rust_decimal::Decimal, _>("round_amount") - summary.get::<rust_decimal::Decimal, _>("total_collected"),
        };

        Ok(round_summary)
    }

    pub async fn get_member_contributions_summary(pool: &PgPool, member_id: Uuid) -> Result<MemberContributionsSummary, AppError> {
        let summary = sqlx::query(
            "SELECT 
                COUNT(*) as total_contributions,
                COALESCE(SUM(amount), 0) as total_amount,
                COUNT(CASE WHEN payment_status = 'paid' THEN 1 END) as paid_contributions,
                COUNT(CASE WHEN payment_status = 'pending' THEN 1 END) as pending_contributions
             FROM contributions 
             WHERE member_id = $1"
        )
        .bind(member_id)
        .fetch_one(pool)
        .await?;

        let member_summary = MemberContributionsSummary {
            total_contributions: summary.get("total_contributions"),
            total_amount: summary.get("total_amount"),
            paid_contributions: summary.get("paid_contributions"),
            pending_contributions: summary.get("pending_contributions"),
        };

        Ok(member_summary)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct RoundSummary {
    pub total_members: i64,
    pub contributions_count: i64,
    pub total_collected: rust_decimal::Decimal,
    pub round_amount: rust_decimal::Decimal,
    pub remaining_amount: rust_decimal::Decimal,
}

#[derive(Debug, serde::Serialize)]
pub struct MemberContributionsSummary {
    pub total_contributions: i64,
    pub total_amount: rust_decimal::Decimal,
    pub paid_contributions: i64,
    pub pending_contributions: i64,
}