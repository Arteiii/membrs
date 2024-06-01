use std::env;
use std::sync::Arc;

use dotenv::dotenv;
use human_panic::{setup_panic, Metadata};
use membrs_lib::bot::Bot;
use reqwest::Client;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::{debug, error, info};
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
    token: Option<String>,
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
        // Load values from the .env file if it exists
        dotenv().ok();

        let token = match env::var("BOT_TOKEN") {
            Ok(token) => Some(token),
            Err(err) => {
                error!("BOT_TOKEN not found ({:?})", err);
                None
            }
        };

        // Fetch PostgreSQL connection details
        let postgres = env::var("POSTGRES").expect("POSTGRES environment variable is not set");

        let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string()); // set default prot to 8000

        Self {
            url: env::var("URL").expect("URL environment variable is not set"),
            port,
            postgres,
            token,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();
    let args = EnvArgs::new();
    debug!("args: {:?}", args);

    reqwest::get("https://google.com/").await.unwrap();

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

    // Store the bot token in the database
    store_bot_and_urls(&pool, &args.token)
        .await
        .expect("failed to store bot token");

    SuperUser::check_and_create_superuser(&pool)
        .await
        .expect("failed to store superuser");

    let bot = if let Some(token) = args.token.as_ref() {
        Some(Bot::new(token))
    } else {
        match ApplicationData::get_bot_token(&pool).await {
            Ok(Some(token)) => Some(Bot::new(&token)),
            Ok(None) => None,
            Err(_) => None,
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
            .support(
                "- \n- Open a support request at https://github.com/Arteiii/membrs/issues/new"
            )
    );

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}

#[inline(always)]
pub async fn store_bot_and_urls(
    pool: &PgPool,
    bot_token: &Option<String>,
) -> Result<(), sqlx::Error> {
    // Execute the upsert query
    ApplicationData::soft_insert_application_data(
        pool,
        &ApplicationData {
            id: 0,
            app_name: "application_data".to_string(),
            url: None,
            bot_token: bot_token.clone(),
            oauth_url: None,
            client_id: None,
            redirect_uri: None,
            client_secret: None,
            guild_id: None,
        },
    )
    .await
    .expect("TODO: panic message");

    // Fetch the updated values to verify the update
    let result = ApplicationData::get_application_data(pool).await.unwrap();

    debug!(
        "Updated values: bot_token: {:?}, url: {:?}",
        result.bot_token, result.url,
    );

    Ok(())
}

#[inline(always)]
pub async fn create_tables(pool: &PgPool) {
    match ApplicationData::create_application_data_table(pool).await {
        Ok(_) => {
            info!("Application data table creation successful");
        }
        Err(err) => {
            error!("Error creating application data table: {:?}", err);
            panic!("Error creating application data table: {:?}", err);
        }
    }

    match SuperUser::create_table(pool).await {
        Ok(_) => {
            info!("SuperUser table creation successful");
        }
        Err(err) => {
            error!("Error creating SuperUser table: {:?}", err);
            panic!("Error creating SuperUser table: {:?}", err);
        }
    }

    match UserData::create_user_data_table(pool).await {
        Ok(_) => {
            info!("User data table creation successful");
        }
        Err(err) => {
            error!("Error creating user data table: {:?}", err);
            panic!("Error creating user data table: {:?}", err);
        }
    }
}
