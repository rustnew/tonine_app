use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::model::contributions::{CreateContribution, UpdateContribution};
use crate::repositories::contributions_repository::{ContributionRepository};
use crate::errors::AppError;

pub struct ContributionHandler;

impl ContributionHandler {
    pub async fn get_contributions(pool: web::Data<sqlx::PgPool>) -> Result<HttpResponse, AppError> {
        let contributions = ContributionRepository::find_all(&pool).await?;
        Ok(HttpResponse::Ok().json(contributions))
    }

    pub async fn get_contribution(
        pool: web::Data<sqlx::PgPool>,
        contribution_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let contribution = ContributionRepository::find_by_id(&pool, contribution_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(contribution))
    }

    pub async fn get_round_contributions(
        pool: web::Data<sqlx::PgPool>,
        round_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let contributions = ContributionRepository::find_by_round(&pool, round_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(contributions))
    }

    pub async fn get_member_contributions(
        pool: web::Data<sqlx::PgPool>,
        member_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let contributions = ContributionRepository::find_by_member(&pool, member_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(contributions))
    }

    pub async fn create_contribution(
        pool: web::Data<sqlx::PgPool>,
        contribution_data: web::Json<CreateContribution>,
    ) -> Result<HttpResponse, AppError> {
        let contribution = ContributionRepository::create(&pool, &contribution_data.into_inner()).await?;
        Ok(HttpResponse::Created().json(contribution))
    }

    pub async fn update_contribution(
        pool: web::Data<sqlx::PgPool>,
        contribution_id: web::Path<Uuid>,
        contribution_data: web::Json<UpdateContribution>,
    ) -> Result<HttpResponse, AppError> {
        let contribution = ContributionRepository::update(&pool, contribution_id.into_inner(), &contribution_data.into_inner()).await?;
        Ok(HttpResponse::Ok().json(contribution))
    }

    pub async fn delete_contribution(
        pool: web::Data<sqlx::PgPool>,
        contribution_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        ContributionRepository::delete(&pool, contribution_id.into_inner()).await?;
        Ok(HttpResponse::NoContent().finish())
    }

    pub async fn mark_as_paid(
        pool: web::Data<sqlx::PgPool>,
        contribution_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let contribution = ContributionRepository::mark_as_paid(&pool, contribution_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(contribution))
    }

    pub async fn mark_as_failed(
        pool: web::Data<sqlx::PgPool>,
        contribution_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let contribution = ContributionRepository::mark_as_failed(&pool, contribution_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(contribution))
    }

    pub async fn get_round_summary(
        pool: web::Data<sqlx::PgPool>,
        round_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let summary = ContributionRepository::get_round_summary(&pool, round_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(summary))
    }

    pub async fn get_member_contributions_summary(
        pool: web::Data<sqlx::PgPool>,
        member_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let summary = ContributionRepository::get_member_contributions_summary(&pool, member_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(summary))
    }
}