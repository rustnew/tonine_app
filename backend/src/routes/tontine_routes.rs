use actix_web::web;
use crate::handlers::tontine_handlers::TontineHandler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/tontines")
            .route("", web::get().to(TontineHandler::get_tontines))
            .route("", web::post().to(TontineHandler::create_tontine))
            .route("/active", web::get().to(TontineHandler::get_active_tontines))
            .route("/user/{user_id}", web::get().to(TontineHandler::get_user_tontines))
            .route("/{id}", web::get().to(TontineHandler::get_tontine))
            .route("/{id}/details", web::get().to(TontineHandler::get_tontine_with_creator))
            .route("/{id}", web::put().to(TontineHandler::update_tontine))
            .route("/{id}", web::delete().to(TontineHandler::delete_tontine))
            .route("/{id}/increment-round", web::put().to(TontineHandler::increment_round))
    );
}