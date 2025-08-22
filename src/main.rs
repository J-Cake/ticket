use std::sync::Arc;
use actix_web::web;
use clap::Parser;
use crate::auth::authenticate_middleware;

pub mod config;
pub mod auth;
pub mod jwk;
pub mod oidc;
mod api;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    env_logger::init();

    let cli = config::CliArgs::parse();

    let config = web::Data::new(toml::de::from_str::<config::Config>(&tokio::fs::read_to_string(&cli.config).await?)
        .expect("invalid config file"));

    let http = web::Data::new(reqwest::Client::new());

    let jwk = web::Data::new(jwk::get(http.clone(), config.clone()).await?);

    let db = web::Data::new(sqlx::PgPool::connect(&config.server.database).await
        .expect("Failed to connect to database"));

    let _config = config.clone();
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(_config.clone())
            .app_data(http.clone())
            .app_data(jwk.clone())
            .app_data(db.clone())
            .wrap(actix_web::middleware::from_fn(authenticate_middleware))
            .service(api::list_tickets)
    })
    .bind(config.server.listen)?
    .run()
    .await?;

    Ok(())
}
