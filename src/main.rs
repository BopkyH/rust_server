mod config;
mod db;
mod errors;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;
mod schema;
mod middleware;

use actix_web::{middleware::Logger, App, HttpServer, web};
use config::AppConfig;
use db::init_pool;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};
use middleware::auth::AuthMiddleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::from_env().expect("Failed to load configuration");

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.log_level));
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_timer(fmt::time::ChronoLocal::rfc_3339())
        .init();

    info!("Starting server at {}:{}", config.server_host, config.server_port);

    let pool = init_pool(&config.database_url).expect("Failed to create DB pool");
    let pool_data = web::Data::new(pool); // ВАЖЛИВО: web::Data

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(AuthMiddleware)
            .app_data(pool_data.clone())
            .configure(routes::configure)
    })
    .bind((config.server_host.clone(), config.server_port))?
    .run()
    .await
}
