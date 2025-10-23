
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Contribution {
    pub id: Uuid,
    pub tontine_round_id: Uuid,
    pub member_id: Uuid,
    pub amount: Decimal,
    pub payment_date: DateTime<Utc>,
    pub payment_method: Option<String>,
    pub payment_status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContribution {
    pub tontine_round_id: Uuid,
    pub member_id: Uuid,
    pub amount: Decimal,
    pub payment_method: PaymentMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContribution {
    pub amount: Option<Decimal>,
    pub payment_method: Option<PaymentMethod>,
    pub payment_status: Option<PaymentStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethod {
    Cash,
    MobileMoney,
    BankTransfer,
}

impl From<PaymentMethod> for String {
    fn from(method: PaymentMethod) -> String {
        match method {
            PaymentMethod::Cash => "cash".to_string(),
            PaymentMethod::MobileMoney => "mobile_money".to_string(),
            PaymentMethod::BankTransfer => "bank_transfer".to_string(),
        }
    }
}

impl TryFrom<String> for PaymentMethod {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "cash" => Ok(PaymentMethod::Cash),
            "mobile_money" => Ok(PaymentMethod::MobileMoney),
            "bank_transfer" => Ok(PaymentMethod::BankTransfer),
            _ => Err(format!("Méthode de paiement invalide: {}", value)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentStatus {
    Paid,
    Pending,
    Failed,
}

impl From<PaymentStatus> for String {
    fn from(status: PaymentStatus) -> String {
        match status {
            PaymentStatus::Paid => "paid".to_string(),
            PaymentStatus::Pending => "pending".to_string(),
            PaymentStatus::Failed => "failed".to_string(),
        }
    }
}

impl TryFrom<String> for PaymentStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "paid" => Ok(PaymentStatus::Paid),
            "pending" => Ok(PaymentStatus::Pending),
            "failed" => Ok(PaymentStatus::Failed),
            _ => Err(format!("Statut de paiement invalide: {}", value)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ContributionWithDetails {
    pub id: Uuid,
    pub tontine_round_id: Uuid,
    pub member_id: Uuid,
    pub amount: Decimal,
    pub payment_date: DateTime<Utc>,
    pub payment_method: Option<String>,
    pub payment_status: String,
    pub created_at: DateTime<Utc>,
    pub member_name: String,
    pub round_number: i32,
    pub tontine_name: String,
}