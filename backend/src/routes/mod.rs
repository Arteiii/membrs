use std::sync::Arc;

use axum::{Router, routing::get};
use tower_http::cors::{Any, CorsLayer};

use crate::app_state::AppState;

pub mod handlers;

pub fn configure_routes(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new().allow_origin(Any);


    Router::new()
        .route("/", get(handlers::index))
        .route("/oauth", get(handlers::oauth::oauth_callback))
        .route("/oauth/url", get(handlers::oauth::oauth_url))
        .route("/users", get(handlers::users::get_user_list))
        .with_state(state)
        .layer(cors)
}
