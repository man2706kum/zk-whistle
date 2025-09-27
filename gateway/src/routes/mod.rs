mod proof;
mod types;
use poem::{IntoResponse, Response, Route, get, handler, http::StatusCode, post, web::Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Health {
    status: String,
}

#[handler]
async fn get_health() -> Response {
    let health = Health {
        status: "Ok".to_string(),
    };
    Json(serde_json::json!(health))
        .with_status(StatusCode::OK)
        .into_response()
}

pub fn api() -> Route {
    let api = Route::new()
        .at("/health", get(get_health))
        .at("/v1/proof",post(proof::submit_proof).get(proof::get_submissions),
    );

    Route::new().nest("/api", api)
}
