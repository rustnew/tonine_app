use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, Eq)]
pub struct Tontine {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub amount_per_member: Decimal,
    pub frequency: String,
    pub max_members: i32,
    pub current_round: i32,
    pub status: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTontine {
    pub name: String,
    pub description: Option<String>,
    pub amount_per_member: Decimal,
    pub frequency: TontineFrequency,
    pub max_members: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTontine {
    pub name: Option<String>,
    pub description: Option<String>,
    pub amount_per_member: Option<Decimal>,
    pub frequency: Option<TontineFrequency>,
    pub max_members: Option<i32>,
    pub status: Option<TontineStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TontineFrequency {
    Daily,
    Weekly,
    Monthly,
}

impl From<TontineFrequency> for String {
    fn from(freq: TontineFrequency) -> String {
        match freq {
            TontineFrequency::Daily => "daily".to_string(),
            TontineFrequency::Weekly => "weekly".to_string(),
            TontineFrequency::Monthly => "monthly".to_string(),
        }
    }
}

impl TryFrom<String> for TontineFrequency {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "daily" => Ok(TontineFrequency::Daily),
            "weekly" => Ok(TontineFrequency::Weekly),
            "monthly" => Ok(TontineFrequency::Monthly),
            _ => Err(format!("Fréquence invalide: {}", value)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TontineStatus {
    Active,
    Completed,
    Cancelled,
}

impl From<TontineStatus> for String {
    fn from(status: TontineStatus) -> String {
        match status {
            TontineStatus::Active => "active".to_string(),
            TontineStatus::Completed => "completed".to_string(),
            TontineStatus::Cancelled => "cancelled".to_string(),
        }
    }
}

impl TryFrom<String> for TontineStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "active" => Ok(TontineStatus::Active),
            "completed" => Ok(TontineStatus::Completed),
            "cancelled" => Ok(TontineStatus::Cancelled),
            _ => Err(format!("Statut invalide: {}", value)),
        }
    }
}