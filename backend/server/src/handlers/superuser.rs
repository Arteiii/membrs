use std::sync::Arc;

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use axum::Json;
use base64::engine::general_purpose;
use base64::Engine;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, error, info};

use membrs_lib::oauth::url::DiscordOAuthUrlBuilder;

use crate::db::application_data::ApplicationData;
use crate::db::superuser::SuperUser;
use crate::db::users;
use crate::AppState;

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

#[derive(Deserialize, Serialize, Debug)]
pub struct GetUsersResponse {
    pub discord_id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub email: String,
    pub banner: Option<String>,
    pub expires_at: DateTime<Utc>,
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
) -> Result<Json<ApplicationDataResult>, Response<String>> {
    if !authorize(&headers, &state.pool).await {
        let response = Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Invalid username or password".to_string())
            .unwrap();
        Err(response)
    } else {
        let data = match ApplicationData::get_application_data(&state.pool).await {
            Ok(data) => data,
            Err(err) => {
                let response = Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(format!("Failed to get data: {}", err))
                    .unwrap();
                return Err(response);
            }
        };

        Ok(Json(data.into()))
    }
}

pub(crate) async fn authenticate_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<String, Response<String>> {
    // Check if the username and password are correct
    if authorize(&headers, &state.pool).await {
        Ok("Success".to_string())
    } else {
        let response = Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Invalid username or password".to_string())
            .unwrap();
        Err(response)
    }
}

pub(crate) async fn get_users(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Vec<GetUsersResponse>>, Response<String>> {
    if !authorize(&headers, &state.pool).await {
        let response = Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Invalid username or password".to_string())
            .unwrap();
        return Err(response);
    }

    match users::UserData::get_users(&state.pool, 10).await {
        Ok(data) => {
            let mut users_response = Vec::new();

            for user_data in data {
                let expires_at = user_data.expires_at.unwrap_or_else(Utc::now);

                let user_response = GetUsersResponse {
                    discord_id: user_data.discord_id.unwrap_or_else(|| "".to_string()),
                    username: user_data.username.unwrap_or_else(|| "".to_string()),
                    avatar: user_data.avatar,
                    email: user_data.email.unwrap_or_else(|| "".to_string()),
                    banner: user_data.banner,
                    expires_at,
                };

                users_response.push(user_response);
            }

            Ok(Json(users_response))
        }
        Err(err) => {
            let response = Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("Failed to get data: {}", err))
                .unwrap();
            Err(response)
        }
    }
}

pub(crate) async fn set_config(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<SetApplicationData>,
) -> Result<Json<String>, Json<String>> {
    if !authorize(&headers, &state.pool).await {
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
        .expect("failed to insert data at set_config endpoint");

    Ok(Json("Updated Config".to_string()))
}

/// rerun true if user was authenticated successfully
/// false otherwise
async fn authorize(headers: &HeaderMap, pool: &PgPool) -> bool {
    // Check if Authorization header exists
    let authorization_header = match headers.get("Authorization") {
        Some(header) => header,
        None => {
            error!("attempted login with missing authorization header");
            return false;
        }
    };

    // Check if the Authorization header starts with "Basic "
    let auth_str = match authorization_header.to_str().ok() {
        Some(auth_str) if auth_str.starts_with("Basic ") => auth_str,
        _ => {
            error!("attempted login with wrongly formated authorization header");
            return false;
        }
    };

    // Decode the Base64 encoded username:password string
    let auth_decoded = match general_purpose::STANDARD.decode(auth_str.trim_start_matches("Basic "))
    {
        Ok(auth_decoded) => auth_decoded,
        Err(e) => {
            error!("Error decoding Base64: {}", e);
            return false;
        }
    };

    // Convert the decoded bytes to a string
    let auth_string = match String::from_utf8(auth_decoded) {
        Ok(auth_string) => auth_string,
        Err(e) => {
            error!("Error converting bytes to string: {}", e);
            return false;
        }
    };

    // Split the string into username and password
    let mut parts = auth_string.splitn(2, ':');
    let (username, password) = match (parts.next(), parts.next()) {
        (Some(username), Some(password)) => (username, password),
        _ => {
            error!("attempted login with wrongly formated authorization header");
            return false;
        }
    };

    // Fetch the superuser from the database
    let superuser = match SuperUser::fetch(pool).await {
        Ok(Some(superuser)) => superuser,
        Ok(None) => {
            error!("Superuser not found in the database");
            return false;
        }
        Err(e) => {
            error!("Error fetching superuser from the database: {}", e);
            return false;
        }
    };

    // Check if the username and password match the superuser
    let authenticated = match (superuser.username.as_deref(), superuser.password.as_deref()) {
        (Some(super_username), Some(super_password)) => {
            if super_username == username && super_password == password {
                info!("login successfully as: {}", username);
                true
            } else {
                error!("Username or password mismatch");
                false
            }
        }
        _ => {
            error!("Attempted login with wrongly formatted authorization header");
            false
        }
    };

    authenticated
}
