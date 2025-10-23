use actix_web::web;
use crate::handlers::user_handlers::UserHandler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .route("", web::get().to(UserHandler::get_users))
            .route("", web::post().to(UserHandler::create_user))
            .route("/{id}", web::get().to(UserHandler::get_user))
            .route("/{id}", web::put().to(UserHandler::update_user))
            .route("/{id}", web::delete().to(UserHandler::delete_user))
            .route("/{id}/change-password", web::put().to(UserHandler::change_password))
    );
}