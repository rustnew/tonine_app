use sqlx::{PgPool, Row};
use uuid::Uuid;


use crate::model::transactions::{Transaction, CreateTransaction, TransactionWithUsers, TransactionType, TransactionStatus};
use crate::errors::AppError;

pub struct TransactionRepository;

impl TransactionRepository {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<TransactionWithUsers>, AppError> {
        let transactions = sqlx::query(
            "SELECT t.*, 
                    u_from.full_name as from_user_name,
                    u_to.full_name as to_user_name,
                    ton.name as tontine_name
             FROM transactions t
             LEFT JOIN users u_from ON t.from_user_id = u_from.id
             LEFT JOIN users u_to ON t.to_user_id = u_to.id
             JOIN tontines ton ON t.tontine_id = ton.id
             ORDER BY t.created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        let transactions_with_users = transactions.into_iter().map(|row| TransactionWithUsers {
            id: row.get("id"),
            tontine_id: row.get("tontine_id"),
            from_user_id: row.get("from_user_id"),
            to_user_id: row.get("to_user_id"),
            amount: row.get("amount"),
            transaction_type: row.get("transaction_type"),
            status: row.get("status"),
            description: row.get("description"),
            created_at: row.get("created_at"),
            from_user_name: row.get("from_user_name"),
            to_user_name: row.get("to_user_name"),
            tontine_name: row.get("tontine_name"),
        }).collect();

        Ok(transactions_with_users)
    }

    pub async fn find_by_id(pool: &PgPool, transaction_id: Uuid) -> Result<TransactionWithUsers, AppError> {
        let transaction = sqlx::query(
            "SELECT t.*, 
                    u_from.full_name as from_user_name,
                    u_to.full_name as to_user_name,
                    ton.name as tontine_name
             FROM transactions t
             LEFT JOIN users u_from ON t.from_user_id = u_from.id
             LEFT JOIN users u_to ON t.to_user_id = u_to.id
             JOIN tontines ton ON t.tontine_id = ton.id
             WHERE t.id = $1"
        )
        .bind(transaction_id)
        .fetch_optional(pool)
        .await?;

        match transaction {
            Some(row) => {
                let transaction_with_users = TransactionWithUsers {
                    id: row.get("id"),
                    tontine_id: row.get("tontine_id"),
                    from_user_id: row.get("from_user_id"),
                    to_user_id: row.get("to_user_id"),
                    amount: row.get("amount"),
                    transaction_type: row.get("transaction_type"),
                    status: row.get("status"),
                    description: row.get("description"),
                    created_at: row.get("created_at"),
                    from_user_name: row.get("from_user_name"),
                    to_user_name: row.get("to_user_name"),
                    tontine_name: row.get("tontine_name"),
                };
                Ok(transaction_with_users)
            },
            None => Err(AppError::NotFound(format!("Transaction avec l'ID {} non trouvée", transaction_id))),
        }
    }

    pub async fn find_by_tontine(pool: &PgPool, tontine_id: Uuid) -> Result<Vec<TransactionWithUsers>, AppError> {
        let transactions = sqlx::query(
            "SELECT t.*, 
                    u_from.full_name as from_user_name,
                    u_to.full_name as to_user_name,
                    ton.name as tontine_name
             FROM transactions t
             LEFT JOIN users u_from ON t.from_user_id = u_from.id
             LEFT JOIN users u_to ON t.to_user_id = u_to.id
             JOIN tontines ton ON t.tontine_id = ton.id
             WHERE t.tontine_id = $1
             ORDER BY t.created_at DESC"
        )
        .bind(tontine_id)
        .fetch_all(pool)
        .await?;

        let transactions_with_users = transactions.into_iter().map(|row| TransactionWithUsers {
            id: row.get("id"),
            tontine_id: row.get("tontine_id"),
            from_user_id: row.get("from_user_id"),
            to_user_id: row.get("to_user_id"),
            amount: row.get("amount"),
            transaction_type: row.get("transaction_type"),
            status: row.get("status"),
            description: row.get("description"),
            created_at: row.get("created_at"),
            from_user_name: row.get("from_user_name"),
            to_user_name: row.get("to_user_name"),
            tontine_name: row.get("tontine_name"),
        }).collect();

        Ok(transactions_with_users)
    }

