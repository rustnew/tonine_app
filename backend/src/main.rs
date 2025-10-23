use actix_cors::Cors;
use actix_web::{App, HttpServer, web, middleware};
use actix_files::Files;
use sqlx::PgPool;
use dotenvy::dotenv;
use std::env;

pub mod model;
pub mod errors;
pub mod handlers;   
pub mod repositories;
pub mod routes;
pub mod auth;


async fn create_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await.expect("Failed to connect to DB")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = web::Data::new(create_pool().await);

    println!(" Backend TONTINE démarré sur http://127.0.0.1:8080");
    
    HttpServer::new(move || {

        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")  
            .allowed_origin("http://127.0.0.1:8080")  
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .supports_credentials()
            .max_age(3600);
            

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(pool.clone())
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}