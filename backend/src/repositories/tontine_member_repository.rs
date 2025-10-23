use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::model::tontine_members::{TontineMember, CreateTontineMember, UpdateTontineMember, TontineMemberWithUser};
use crate::errors::AppError;

pub struct TontineMemberRepository;

impl TontineMemberRepository {

    pub async fn find_all(pool: &PgPool) -> Result<Vec<TontineMemberWithUser>, AppError> {
        let members = sqlx::query(
            "SELECT tm.*, u.email as user_email, u.phone as user_phone, u.full_name as user_full_name
             FROM tontine_members tm
             JOIN users u ON tm.user_id = u.id
             ORDER BY tm.join_date DESC"
        )
        .fetch_all(pool)
        .await?;

        let members_with_user = members.into_iter().map(|row| TontineMemberWithUser {
            id: row.get("id"),
            tontine_id: row.get("tontine_id"),
            user_id: row.get("user_id"),
            join_date: row.get("join_date"),
            is_active: row.get("is_active"),
            position_order: row.get("position_order"),
            user_email: row.get("user_email"),
            user_phone: row.get("user_phone"),
            user_full_name: row.get("user_full_name"),
        }).collect();

        Ok(members_with_user)
    }

    pub async fn find_by_id(pool: &PgPool, member_id: Uuid) -> Result<TontineMemberWithUser, AppError> {
        let member = sqlx::query(
            "SELECT tm.*, u.email as user_email, u.phone as user_phone, u.full_name as user_full_name
             FROM tontine_members tm
             JOIN users u ON tm.user_id = u.id
             WHERE tm.id = $1"
        )
        .bind(member_id)
        .fetch_optional(pool)
        .await?;

        match member {
            Some(row) => {
                let member_with_user = TontineMemberWithUser {
                    id: row.get("id"),
                    tontine_id: row.get("tontine_id"),
                    user_id: row.get("user_id"),
                    join_date: row.get("join_date"),
                    is_active: row.get("is_active"),
                    position_order: row.get("position_order"),
                    user_email: row.get("user_email"),
                    user_phone: row.get("user_phone"),
                    user_full_name: row.get("user_full_name"),
                };
                Ok(member_with_user)
            },
            None => Err(AppError::NotFound(format!("Membre avec l'ID {} non trouvé", member_id))),
        }
    }

    pub async fn find_by_tontine(pool: &PgPool, tontine_id: Uuid) -> Result<Vec<TontineMemberWithUser>, AppError> {
        let members = sqlx::query(
            "SELECT tm.*, u.email as user_email, u.phone as user_phone, u.full_name as user_full_name
             FROM tontine_members tm
             JOIN users u ON tm.user_id = u.id
             WHERE tm.tontine_id = $1
             ORDER BY tm.position_order ASC, tm.join_date ASC"
        )
        .bind(tontine_id)
        .fetch_all(pool)
        .await?;

        let members_with_user = members.into_iter().map(|row| TontineMemberWithUser {
            id: row.get("id"),
            tontine_id: row.get("tontine_id"),
            user_id: row.get("user_id"),
            join_date: row.get("join_date"),
            is_active: row.get("is_active"),
            position_order: row.get("position_order"),
            user_email: row.get("user_email"),
            user_phone: row.get("user_phone"),
            user_full_name: row.get("user_full_name"),
        }).collect();

        Ok(members_with_user)
    }

    pub async fn find_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<TontineMemberWithUser>, AppError> {
        let members = sqlx::query(
            "SELECT tm.*, u.email as user_email, u.phone as user_phone, u.full_name as user_full_name
             FROM tontine_members tm
             JOIN users u ON tm.user_id = u.id
             WHERE tm.user_id = $1 AND tm.is_active = true
             ORDER BY tm.join_date DESC"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        let members_with_user = members.into_iter().map(|row| TontineMemberWithUser {
            id: row.get("id"),
            tontine_id: row.get("tontine_id"),
            user_id: row.get("user_id"),
            join_date: row.get("join_date"),
            is_active: row.get("is_active"),
            position_order: row.get("position_order"),
            user_email: row.get("user_email"),
            user_phone: row.get("user_phone"),
            user_full_name: row.get("user_full_name"),
        }).collect();

