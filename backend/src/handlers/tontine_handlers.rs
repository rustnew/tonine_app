use actix_web::{web, HttpResponse};
use uuid::Uuid;


use crate::model::tontine::{CreateTontine, UpdateTontine};
use crate::repositories::tontine_repository::{TontineRepository};
use crate::errors::AppError;

pub struct TontineHandler;

impl TontineHandler {
    pub async fn get_tontines(pool: web::Data<sqlx::PgPool>) -> Result<HttpResponse, AppError> {
        let tontines = TontineRepository::find_all(&pool).await?;
        Ok(HttpResponse::Ok().json(tontines))
    }

    pub async fn get_tontine(
        pool: web::Data<sqlx::PgPool>,
        tontine_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let tontine = TontineRepository::find_by_id(&pool, tontine_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(tontine))
    }

    pub async fn get_tontine_with_creator(
        pool: web::Data<sqlx::PgPool>,
        tontine_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let tontine = TontineRepository::get_tontine_with_creator(&pool, tontine_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(tontine))
    }

    pub async fn create_tontine(
        pool: web::Data<sqlx::PgPool>,
        tontine_data: web::Json<CreateTontine>,
        user_id : String,
    ) -> Result<HttpResponse, AppError> {
        let tontine = TontineRepository::create(&pool, &tontine_data.into_inner(), user_id).await?;
        Ok(HttpResponse::Created().json(tontine))
    }

    pub async fn update_tontine(
        pool: web::Data<sqlx::PgPool>,
        tontine_id: web::Path<Uuid>,
        tontine_data: web::Json<UpdateTontine>,
    ) -> Result<HttpResponse, AppError> {
        let tontine = TontineRepository::update(&pool, tontine_id.into_inner(), &tontine_data.into_inner()).await?;
        Ok(HttpResponse::Ok().json(tontine))
    }

    pub async fn delete_tontine(
        pool: web::Data<sqlx::PgPool>,
        tontine_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        TontineRepository::delete(&pool, tontine_id.into_inner()).await?;
        Ok(HttpResponse::NoContent().finish())
    }

    pub async fn get_user_tontines(
        pool: web::Data<sqlx::PgPool>,
        user_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let tontines = TontineRepository::find_by_creator(&pool, user_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(tontines))
    }

    pub async fn get_active_tontines(
        pool: web::Data<sqlx::PgPool>,
    ) -> Result<HttpResponse, AppError> {
        let tontines = TontineRepository::get_active_tontines(&pool).await?;
        Ok(HttpResponse::Ok().json(tontines))
    }

    pub async fn increment_round(
        pool: web::Data<sqlx::PgPool>,
        tontine_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let tontine = TontineRepository::increment_round(&pool, tontine_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(tontine))
    }
}