    pub async fn find_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<TransactionWithUsers>, AppError> {
        let transactions = sqlx::query(
            "SELECT t.*, 
                    u_from.full_name as from_user_name,
                    u_to.full_name as to_user_name,
                    ton.name as tontine_name
             FROM transactions t
             LEFT JOIN users u_from ON t.from_user_id = u_from.id
             LEFT JOIN users u_to ON t.to_user_id = u_to.id
             JOIN tontines ton ON t.tontine_id = ton.id
             WHERE t.from_user_id = $1 OR t.to_user_id = $1
             ORDER BY t.created_at DESC"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        let transactions_with_users = transactions.into_iter().map(|row| TransactionWithUsers {
            id: row.get("id"),
            tontine_id: row.get("tontine_id"),
            from_user_id: row.get("from_user_id"),
            to_user_id: row.get("to_user_id"),
            amount: row.get("amount"),
            transaction_type: row.get("transaction_type"),
            status: row.get("status"),
            description: row.get("description"),
            created_at: row.get("created_at"),
            from_user_name: row.get("from_user_name"),
            to_user_name: row.get("to_user_name"),
            tontine_name: row.get("tontine_name"),
        }).collect();

        Ok(transactions_with_users)
    }

    pub async fn create(pool: &PgPool, transaction_data: &CreateTransaction) -> Result<Transaction, AppError> {
        // Vérifier si la tontine existe
        let tontine_exists = sqlx::query("SELECT id FROM tontines WHERE id = $1")
            .bind(&transaction_data.tontine_id)
            .fetch_optional(pool)
            .await?;

        if tontine_exists.is_none() {
            return Err(AppError::ValidationError("La tontine spécifiée n'existe pas".to_string()));
        }

        // Vérifier les utilisateurs si spécifiés
        if let Some(from_user_id) = &transaction_data.from_user_id {
            let from_user_exists = sqlx::query("SELECT id FROM users WHERE id = $1")
                .bind(from_user_id)
                .fetch_optional(pool)
                .await?;

            if from_user_exists.is_none() {
                return Err(AppError::ValidationError("L'utilisateur source spécifié n'existe pas".to_string()));
            }
        }

        if let Some(to_user_id) = &transaction_data.to_user_id {
            let to_user_exists = sqlx::query("SELECT id FROM users WHERE id = $1")
                .bind(to_user_id)
                .fetch_optional(pool)
                .await?;

            if to_user_exists.is_none() {
                return Err(AppError::ValidationError("L'utilisateur destinataire spécifié n'existe pas".to_string()));
            }
        }

        let transaction_type_str: String = transaction_data.transaction_type.clone().into();

        let transaction = sqlx::query_as::<_, Transaction>(
            "INSERT INTO transactions (tontine_id, from_user_id, to_user_id, amount, transaction_type, description) 
             VALUES ($1, $2, $3, $4, $5, $6) 
             RETURNING id, tontine_id, from_user_id, to_user_id, amount, transaction_type, status, description, created_at"
        )
        .bind(&transaction_data.tontine_id)
        .bind(&transaction_data.from_user_id)
        .bind(&transaction_data.to_user_id)
        .bind(&transaction_data.amount)
        .bind(&transaction_type_str)
        .bind(&transaction_data.description)
        .fetch_one(pool)
        .await?;

        Ok(transaction)
    }

    pub async fn create_contribution_transaction(
        pool: &PgPool,
        tontine_id: Uuid,
        from_user_id: Uuid,
        amount: rust_decimal::Decimal,
        description: Option<String>,
    ) -> Result<Transaction, AppError> {
        let transaction_data = CreateTransaction {
            tontine_id,
            from_user_id: Some(from_user_id),
            to_user_id: None, // La tontine reçoit l'argent
            amount,
            transaction_type: TransactionType::Contribution,
            description: description.or_else(|| Some("Cotisation tontine".to_string())),
        };

        Self::create(pool, &transaction_data).await
    }

    pub async fn create_payout_transaction(
        pool: &PgPool,
        tontine_id: Uuid,
        to_user_id: Uuid,
        amount: rust_decimal::Decimal,
        description: Option<String>,
    ) -> Result<Transaction, AppError> {
        let transaction_data = CreateTransaction {
            tontine_id,
            from_user_id: None, // La tontine paie
            to_user_id: Some(to_user_id),
            amount,
            transaction_type: TransactionType::Payout,
            description: description.or_else(|| Some("Paiement bénéficiaire tontine".to_string())),
        };

        Self::create(pool, &transaction_data).await
    }

    pub async fn create_refund_transaction(
        pool: &PgPool,
        tontine_id: Uuid,
        from_user_id: Uuid,
        to_user_id: Uuid,
        amount: rust_decimal::Decimal,
        description: Option<String>,
    ) -> Result<Transaction, AppError> {
        let transaction_data = CreateTransaction {
            tontine_id,
            from_user_id: Some(from_user_id),
            to_user_id: Some(to_user_id),
            amount,
            transaction_type: TransactionType::Refund,
            description: description.or_else(|| Some("Remboursement tontine".to_string())),
        };

        Self::create(pool, &transaction_data).await
    }

