
mod config;
mod errors;
mod models;
mod routes;

use dotenv::dotenv;
use actix_web::{App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use crate::config::fetch_expose_url;

// Note: This will work after actix_web is added as a dependency
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=debug");
        }
    }
    config::init_db().await.unwrap();
    env_logger::init();

    let expose_url = fetch_expose_url().unwrap();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
            )
            .configure(routes::api)
    })
    .bind(expose_url)?
    .run()
    .await
}

// TODO: Rewrite tests using actix_web::test if needed
