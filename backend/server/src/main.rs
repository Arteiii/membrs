use std::env;
use std::sync::Arc;

use dotenv::dotenv;
use human_panic::{setup_panic, Metadata};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::{debug, error, info};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

use membrs_lib::bot::Bot;

use crate::db::application_data::ApplicationData;
use crate::db::superuser::SuperUser;
use crate::db::users::UserData;

mod db;
mod routes;

mod handlers;

struct EnvArgs {
    backend_url: String,
    frontend_url: String,
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
                error!("{:?}", err);
                None
            }
        };

        // Fetch PostgreSQL connection details
        let postgres_user =
            env::var("POSTGRES_USER").expect("POSTGRES_USER environment variable is not set");
        let postgres_password = env::var("POSTGRES_PASSWORD")
            .expect("POSTGRES_PASSWORD environment variable is not set");
        let postgres_db =
            env::var("POSTGRES_DB").expect("POSTGRES_DB environment variable is not set");
        let postgres_host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string()); // default localhost if not set
        let postgres_port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string()); // default 5432 if not set

        let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string()); // set default prot to 8000

        // Construct the DATABASE_URL
        let postgres = format!(
            "postgres://{}:{}@{}:{}/{}",
            postgres_user, postgres_password, postgres_host, postgres_port, postgres_db
        );

        Self {
            backend_url: env::var("BACKEND_URL")
                .expect("BACKEND_URL environment variable is not set"),
            frontend_url: env::var("FRONTEND_URL")
                .expect("FRONTEND_URL environment variable is not set"),
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

    // todo: add config for addr
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &args.port))
        .await
        .unwrap();
    debug!("connecting to: {:?}", &args.postgres);

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&args.postgres)
        .await
        .expect("failed to create pg connection");

    create_tables(&pool).await;

    // Store the bot token in the database
    store_bot_and_urls(&pool, &args.token, &args.backend_url, &args.frontend_url)
        .await
        .expect("failed to store bot token");

    SuperUser::check_and_create_superuser(&pool)
        .await
        .expect("failed to store superuser");

    let bot = args.token.as_ref().map(|token| Bot::new(token));

    let shared_state = Arc::new(AppState { pool, bot });

    axum::serve(listener, routes::configure_routes(shared_state))
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
pub async fn store_bot_and_urls(
    pool: &PgPool,
    bot_token: &Option<String>,
    backend_url: &str,
    frontend_url: &str,
) -> Result<(), sqlx::Error> {
    // Execute the upsert query
    ApplicationData::soft_insert_application_data(
        pool,
        &ApplicationData {
            id: 0,
            app_name: "application_data".to_string(),
            backend_url: Some(backend_url.to_string()),
            frontend_url: Some(frontend_url.to_string()),
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
        "Updated values: bot_token: {:?}, backend_url: {:?}, frontend_url: {:?}",
        result.bot_token, result.backend_url, result.frontend_url
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
