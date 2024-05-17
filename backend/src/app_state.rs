use sqlx::PgPool;

use membrs_lib::bot::Bot;

// helper struct to share data using states
#[allow(dead_code)]
pub struct AppState {
    ///
    pub pool: PgPool,
    /// discord bot instance
    pub bot: Bot,
}
