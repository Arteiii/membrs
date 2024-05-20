use std::env;
use std::process::exit;
use std::sync::Arc;

use dotenv::dotenv;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing::{debug, error, Level};
use tracing_subscriber::FmtSubscriber;

use membrs_lib::bot::Bot;

use crate::db::application_data::ApplicationData;
use crate::db::superuser::SuperUser;

mod db;
mod routes;

mod handlers;

struct EnvArgs {
	backend_url: String,
	frontend_url: String,
	postgres: String,
	token: String,
}

/// helper struct to share data using states
pub struct AppState {
	/// postgresql pool
	pub pool: PgPool,
	/// discord bot instance
	pub bot: Bot,
}

impl EnvArgs {
	fn new() -> Self {
		// Load values from the .env file if it exists
		dotenv().ok();

		Self {
			backend_url: env::var("BACKEND_URL")
				.expect("BACKEND_URL environment variable is not set"),
			frontend_url: env::var("FRONTEND_URL")
				.expect("FRONTEND_URL environment variable is not set"),
			postgres: env::var("DATABASE_URL")
				.expect("DATABASE_URL environment variable is not set"),
			token: env::var("BOT_TOKEN").expect("BOT_TOKEN environment variable is not set"),
		}
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	init_tracing();
	let args = EnvArgs::new();

	// todo: add config for addr
	let listener = tokio::net::TcpListener::bind(get_addr(&args.backend_url).await)
		.await
		.unwrap();
	debug!("connecting to: {:?}", &args.postgres);

	let pool = PgPoolOptions::new()
		.max_connections(5)
		.connect(&args.postgres)
		.await
		.expect("failed to create pg connection");

	
	// Store the bot token in the database
	store_bot_and_urls(&pool, &args.token, &args.backend_url, &args.frontend_url)
		.await
		.expect("failed to store bot token");

	SuperUser::create_table(&pool)
		.await
		.expect("failed to create superuser table");

	SuperUser::check_and_create_superuser(&pool)
		.await
		.expect("failed to store superuser");

	let shared_state = Arc::new(AppState {
		pool,
		bot: Bot::new(&args.token),
	});

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

pub async fn store_bot_and_urls(
	pool: &PgPool,
	bot_token: &str,
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
			bot_token: Some(bot_token.to_string()),
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
