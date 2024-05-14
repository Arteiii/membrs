use std::sync::Arc;

use axum::{Router, routing::get};

use crate::app_state::AppState;

pub mod handlers;

pub fn configure_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/oauth", get(handlers::oauth::oauth_callback))
        .route("/oauth/url", get(handlers::oauth::oauth_url))
        .route("/users", get(handlers::users::get_user_list))
        .with_state(state)
}
