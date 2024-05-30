use std::sync::Arc;
use std::time::Duration;

use axum::{
    routing::{get, post, put},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::AppState;

use crate::handlers;

pub fn configure_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handlers::index))
        .route("/oauth", get(handlers::oauth::oauth_callback))
        .route("/oauth/url", get(handlers::oauth::oauth_url))

        .route("/superuser/config", get(handlers::superuser::get_config))
        .route("/superuser/config", post(handlers::superuser::set_config))
        .route("/superuser", get(handlers::superuser::authenticate_user))
        .route("/superuser", put(handlers::superuser::update_superuser))
        .route("/superuser/users", get(handlers::superuser::get_users))
        .with_state(state)
        .layer(TimeoutLayer::new(Duration::from_secs(90))) // abort request after 90sec
        .layer(CorsLayer::new().allow_origin(Any).allow_headers(Any))
        .layer(TraceLayer::new_for_http())
}
