use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TontineRound {
    pub id: Uuid,
    pub tontine_id: Uuid,
    pub round_number: i32,
    pub beneficiary_user_id: Option<Uuid>,
    pub amount: Decimal,
    pub round_date: Option<DateTime<Utc>>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTontineRound {
    pub tontine_id: Uuid,
    pub round_number: i32,
    pub beneficiary_user_id: Uuid,
    pub amount: Decimal,
    pub round_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTontineRound {
    pub beneficiary_user_id: Option<Uuid>,
    pub amount: Option<Decimal>,
    pub round_date: Option<DateTime<Utc>>,
    pub status: Option<RoundStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoundStatus {
    Pending,
    Completed,
    Cancelled,
}

impl From<RoundStatus> for String {
    fn from(status: RoundStatus) -> String {
        match status {
            RoundStatus::Pending => "pending".to_string(),
            RoundStatus::Completed => "completed".to_string(),
            RoundStatus::Cancelled => "cancelled".to_string(),
        }
    }
}

impl TryFrom<String> for RoundStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "pending" => Ok(RoundStatus::Pending),
            "completed" => Ok(RoundStatus::Completed),
            "cancelled" => Ok(RoundStatus::Cancelled),
            _ => Err(format!("Statut de tour invalide: {}", value)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TontineRoundWithBeneficiary {
    pub id: Uuid,
    pub tontine_id: Uuid,
    pub round_number: i32,
    pub beneficiary_user_id: Option<Uuid>,
    pub amount: Decimal,
    pub round_date: Option<DateTime<Utc>>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub beneficiary_name: Option<String>,
    pub beneficiary_email: Option<String>,
}