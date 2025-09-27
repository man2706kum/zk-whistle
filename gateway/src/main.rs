mod config;
mod errors;
mod models;
mod routes;

use dotenv::dotenv;
use poem::{EndpointExt, middleware::Cors, Server, listener::TcpListener, middleware::Tracing};

use crate::config::fetch_expose_url;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "poem=debug");
        }
    }
    let cors = Cors::new();

    config::init_db().await.unwrap();
    tracing_subscriber::fmt::init();

    Server::new(TcpListener::bind(fetch_expose_url().unwrap()))
        .run(routes::api().with(Tracing).with(cors))
        .await
}

#[cfg(test)]
mod tests {
    use poem::test::TestClient;

    use crate::routes::api;

    #[tokio::test]
    async fn test_health() {
        // Create a test client from the route
        let cli = TestClient::new(api());
        let resp = cli.get("/api/health").send().await;

        resp.assert_status_is_ok();

        let json = resp.json().await;
        let json_value = json.value();

        json_value.object().get("status").assert_string("Ok");
    }
}
