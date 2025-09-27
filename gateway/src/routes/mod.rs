mod proof;
mod types;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Health {
    status: String,
}

async fn get_health() -> impl Responder {
    let health = Health {
        status: "Ok".to_string(),
    };
    HttpResponse::Ok().json(health)
}

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(get_health))
            .service(
                web::resource("/v1/proof")
                    .route(web::post().to(proof::submit_proof))
                    .route(web::get().to(proof::get_submissions))
            )
    );
}
