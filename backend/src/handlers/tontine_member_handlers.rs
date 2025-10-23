use actix_web::{web, HttpResponse};
use uuid::Uuid;
use serde_json::json;

use crate::model::tontine_members::{CreateTontineMember, UpdateTontineMember};
use crate::repositories::tontine_member_repository::TontineMemberRepository;
use crate::errors::AppError;

pub struct TontineMemberHandler;

impl TontineMemberHandler {
    pub async fn get_members(pool: web::Data<sqlx::PgPool>) -> Result<HttpResponse, AppError> {
        let members = TontineMemberRepository::find_all(&pool).await?;
        Ok(HttpResponse::Ok().json(members))
    }

    pub async fn get_member(
        pool: web::Data<sqlx::PgPool>,
        member_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let member = TontineMemberRepository::find_by_id(&pool, member_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(member))
    }

    pub async fn get_tontine_members(
        pool: web::Data<sqlx::PgPool>,
        tontine_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let members = TontineMemberRepository::find_by_tontine(&pool, tontine_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(members))
    }

    pub async fn get_user_members(
        pool: web::Data<sqlx::PgPool>,
        user_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let members = TontineMemberRepository::find_by_user(&pool, user_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(members))
    }

    pub async fn create_member(
        pool: web::Data<sqlx::PgPool>,
        member_data: web::Json<CreateTontineMember>,
    ) -> Result<HttpResponse, AppError> {
        let member = TontineMemberRepository::create(&pool, &member_data.into_inner()).await?;
        Ok(HttpResponse::Created().json(member))
    }

    pub async fn update_member(
        pool: web::Data<sqlx::PgPool>,
        member_id: web::Path<Uuid>,
        member_data: web::Json<UpdateTontineMember>,
    ) -> Result<HttpResponse, AppError> {
        let member = TontineMemberRepository::update(&pool, member_id.into_inner(), &member_data.into_inner()).await?;
        Ok(HttpResponse::Ok().json(member))
    }

    pub async fn delete_member(
        pool: web::Data<sqlx::PgPool>,
        member_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        TontineMemberRepository::delete(&pool, member_id.into_inner()).await?;
        Ok(HttpResponse::NoContent().finish())
    }

    pub async fn deactivate_member(
        pool: web::Data<sqlx::PgPool>,
        member_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let member = TontineMemberRepository::deactivate_member(&pool, member_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(member))
    }

    pub async fn get_member_count(
        pool: web::Data<sqlx::PgPool>,
        tontine_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, AppError> {
        let count = TontineMemberRepository::get_tontine_member_count(&pool, tontine_id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(json!({ "member_count": count })))
    }
}