        Ok(members_with_user)
    }


    
    pub async fn create(pool: &PgPool, member_data: &CreateTontineMember) -> Result<TontineMember, AppError> {
        // Vérifier si la tontine existe
        let tontine_exists = sqlx::query("SELECT id FROM tontines WHERE id = $1")
            .bind(&member_data.tontine_id)
            .fetch_optional(pool)
            .await?;

        if tontine_exists.is_none() {
            return Err(AppError::ValidationError("La tontine spécifiée n'existe pas".to_string()));
        }

        // Vérifier si l'utilisateur existe
        let user_exists = sqlx::query("SELECT id FROM users WHERE id = $1")
            .bind(&member_data.user_id)
            .fetch_optional(pool)
            .await?;

        if user_exists.is_none() {
            return Err(AppError::ValidationError("L'utilisateur spécifié n'existe pas".to_string()));
        }

        // Vérifier si l'utilisateur est déjà membre
        let existing_member = sqlx::query("SELECT id FROM tontine_members WHERE tontine_id = $1 AND user_id = $2")
            .bind(&member_data.tontine_id)
            .bind(&member_data.user_id)
            .fetch_optional(pool)
            .await?;

        if existing_member.is_some() {
            return Err(AppError::ValidationError("L'utilisateur est déjà membre de cette tontine".to_string()));
        }

        // Vérifier le nombre maximum de membres
        let member_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tontine_members WHERE tontine_id = $1 AND is_active = true")
            .bind(&member_data.tontine_id)
            .fetch_one(pool)
            .await?;

        let tontine_max_members: i64 = sqlx::query_scalar("SELECT max_members FROM tontines WHERE id = $1")
            .bind(&member_data.tontine_id)
            .fetch_one(pool)
            .await?;

        if member_count >= tontine_max_members {
            return Err(AppError::ValidationError("La tontine a atteint son nombre maximum de membres".to_string()));
        }

        // Déterminer la position order si non fournie
        let position_order = match member_data.position_order {
            Some(order) => order,
            None => {
                let max_position: Option<i32> = sqlx::query_scalar(
                    "SELECT MAX(position_order) FROM tontine_members WHERE tontine_id = $1"
                )
                .bind(&member_data.tontine_id)
                .fetch_one(pool)
                .await?;

                max_position.unwrap_or(0) + 1
            }
        };

        let member = sqlx::query_as::<_, TontineMember>(
            "INSERT INTO tontine_members (tontine_id, user_id, position_order) 
             VALUES ($1, $2, $3) 
             RETURNING id, tontine_id, user_id, join_date, is_active, position_order"
        )
        .bind(&member_data.tontine_id)
        .bind(&member_data.user_id)
        .bind(position_order)
        .fetch_one(pool)
        .await?;

        Ok(member)
    }



    pub async fn update(pool: &PgPool, member_id: Uuid, member_data: &UpdateTontineMember) -> Result<TontineMember, AppError> {
    // Vérifier si le membre existe
    let existing_member = sqlx::query_as::<_, TontineMember>(
        "SELECT id, tontine_id, user_id, join_date, is_active, position_order FROM tontine_members WHERE id = $1"
    )
    .bind(member_id)
    .fetch_optional(pool)
    .await?;

    if existing_member.is_none() {
        return Err(AppError::NotFound(format!("Membre avec l'ID {} non trouvé", member_id)));
    }

    // Construire la requête en fonction des champs fournis
    match (member_data.is_active, member_data.position_order) {
        (Some(is_active), Some(position_order)) => {
            // Les deux champs sont fournis
            let member = sqlx::query_as::<_, TontineMember>(
                "UPDATE tontine_members SET is_active = $1, position_order = $2 
                 WHERE id = $3 
                 RETURNING id, tontine_id, user_id, join_date, is_active, position_order"
            )
            .bind(is_active)
            .bind(position_order)
            .bind(member_id)
            .fetch_one(pool)
            .await?;
            Ok(member)
        }
        (Some(is_active), None) => {
            // Seul is_active est fourni
            let member = sqlx::query_as::<_, TontineMember>(
                "UPDATE tontine_members SET is_active = $1 
                 WHERE id = $2 
                 RETURNING id, tontine_id, user_id, join_date, is_active, position_order"
            )
            .bind(is_active)
            .bind(member_id)
            .fetch_one(pool)
            .await?;
            Ok(member)
        }
        (None, Some(position_order)) => {
            // Seul position_order est fourni
            let member = sqlx::query_as::<_, TontineMember>(
                "UPDATE tontine_members SET position_order = $1 
                 WHERE id = $2 
                 RETURNING id, tontine_id, user_id, join_date, is_active, position_order"
            )
            .bind(position_order)
            .bind(member_id)
            .fetch_one(pool)
            .await?;
            Ok(member)
        }
        (None, None) => {
            // Aucun champ fourni, retourner le membre existant
            Ok(existing_member.unwrap())
        }
    }
}

    pub async fn delete(pool: &PgPool, member_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM tontine_members WHERE id = $1")
            .bind(member_id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Membre avec l'ID {} non trouvé", member_id)));
        }

        Ok(())
    }

    pub async fn deactivate_member(pool: &PgPool, member_id: Uuid) -> Result<TontineMember, AppError> {
        let member = sqlx::query_as::<_, TontineMember>(
            "UPDATE tontine_members SET is_active = false 
             WHERE id = $1 
             RETURNING id, tontine_id, user_id, join_date, is_active, position_order"
        )
        .bind(member_id)
        .fetch_one(pool)
        .await?;

        Ok(member)
    }

    pub async fn get_tontine_member_count(pool: &PgPool, tontine_id: Uuid) -> Result<i64, AppError> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM tontine_members WHERE tontine_id = $1 AND is_active = true"
        )
        .bind(tontine_id)
        .fetch_one(pool)
        .await?;

        Ok(count)
    }
}