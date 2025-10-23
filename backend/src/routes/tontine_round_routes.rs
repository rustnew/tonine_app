use actix_web::web;
use crate::handlers::tontine_round_handlers::TontineRoundHandler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/tontine-rounds")
            .route("", web::get().to(TontineRoundHandler::get_rounds))
            .route("", web::post().to(TontineRoundHandler::create_round))
            .route("/status/{status}", web::get().to(TontineRoundHandler::get_rounds_by_status))
            .route("/tontine/{tontine_id}", web::get().to(TontineRoundHandler::get_tontine_rounds))
            .route("/tontine/{tontine_id}/current", web::get().to(TontineRoundHandler::get_current_round))
            .route("/tontine/{tontine_id}/next-round", web::get().to(TontineRoundHandler::get_next_round_number))
            .route("/{id}", web::get().to(TontineRoundHandler::get_round))
            .route("/{id}", web::put().to(TontineRoundHandler::update_round))
            .route("/{id}", web::delete().to(TontineRoundHandler::delete_round))
            .route("/{id}/complete", web::put().to(TontineRoundHandler::complete_round))
            .route("/{id}/cancel", web::put().to(TontineRoundHandler::cancel_round))
    );
}