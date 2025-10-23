use actix_web::web;
use crate::handlers::transaction_handlers::TransactionHandler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/transactions")
            .route("", web::get().to(TransactionHandler::get_transactions))
            .route("", web::post().to(TransactionHandler::create_transaction))
            .route("/contribution", web::post().to(TransactionHandler::create_contribution_transaction))
            .route("/payout", web::post().to(TransactionHandler::create_payout_transaction))
            .route("/type/{transaction_type}", web::get().to(TransactionHandler::get_transactions_by_type))
            .route("/tontine/{tontine_id}", web::get().to(TransactionHandler::get_tontine_transactions))
            .route("/tontine/{tontine_id}/summary", web::get().to(TransactionHandler::get_tontine_financial_summary))
            .route("/user/{user_id}", web::get().to(TransactionHandler::get_user_transactions))
            .route("/user/{user_id}/summary", web::get().to(TransactionHandler::get_user_financial_summary))
            .route("/{id}", web::get().to(TransactionHandler::get_transaction))
            .route("/{id}/status/{status}", web::put().to(TransactionHandler::update_transaction_status))
    );
}