    pub async fn update_status(pool: &PgPool, transaction_id: Uuid, status: TransactionStatus) -> Result<Transaction, AppError> {
        let status_str: String = status.into();

        let transaction = sqlx::query_as::<_, Transaction>(
            "UPDATE transactions 
             SET status = $1 
             WHERE id = $2 
             RETURNING id, tontine_id, from_user_id, to_user_id, amount, transaction_type, status, description, created_at"
        )
        .bind(&status_str)
        .bind(transaction_id)
        .fetch_one(pool)
        .await?;

        Ok(transaction)
    }

    pub async fn get_transactions_by_type(pool: &PgPool, transaction_type: TransactionType) -> Result<Vec<TransactionWithUsers>, AppError> {
        let type_str: String = transaction_type.into();

        let transactions = sqlx::query(
            "SELECT t.*, 
                    u_from.full_name as from_user_name,
                    u_to.full_name as to_user_name,
                    ton.name as tontine_name
             FROM transactions t
             LEFT JOIN users u_from ON t.from_user_id = u_from.id
             LEFT JOIN users u_to ON t.to_user_id = u_to.id
             JOIN tontines ton ON t.tontine_id = ton.id
             WHERE t.transaction_type = $1
             ORDER BY t.created_at DESC"
        )
        .bind(&type_str)
        .fetch_all(pool)
        .await?;

        let transactions_with_users = transactions.into_iter().map(|row| TransactionWithUsers {
            id: row.get("id"),
            tontine_id: row.get("tontine_id"),
            from_user_id: row.get("from_user_id"),
            to_user_id: row.get("to_user_id"),
            amount: row.get("amount"),
            transaction_type: row.get("transaction_type"),
            status: row.get("status"),
            description: row.get("description"),
            created_at: row.get("created_at"),
            from_user_name: row.get("from_user_name"),
            to_user_name: row.get("to_user_name"),
            tontine_name: row.get("tontine_name"),
        }).collect();

        Ok(transactions_with_users)
    }

    pub async fn get_tontine_financial_summary(pool: &PgPool, tontine_id: Uuid) -> Result<TontineFinancialSummary, AppError> {
        let summary = sqlx::query(
            "SELECT 
                COUNT(*) as total_transactions,
                COALESCE(SUM(CASE WHEN transaction_type = 'contribution' THEN amount ELSE 0 END), 0) as total_contributions,
                COALESCE(SUM(CASE WHEN transaction_type = 'payout' THEN amount ELSE 0 END), 0) as total_payouts,
                COALESCE(SUM(CASE WHEN transaction_type = 'refund' THEN amount ELSE 0 END), 0) as total_refunds,
                (COALESCE(SUM(CASE WHEN transaction_type = 'contribution' THEN amount ELSE 0 END), 0) - 
                 COALESCE(SUM(CASE WHEN transaction_type IN ('payout', 'refund') THEN amount ELSE 0 END), 0)) as current_balance
             FROM transactions 
             WHERE tontine_id = $1 AND status = 'completed'"
        )
        .bind(tontine_id)
        .fetch_one(pool)
        .await?;

        let financial_summary = TontineFinancialSummary {
            total_transactions: summary.get("total_transactions"),
            total_contributions: summary.get("total_contributions"),
            total_payouts: summary.get("total_payouts"),
            total_refunds: summary.get("total_refunds"),
            current_balance: summary.get("current_balance"),
        };

        Ok(financial_summary)
    }

    pub async fn get_user_financial_summary(pool: &PgPool, user_id: Uuid) -> Result<UserFinancialSummary, AppError> {
        let summary = sqlx::query(
            "SELECT 
                COUNT(*) as total_transactions,
                COALESCE(SUM(CASE WHEN from_user_id = $1 THEN amount ELSE 0 END), 0) as total_sent,
                COALESCE(SUM(CASE WHEN to_user_id = $1 THEN amount ELSE 0 END), 0) as total_received,
                (COALESCE(SUM(CASE WHEN to_user_id = $1 THEN amount ELSE 0 END), 0) - 
                 COALESCE(SUM(CASE WHEN from_user_id = $1 THEN amount ELSE 0 END), 0)) as net_balance
             FROM transactions 
             WHERE (from_user_id = $1 OR to_user_id = $1) AND status = 'completed'"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        let user_summary = UserFinancialSummary {
            total_transactions: summary.get("total_transactions"),
            total_sent: summary.get("total_sent"),
            total_received: summary.get("total_received"),
            net_balance: summary.get("net_balance"),
        };

        Ok(user_summary)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct TontineFinancialSummary {
    pub total_transactions: i64,
    pub total_contributions: rust_decimal::Decimal,
    pub total_payouts: rust_decimal::Decimal,
    pub total_refunds: rust_decimal::Decimal,
    pub current_balance: rust_decimal::Decimal,
}

#[derive(Debug, serde::Serialize)]
pub struct UserFinancialSummary {
    pub total_transactions: i64,
    pub total_sent: rust_decimal::Decimal,
    pub total_received: rust_decimal::Decimal,
    pub net_balance: rust_decimal::Decimal,
}