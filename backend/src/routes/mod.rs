pub mod user_routes;
pub mod tontine_routes;
pub mod tontine_member_routes;
pub mod tontine_round_routes;
pub mod  contribution_routes;
pub mod transaction_routes;
pub mod auth_routes;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    user_routes::config(cfg);
    tontine_routes::config(cfg);
    tontine_member_routes::config(cfg);
    tontine_round_routes::config(cfg);
    contribution_routes::config(cfg);
    transaction_routes::config(cfg);
    auth_routes::config(cfg);
}