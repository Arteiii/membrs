use std::process::exit;
use std::sync::Arc;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::Mutex;
use tokio::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, Level};
use tracing_subscriber::FmtSubscriber;

use membrs_lib::bot::Bot;

use crate::app_state::AppState;

mod app_state;

mod db;
mod routes;

struct EnvArgs {
    addr: String,
    postgres: String,
    token: String,
}

impl EnvArgs {
    fn new() -> Self {
        todo!();

        Self {
            addr: "".to_string(),
            postgres: "".to_string(),
            token: "".to_string(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();
    let args = EnvArgs::new();

    // todo: add config for addr
    let listener = tokio::net::TcpListener::bind(get_addr(&args.addr).await)
        .await
        .unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&args.postgres)
        .await
        .expect("failed to create pg connection");

    // create_application_data_table(&pool).await.unwrap();

    // Store the bot token in the database
    store_bot_token(&pool, &args.token)
        .await
        .expect("failed to store bot token");


    let shared_state = Arc::new(app_state::AppState {
        pool,
        bot: Bot::new(&args.token),
    });
    
    axum::serve(listener, routes::configure_routes(shared_state))
        .await
        .expect("Failed to run Axum server");

    Ok(())
}

async fn continous_tasks(state: Arc<Mutex<AppState>>) {
    loop {
        state.lock().await.pool;

        // Sleep for 1 hour before running the cleanup task again
        sleep(Duration::from_secs(3600)).await;
    }
}

fn init_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

async fn get_addr(url_raw: &str) -> String {
    let url = if url_raw.starts_with("http://") {
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

async fn store_bot_token(pool: &PgPool, bot_token: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO application_data (app_name, bot_token)
        VALUES ('bot_token', $1)
        ON CONFLICT (app_name)
        DO UPDATE SET bot_token = $1
        "#,
        bot_token
    )
        .execute(pool)
        .await?;

    Ok(())
}

async fn create_application_data_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
            CREATE TABLE application_data (
                id SERIAL PRIMARY KEY,
                app_name VARCHAR(255) NOT NULL UNIQUE,
                backend_url VARCHAR(255),
                frontend_url VARCHAR(255),
                bot_token VARCHAR(255),
                oauth_url VARCHAR(255),
                client_id VARCHAR(255),
                redirect_uri VARCHAR(255),
                client_secret VARCHAR(255),
                guild_id VARCHAR(255)
            );
        "#,
    )
        .execute(pool)
        .await?;
    Ok(())
}
