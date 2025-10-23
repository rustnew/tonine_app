use actix_web::web;
use crate::handlers::tontine_member_handlers::TontineMemberHandler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/tontine-members")
            .route("", web::get().to(TontineMemberHandler::get_members))
            .route("", web::post().to(TontineMemberHandler::create_member))
            .route("/tontine/{tontine_id}", web::get().to(TontineMemberHandler::get_tontine_members))
            .route("/tontine/{tontine_id}/count", web::get().to(TontineMemberHandler::get_member_count))
            .route("/user/{user_id}", web::get().to(TontineMemberHandler::get_user_members))
            .route("/{id}", web::get().to(TontineMemberHandler::get_member))
            .route("/{id}", web::put().to(TontineMemberHandler::update_member))
            .route("/{id}", web::delete().to(TontineMemberHandler::delete_member))
            .route("/{id}/deactivate", web::put().to(TontineMemberHandler::deactivate_member))
    );
}