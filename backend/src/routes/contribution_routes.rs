use actix_web::web;
use crate::handlers::contribution_handlers::ContributionHandler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/contributions")
            .route("", web::get().to(ContributionHandler::get_contributions))
            .route("", web::post().to(ContributionHandler::create_contribution))
            .route("/round/{round_id}", web::get().to(ContributionHandler::get_round_contributions))
            .route("/round/{round_id}/summary", web::get().to(ContributionHandler::get_round_summary))
            .route("/member/{member_id}", web::get().to(ContributionHandler::get_member_contributions))
            .route("/member/{member_id}/summary", web::get().to(ContributionHandler::get_member_contributions_summary))
            .route("/{id}", web::get().to(ContributionHandler::get_contribution))
            .route("/{id}", web::put().to(ContributionHandler::update_contribution))
            .route("/{id}", web::delete().to(ContributionHandler::delete_contribution))
            .route("/{id}/mark-paid", web::put().to(ContributionHandler::mark_as_paid))
            .route("/{id}/mark-failed", web::put().to(ContributionHandler::mark_as_failed))
    );
}