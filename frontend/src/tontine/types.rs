// src/tontine/types.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Tontine {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub amount_per_member: String,
    pub frequency: String,
    pub max_members: i32,
    pub current_round: i32,
    pub status: String,
    pub created_by: Uuid,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateTontine {
    pub name: String,
    pub description: Option<String>,
    pub amount_per_member: String,
    pub frequency: String,
    pub max_members: i32,
    pub created_by: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateTontine {
    pub name: Option<String>,
    pub description: Option<String>,
    pub amount_per_member: Option<String>,
    pub frequency: Option<String>,
    pub max_members: Option<i32>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TontineWithCreator {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub amount_per_member: String,
    pub frequency: String,
    pub max_members: i32,
    pub current_round: i32,
    pub status: String,
    pub created_by: Uuid,
    pub created_at: String,
    pub updated_at: String,
    pub creator_name: String,
    pub creator_email: String,
}