use std::env;
use std::sync::Arc;

use dotenv::dotenv;
use human_panic::{setup_panic, Metadata};
use discord_lib::bot::Bot;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::{debug, error};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

use crate::db::application_data::ApplicationData;
use crate::db::superuser::SuperUser;
use crate::db::users::UserData;

mod db;
mod routes;

mod handlers;

#[derive(Debug)]
struct EnvArgs {
    url: String,
    postgres: String,
    port: String,
}

/// helper struct to share data using states
pub struct AppState {
    /// postgresql pool
    pub pool: PgPool,
    /// discord bot instance
    pub bot: Option<Bot>,
}

impl EnvArgs {
    #[inline]
    fn new() -> Self {
        dotenv().ok();

        Self {
            url: env::var("URL").expect("URL environment variable is not set"),
            port: env::var("PORT").unwrap_or_else(|_| "8000".to_string()),
            postgres: env::var("POSTGRES").expect("POSTGRES environment variable is not set"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();
    let args = EnvArgs::new();
    debug!("args: {:?}", args);

    // test for https error
    reqwest::get("https://google.com/").await.unwrap();
    reqwest::get("http://google.com/").await.unwrap();
    reqwest::get("https://discord.com/").await.unwrap();

    // todo: add config for addr
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &args.port))
        .await
        .unwrap();

    debug!("connecting to: {:?}", &args.postgres);

    let pool: PgPool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&args.postgres)
        .await
    {
        Ok(pool) => pool,
        Err(err) => {
            error!("Failed to create pg connection: {:?}", err);
            panic!("Failed to create pg connection");
        }
    };

    create_tables(&pool).await;

    SuperUser::check_and_create_superuser(&pool)
        .await
        .expect("failed to store superuser");

    let bot = match ApplicationData::get_bot_token(&pool).await {
        Ok(Some(token)) => Some(Bot::new(&token)),
        Ok(None) => None,
        Err(err) => {
            error!("get bot token error: {:?}", err);
            None
        }
    };

    let shared_state = Arc::new(AppState { pool, bot });

    axum::serve(listener, routes::configure_routes(shared_state, args.url))
        .await
        .expect("Failed to run Axum server");

    Ok(())
}

#[inline(always)]
fn init_tracing() {
    setup_panic!(
        Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
            .authors("Arteii <ben.arteii@proton.me>")
            .homepage("https://github.com/Arteiii/membrs")
            .support("- Open a support request at https://github.com/Arteiii/membrs/issues/new")
    );

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}

#[inline(always)]
pub async fn create_tables(pool: &PgPool) {
    match ApplicationData::create_application_data_table(pool).await {
        Ok(_) => {
            debug!("Application data table creation successful");
        }
        Err(err) => {
            error!("Error creating application data table: {:?}", err);
            panic!("Error creating application data table: {:?}", err);
        }
    }

    match SuperUser::create_table(pool).await {
        Ok(_) => {
            debug!("SuperUser table creation successful");
        }
        Err(err) => {
            error!("Error creating SuperUser table: {:?}", err);
            panic!("Error creating SuperUser table: {:?}", err);
        }
    }

    match UserData::create_user_data_table(pool).await {
        Ok(_) => {
            debug!("User data table creation successful");
        }
        Err(err) => {
            error!("Error creating user data table: {:?}", err);
            panic!("Error creating user data table: {:?}", err);
        }
    }
}
