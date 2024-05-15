use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use tower_http::cors::{Any, CorsLayer};


mod app_state;

mod routes;


#[derive(Serialize, Deserialize)]
struct Settings {
    active: bool,
    marketing: bool,
}

#[derive(Serialize)]
struct User<'a> {
    name: &'a str,
    settings: Settings,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let shared_state = Arc::new(app_state::AppState::default());

    let listener = tokio::net::TcpListener::bind(shared_state.addr.clone())
        .await
        .unwrap();

    axum::serve(listener, routes::configure_routes(shared_state))
        .await
        .expect("Failed to run Axum server");

    Ok(())
}

fn init_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
