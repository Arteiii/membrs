use std::sync::Arc;
use std::time::Duration;

use axum::{routing::get, Router};
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::app_state::AppState;

pub mod handlers;

pub fn configure_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handlers::index))
        .route("/oauth", get(handlers::oauth::oauth_callback))
        .route("/oauth/url", get(handlers::oauth::oauth_url))
        .route("/users", get(handlers::users::get_user_list))
        .with_state(state)
        .layer(TimeoutLayer::new(Duration::from_secs(90))) // abort request after 90sec
        .layer(CorsLayer::new().allow_origin(Any))
        .layer(TraceLayer::new_for_http())
}
