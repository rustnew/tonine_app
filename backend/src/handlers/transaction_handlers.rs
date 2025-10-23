use actix_web::{web, HttpResponse};
use uuid::Uuid;


use crate::model::transactions::{CreateTransaction, TransactionType, TransactionStatus};
use crate::repositories::transaction_repository::{TransactionRepository, TontineFinancialSummary, UserFinancialSummary};
use crate::errors::AppError;

pub struct TransactionHandler;

impl TransactionHandler {
    pub async fn get_transactions(pool: web::Data<sqlx::PgPool>) -> Result<HttpResponse, AppError> {
        let transactions = TransactionRepository::find_all(&pool).await?;
        Ok(HttpResponse::Ok().json(transactions))
    }

    pub async fn get_transaction(
        pool: web::Data<sqlx::PgPool>,
        transaction_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let transaction = TransactionRepository::find_by_id(&pool, transaction_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(transaction))
    }

    pub async fn get_tontine_transactions(
        pool: web::Data<sqlx::PgPool>,
        tontine_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let transactions = TransactionRepository::find_by_tontine(&pool, tontine_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(transactions))
    }

    pub async fn get_user_transactions(
        pool: web::Data<sqlx::PgPool>,
        user_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let transactions = TransactionRepository::find_by_user(&pool, user_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(transactions))
    }

    pub async fn create_transaction(
        pool: web::Data<sqlx::PgPool>,
        transaction_data: web::Json<CreateTransaction>,
    ) -> Result<HttpResponse, AppError> {
        let transaction = TransactionRepository::create(&pool, &transaction_data.into_inner()).await?;
        Ok(HttpResponse::Created().json(transaction))
    }

    pub async fn create_contribution_transaction(
        pool: web::Data<sqlx::PgPool>,
        transaction_data: web::Json<CreateContributionTransaction>,
    ) -> Result<HttpResponse, AppError> {
        let data = transaction_data.into_inner();
        let transaction = TransactionRepository::create_contribution_transaction(
            &pool, 
            data.tontine_id, 
            data.from_user_id, 
            data.amount, 
            data.description
        ).await?;
        Ok(HttpResponse::Created().json(transaction))
    }

    pub async fn create_payout_transaction(
        pool: web::Data<sqlx::PgPool>,
        transaction_data: web::Json<CreatePayoutTransaction>,
    ) -> Result<HttpResponse, AppError> {
        let data = transaction_data.into_inner();
        let transaction = TransactionRepository::create_payout_transaction(
            &pool, 
            data.tontine_id, 
            data.to_user_id, 
            data.amount, 
            data.description
        ).await?;
        Ok(HttpResponse::Created().json(transaction))
    }

    pub async fn update_transaction_status(
        pool: web::Data<sqlx::PgPool>,
        path: web::Path<(Uuid, String)>,
    ) -> Result<HttpResponse, AppError> {
        let (transaction_id, status_str) = path.into_inner();
        
        let status = TransactionStatus::try_from(status_str)
            .map_err(|e| AppError::ValidationError(e))?;

        let transaction = TransactionRepository::update_status(&pool, transaction_id, status).await?;
        Ok(HttpResponse::Ok().json(transaction))
    }

    pub async fn get_transactions_by_type(
        pool: web::Data<sqlx::PgPool>,
        transaction_type: web::Path<String>,
    ) -> Result<HttpResponse, AppError> {
        let t_type = TransactionType::try_from(transaction_type.into_inner())
            .map_err(|e| AppError::ValidationError(e))?;

        let transactions = TransactionRepository::get_transactions_by_type(&pool, t_type).await?;
        Ok(HttpResponse::Ok().json(transactions))
    }

    pub async fn get_tontine_financial_summary(
        pool: web::Data<sqlx::PgPool>,
        tontine_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let summary = TransactionRepository::get_tontine_financial_summary(&pool, tontine_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(summary))
    }

    pub async fn get_user_financial_summary(
        pool: web::Data<sqlx::PgPool>,
        user_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let summary = TransactionRepository::get_user_financial_summary(&pool, user_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(summary))
    }
}

#[derive(serde::Deserialize)]
pub struct CreateContributionTransaction {
    pub tontine_id: Uuid,
    pub from_user_id: Uuid,
    pub amount: rust_decimal::Decimal,
    pub description: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct CreatePayoutTransaction {
    pub tontine_id: Uuid,
    pub to_user_id: Uuid,
    pub amount: rust_decimal::Decimal,
    pub description: Option<String>,
}