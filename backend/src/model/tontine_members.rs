use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TontineMember {
    pub id: Uuid,
    pub tontine_id: Uuid,
    pub user_id: Uuid,
    pub join_date: DateTime<Utc>,
    pub is_active: bool,
    pub position_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTontineMember {
    pub tontine_id: Uuid,
    pub user_id: Uuid,
    pub position_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTontineMember {
    pub is_active: Option<bool>,
    pub position_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TontineMemberWithUser {
    pub id: Uuid,
    pub tontine_id: Uuid,
    pub user_id: Uuid,
    pub join_date: DateTime<Utc>,
    pub is_active: bool,
    pub position_order: Option<i32>,
    pub user_email: String,
    pub user_phone: String,
    pub user_full_name: String,
}