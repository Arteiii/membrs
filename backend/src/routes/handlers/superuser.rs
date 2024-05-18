use std::sync::Arc;

use axum::extract::State;
use axum::http::HeaderMap;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::app_state::AppState;
use crate::db::application_data::ApplicationData;
use membrs_lib::oauth::url::DiscordOAuthUrlBuilder;

#[derive(Deserialize, Serialize, Debug)]
pub struct ApplicationDataResult {
    pub backend_url: String,
    pub frontend_url: String,
    pub bot_token: String,
    pub oauth_url: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub client_secret: String,
    pub guild_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetApplicationData {
    pub bot_token: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub client_secret: String,
    pub guild_id: String,
}

impl From<ApplicationData> for ApplicationDataResult {
    fn from(data: ApplicationData) -> Self {
        ApplicationDataResult {
            backend_url: data.backend_url.unwrap_or_default(),
            frontend_url: data.frontend_url.unwrap_or_default(),
            bot_token: data.bot_token.unwrap_or_default(),
            oauth_url: data.oauth_url.unwrap_or_default(),
            client_id: data.client_id.unwrap_or_default(),
            redirect_uri: data.redirect_uri.unwrap_or_default(),
            client_secret: data.client_secret.unwrap_or_default(),
            guild_id: data.guild_id.unwrap_or_default(),
        }
    }
}

pub(crate) async fn get_config(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<ApplicationDataResult>, Json<String>> {
    if !authorize(&headers).await {
        return Err(Json("Invalid username or password".to_string()));
    }

    let data = match ApplicationData::get_application_data(&state.pool).await {
        Ok(data) => data,
        Err(err) => return Err(Json(format!("failed to get data {}", err))),
    };

    Ok(Json(data.into()))
}

pub(crate) async fn get_bot_token(headers: HeaderMap) -> Result<String, String> {
    if !authorize(&headers).await {
        return Err("Invalid username or password".to_string());
    };

    Ok("test".to_string())
}

/// update the server id and adds all the members to the new one
pub(crate) async fn update_server_id(headers: HeaderMap) -> Result<String, String> {
    if !authorize(&headers).await {
        return Err("Invalid username or password".to_string());
    };

    Ok("test".to_string())
}

pub(crate) async fn authenticate_user(headers: HeaderMap) -> Result<Json<String>, Json<String>> {
    // Check if the username and password are correct
    if authorize(&headers).await {
        Ok(Json("Success".to_string()))
    } else {
        Err(Json("Invalid username or password".to_string()))
    }
}

pub(crate) async fn set_config(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<SetApplicationData>,
) -> Result<Json<String>, Json<String>> {
    if !authorize(&headers).await {
        return Err(Json("Invalid username or password".to_string()));
    }

    let oauth_url = DiscordOAuthUrlBuilder::new(&payload.client_id, &payload.redirect_uri)
        .guilds_join()
        .build();

    debug!("new oauth url generated {}", &oauth_url);

    let app_data = ApplicationData {
        id: 0,
        app_name: "application_data".to_string(),
        backend_url: None,
        frontend_url: None,
        bot_token: Some(payload.bot_token),
        oauth_url: Some(oauth_url),
        client_id: Some(payload.client_id),
        redirect_uri: Some(payload.redirect_uri),
        client_secret: Some(payload.client_secret),
        guild_id: Some(payload.guild_id),
    };

    ApplicationData::soft_insert_application_data(&state.pool, &app_data)
        .await
        .expect("TODO: panic message");

    Ok(Json("Updated Config".to_string()))
}

/// rerun true if user was authenticated successfully
/// false otherwise
async fn authorize(headers: &HeaderMap) -> bool {
    // Get the values of the 'Username' and 'Password' headers
    let username_header = headers
        .get("Username")
        .ok_or_else(|| "Username header is missing")
        .expect("Username failed")
        .to_str()
        .map_err(|_| "Failed to parse Username header value as string")
        .expect("Username failed");

    let password_header = headers
        .get("Password")
        .ok_or_else(|| "Password header is missing")
        .expect("Password failed")
        .to_str()
        .map_err(|_| "Failed to parse Password header value as string")
        .expect("Password failed");

    // Check if the username and password are correct
    if username_header == "admin" && password_header == "admin" {
        true
    } else {
        false
    }
}
