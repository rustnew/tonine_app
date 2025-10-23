use actix_web::{web, HttpResponse};
use uuid::Uuid;
use serde_json::json;

use crate::model::tontine_rounds::{CreateTontineRound, UpdateTontineRound, RoundStatus};
use crate::repositories::tontine_round_repository::TontineRoundRepository;
use crate::errors::AppError;

pub struct TontineRoundHandler;

impl TontineRoundHandler {
    pub async fn get_rounds(pool: web::Data<sqlx::PgPool>) -> Result<HttpResponse, AppError> {
        let rounds = TontineRoundRepository::find_all(&pool).await?;
        Ok(HttpResponse::Ok().json(rounds))
    }

    pub async fn get_round(
        pool: web::Data<sqlx::PgPool>,
        round_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let round = TontineRoundRepository::find_by_id(&pool, round_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(round))
    }

    pub async fn get_tontine_rounds(
        pool: web::Data<sqlx::PgPool>,
        tontine_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let rounds = TontineRoundRepository::find_by_tontine(&pool, tontine_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(rounds))
    }

    pub async fn get_current_round(
        pool: web::Data<sqlx::PgPool>,
        tontine_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let round = TontineRoundRepository::find_current_round(&pool, tontine_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(round))
    }

    pub async fn create_round(
        pool: web::Data<sqlx::PgPool>,
        round_data: web::Json<CreateTontineRound>,
    ) -> Result<HttpResponse, AppError> {
        let round = TontineRoundRepository::create(&pool, &round_data.into_inner()).await?;
        Ok(HttpResponse::Created().json(round))
    }

    pub async fn update_round(
        pool: web::Data<sqlx::PgPool>,
        round_id: web::Path<Uuid>,
        round_data: web::Json<UpdateTontineRound>,
    ) -> Result<HttpResponse, AppError> {
        let round = TontineRoundRepository::update(&pool, round_id.into_inner(), &round_data.into_inner()).await?;
        Ok(HttpResponse::Ok().json(round))
    }

    pub async fn delete_round(
        pool: web::Data<sqlx::PgPool>,
        round_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        TontineRoundRepository::delete(&pool, round_id.into_inner()).await?;
        Ok(HttpResponse::NoContent().finish())
    }

    pub async fn complete_round(
        pool: web::Data<sqlx::PgPool>,
        round_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let round = TontineRoundRepository::complete_round(&pool, round_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(round))
    }

    pub async fn cancel_round(
        pool: web::Data<sqlx::PgPool>,
        round_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let round = TontineRoundRepository::cancel_round(&pool, round_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(round))
    }

    pub async fn get_rounds_by_status(
        pool: web::Data<sqlx::PgPool>,
        status: web::Path<String>,
    ) -> Result<HttpResponse, AppError> {
        let round_status = RoundStatus::try_from(status.into_inner())
            .map_err(|e| AppError::ValidationError(e))?;
            
        let rounds = TontineRoundRepository::get_rounds_by_status(&pool, round_status).await?;
        Ok(HttpResponse::Ok().json(rounds))
    }

    pub async fn get_next_round_number(
        pool: web::Data<sqlx::PgPool>,
        tontine_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let next_round = TontineRoundRepository::get_next_round_number(&pool, tontine_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(json!({ "next_round_number": next_round })))
    }
}