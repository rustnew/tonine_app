use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::handlers::auth_handler::AuthHandler;
use crate::auth::middleware::validator;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth_middleware = HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/api/auth")
            // Routes publiques
            .route("/login", web::post().to(AuthHandler::login))
            .route("/logout", web::post().to(AuthHandler::logout))
            .route("/request-password-reset", web::post().to(AuthHandler::request_password_reset))
            .route("/confirm-password-reset", web::post().to(AuthHandler::confirm_password_reset))
            // Routes protégées
            .service(
                web::scope("")
                    .wrap(auth_middleware)
                    .route("/me", web::get().to(AuthHandler::get_me))
                    .route("/change-password", web::put().to(AuthHandler::change_password))
                    .route("/refresh-token", web::post().to(AuthHandler::refresh_token))
            )
    );
}