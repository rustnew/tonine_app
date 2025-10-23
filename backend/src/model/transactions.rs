use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub tontine_id: Uuid,
    pub from_user_id: Option<Uuid>,
    pub to_user_id: Option<Uuid>,
    pub amount: Decimal,
    pub transaction_type: String,
    pub status: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransaction {
    pub tontine_id: Uuid,
    pub from_user_id: Option<Uuid>,
    pub to_user_id: Option<Uuid>,
    pub amount: Decimal,
    pub transaction_type: TransactionType,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Contribution,
    Payout,
    Refund,
}

impl From<TransactionType> for String {
    fn from(t_type: TransactionType) -> String {
        match t_type {
            TransactionType::Contribution => "contribution".to_string(),
            TransactionType::Payout => "payout".to_string(),
            TransactionType::Refund => "refund".to_string(),
        }
    }
}

impl TryFrom<String> for TransactionType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "contribution" => Ok(TransactionType::Contribution),
            "payout" => Ok(TransactionType::Payout),
            "refund" => Ok(TransactionType::Refund),
            _ => Err(format!("Type de transaction invalide: {}", value)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Completed,
    Pending,
    Failed,
}

impl From<TransactionStatus> for String {
    fn from(status: TransactionStatus) -> String {
        match status {
            TransactionStatus::Completed => "completed".to_string(),
            TransactionStatus::Pending => "pending".to_string(),
            TransactionStatus::Failed => "failed".to_string(),
        }
    }
}

impl TryFrom<String> for TransactionStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "completed" => Ok(TransactionStatus::Completed),
            "pending" => Ok(TransactionStatus::Pending),
            "failed" => Ok(TransactionStatus::Failed),
            _ => Err(format!("Statut de transaction invalide: {}", value)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TransactionWithUsers {
    pub id: Uuid,
    pub tontine_id: Uuid,
    pub from_user_id: Option<Uuid>,
    pub to_user_id: Option<Uuid>,
    pub amount: Decimal,
    pub transaction_type: String,
    pub status: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub from_user_name: Option<String>,
    pub to_user_name: Option<String>,
    pub tontine_name: String,
}