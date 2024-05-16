use std::process::exit;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tracing::{debug, error, Level};
use tracing_subscriber::FmtSubscriber;

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

    let listener = tokio::net::TcpListener::bind(get_addr(&shared_state.addr.clone()).await)
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


async fn get_addr(url_raw: &str) -> String
{
    let mut url = if url_raw.starts_with("http://") {
        url_raw.trim_start_matches("http://").to_string()
    } else if url_raw.starts_with("https://") {
        url_raw.trim_start_matches("https://").to_string()
    } else {
        url_raw.to_string()
    };

    if let Some(colon_pos) = url.rfind(':') {
        let port = &url[colon_pos..];

        let full = format!("0.0.0.0{}", port);
        debug!("formatted addr: {}", &full);

        full
    } else {
        error!("No port number found in the URL.");
        exit(0);
    }
}
