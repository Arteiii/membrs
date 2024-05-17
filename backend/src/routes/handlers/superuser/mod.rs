use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Json as JsonBody, Query, State};
use axum::{extract, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;

use membrs_lib::oauth::ClientData;

use crate::app_state::AppState;

#[derive(Deserialize, Serialize)]
pub struct SuperUser {
    /// username
    username: String,
    /// password (hashed)
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct ConfigResult {
    /// the user itself
    superuser_data: SuperUser,
    /// Oauth client data (discord application)
    pub data: ClientData,
    /// the oauth url thats used by default (usefull for directly accessing it)
    pub oauth_url: String,
    /// the frontend url
    pub frontend_url: String,
    /// the backend url publicly accessible
    pub backend_url: String,
}

pub(crate) async fn get_config() -> Result<Json<ConfigResult>, Json<String>> {
    Err(Json("Not implemented yet".to_string()))
}

pub(crate) async fn get_bot_token() -> Result<String, String> {
    Ok("test".to_string())
}


pub(crate) async fn authenticate_user(
    extract::Json(payload): extract::Json<SuperUser>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<String>, Json<String>> {
    // Check if the username and password are correct
    if payload.username == "admin" && payload.password == "admin123" {
        
        Ok(Json("bli bla blub".to_string()))
    } else {
        Err(Json("Invalid username or password".to_string()))
    }
}

