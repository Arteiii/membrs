use std::error::Error;
use std::sync::Arc;

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use axum::Json;
use base64::engine::general_purpose;
use base64::Engine;
use chrono::{DateTime, Utc};
use discord_lib::bot::{AddGuildMember, Bot};
use discord_lib::model;
use discord_lib::oauth::{ClientData, OAuthClient, OAuthError, OAuthToken};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use tokio::task;
use tracing::{debug, error, info, trace};

use discord_lib::oauth::url::DiscordOAuthUrlBuilder;

use crate::db::application_data::ApplicationData;
use crate::db::superuser::SuperUser;
use crate::db::users::UserData;
use crate::AppState;

#[derive(Deserialize, Serialize, Debug)]
pub struct ApplicationDataResult {
    pub url: String,
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
            url: data.url.unwrap_or_default(),
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
    authorize(&headers, &state.pool).await?;

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

pub(crate) async fn authenticate_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<String, Response<String>> {
    authorize(&headers, &state.pool).await?;

    Ok("Success".to_string())
}

#[derive(Deserialize)]
pub struct UpdateUserRequestBody {
    new_username: String,
    new_password: String,
}

pub(crate) async fn update_superuser(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    body: Json<UpdateUserRequestBody>,
) -> Result<String, Response<String>> {
    authorize(&headers, &state.pool).await?;

    let new_username = &body.new_username;
    let new_password = &body.new_password;

    match SuperUser::upsert(
        &state.pool,
        Some(new_username.clone()),
        Some(new_password.clone()),
    )
    .await
    {
        Ok(_) => Ok("Success".to_string()),
        Err(response) => {
            error!("update superuser in db failed: {}", response);

            Err(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("failed to update username/password".to_string())
                .unwrap())
        }
    }
}

pub(crate) async fn get_users(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Vec<GetUsersResponse>>, Response<String>> {
    authorize(&headers, &state.pool).await?;

    match UserData::get_users(&state.pool, 10).await {
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

pub(crate) async fn get_bot_guilds(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Vec<model::bot::Guild>>, Response<String>> {
    // Authorize the request
    match authorize(&headers, &state.pool).await {
        Ok(()) => (),
        Err(err) => {
            error!("Authorization failed: {:?}", err);
            return Err(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(json!({ "error": "Authorization failed" }).to_string())
                .unwrap());
        }
    }

    // Get the bot instance
    let bot = match state.bot.clone() {
        Some(bot) => bot,
        None => match ApplicationData::get_bot_token(&state.pool).await {
            Ok(Some(token)) => Bot::new(&token),
            Ok(None) => {
                error!("Bot is not set up correctly. Please visit the admin dashboard.");
                return Err(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(json!({ "error": "Bot is not set up correctly" }).to_string())
                    .unwrap());
            }
            Err(err) => {
                error!("Failed to retrieve bot token from the database: {:?}", err);
                return Err(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(
                        json!({ "error": "Failed to retrieve bot token from the database" })
                            .to_string(),
                    )
                    .unwrap());
            }
        },
    };

    match bot.get_guilds().await {
        Ok(guilds) => {
            trace!("Retrieved guilds: {:?}", guilds);
            Ok(Json(guilds))
        }
        Err(err) => {
            error!("Failed to get guilds: {:?}", err);
            Err(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(json!({ "error": "Failed to get guilds" }).to_string())
                .unwrap())
        }
    }
}

pub(crate) async fn set_config(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<SetApplicationData>,
) -> Result<Json<String>, Response<String>> {
    authorize(&headers, &state.pool).await?;

    let oauth_url = DiscordOAuthUrlBuilder::new(&payload.client_id, &payload.redirect_uri)
        .email() // allow reading email at users/@me
        .identify() // allow reading email at users/@me
        .guilds() // allow reading guilds
        .guilds_join()
        .build();

    debug!("new oauth url generated {}", &oauth_url);

    let app_data = ApplicationData {
        id: 0,
        app_name: "application_data".to_string(),
        url: None,
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

/// Authorizes a request using Basic authentication
///
/// This function checks the `Authorization` header for valid credentials,
/// decodes the Base64 username and password, and verifies them against a superuser
/// fetched from the database. Returns `Ok(())` on successful authentication,
/// otherwise returns an error response
///
/// # Arguments
///
/// * `headers` - The request headers containing the `Authorization` header
/// * `pool` - A connection pool to the PostgreSQL database
///
/// # Errors
///
/// Returns an error response if the `Authorization` header is missing,
/// incorrectly formatted, or if the credentials do not match the superuser
async fn authorize(headers: &HeaderMap, pool: &PgPool) -> Result<(), Response<String>> {
    let (username, password) = match extract_username_password(headers, "Authorization").await {
        Ok((username, password)) => (username, password),
        Err(response) => return Err(response),
    };

    let superuser = match SuperUser::fetch(pool).await {
        Ok(Some(superuser)) => superuser,
        Ok(None) => {
            error!("Superuser not found in the database");

            let response = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body("user not found".to_string())
                .unwrap();

            return Err(response);
        }
        Err(e) => {
            error!("Error fetching superuser from the database: {}", e);

            let response = Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("couldn't fetch superuser".to_string())
                .unwrap();

            return Err(response);
        }
    };

    debug!("SuperUser data: {:?}", superuser);

    match (superuser.username.as_deref(), superuser.password.as_deref()) {
        (Some(super_username), Some(super_password)) => {
            if super_username == username && super_password == password {
                info!("login successfully as: {}", username);
                Ok(())
            } else {
                error!("Username or password mismatch");

                let response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("Username or password mismatch".to_string())
                    .unwrap();

                Err(response)
            }
        }
        _ => {
            error!("Attempted login with wrongly formatted authorization header");

            let response = Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .body("wrongly formatted authorization header".to_string())
                .unwrap();

            Err(response)
        }
    }
}

async fn extract_username_password(
    headers: &HeaderMap,
    header_name: &str,
) -> Result<(String, String), Response<String>> {
    // Check if Authorization header exists
    let authorization_header = match headers.get(header_name) {
        Some(header) => header,
        None => {
            error!("attempted login with missing authorization header");

            let response = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body("missing authorization header".to_string())
                .unwrap();

            return Err(response);
        }
    };

    // Check if the Authorization header starts with "Basic "
    let auth_str = match authorization_header.to_str().ok() {
        Some(auth_str) if auth_str.starts_with("Basic ") => auth_str,
        _ => {
            error!("attempted login with wrongly formatted authorization header");

            let response = Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .body("wrongly formatted authorization header".to_string())
                .unwrap();

            return Err(response);
        }
    };

    // Decode the Base64 encoded username:password string
    let auth_decoded = match general_purpose::STANDARD.decode(auth_str.trim_start_matches("Basic "))
    {
        Ok(auth_decoded) => auth_decoded,
        Err(e) => {
            error!("Error decoding Base64: {}", e);

            let response = Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .body("couldn't parse authorization header".to_string())
                .unwrap();

            return Err(response);
        }
    };

    // Convert the decoded bytes to a string
    let auth_string = match String::from_utf8(auth_decoded) {
        Ok(auth_string) => auth_string,
        Err(e) => {
            error!("Error converting bytes to string: {}", e);

            let response = Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .body("couldn't parse authorization header".to_string())
                .unwrap();

            return Err(response);
        }
    };

    // Split the string into username and password
    let mut parts = auth_string.splitn(2, ':');
    let (username, password) = match (parts.next(), parts.next()) {
        (Some(username), Some(password)) => (username.to_string(), password.to_string()),
        _ => {
            error!("attempted login with wrongly formatted authorization header");

            let response = Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .body("wrongly formatted authorization header".to_string())
                .unwrap();

            return Err(response);
        }
    };

    Ok((username, password))
}

#[derive(Deserialize)]
pub struct PullMembers {
    guild_id: String,
}

pub(crate) async fn pull_members(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: Json<PullMembers>,
) -> Result<String, Response<String>> {
    authorize(&headers, &state.pool).await?;

    let guild_id = body.guild_id.clone();

    task::spawn(async {
        pull_all(state, guild_id).await;
    });

    Ok("Success!! Please Wait...".to_string())
}

async fn pull_all(state: Arc<AppState>, guild_id: String) {
    if let Err(err) = pull_all_internal(state.clone(), guild_id.clone()).await {
        error!("Error pulling all users for guild {}: {}", guild_id, err);
    }
}

/// untested!!!
async fn pull_all_internal(state: Arc<AppState>, guild_id: String) -> Result<(), Box<dyn Error>> {
    info!("Starting to pull all users for guild {}", guild_id);

    let client_data = {
        let data = match ApplicationData::get_application_data(&state.pool).await {
            Ok(data) => data,
            Err(err) => {
                let msg = format!("db error during get first application: {}", err);
                error!("{}", msg);
                return Err(msg.into());
            }
        };

        ClientData {
            client_id: match data.client_id {
                Some(id) => id,
                None => {
                    let msg = "Client ID not found".to_string();
                    error!("{}", msg);
                    return Err(msg.into());
                }
            },
            client_secret: match data.client_secret {
                Some(secret) => secret,
                None => {
                    let msg = "Client secret not found".to_string();
                    error!("{}", msg);
                    return Err(msg.into());
                }
            },
            redirect_uri: match data.redirect_uri {
                Some(uri) => uri,
                None => {
                    let msg = "Redirect URI not found".to_string();
                    error!("{}", msg);
                    return Err(msg.into());
                }
            },
        }
    };

    let bot = match state.bot.clone() {
        Some(bot) => bot,
        None => match ApplicationData::get_bot_token(&state.pool).await {
            Ok(Some(token)) => Bot::new(&token),
            Ok(None) => {
                error!("Bot is not set up correctly. Please visit the admin dashboard.");
                return Err(
                    "Bot is not set up correctly. Please visit the admin dashboard.".into(),
                ); // Return an error
            }
            Err(_) => {
                error!("Failed to retrieve bot token from the database.");
                return Err("Failed to retrieve bot token from the database.".into());
                // Return an error
            }
        },
    };

    match UserData::count_users(&state.pool).await {
        Ok(count) => {
            info!("Total {} users found in the database", count);

            for i in 1..=count {
                match UserData::get_users(&state.pool, i).await {
                    Ok(users) => {
                        info!("Successfully fetched {} users", users.len());

                        for user in users {
                            let guild_id_clone = guild_id.clone();
                            let user_clone = user.clone();
                            let client_data = client_data.clone();
                            let bot = bot.clone();

                            task::spawn(async move {
                                match pull_one(bot, user_clone, &guild_id_clone, &client_data).await
                                {
                                    Ok(_) => info!(
                                        "Successfully processed user {} for guild {}",
                                        user.id, guild_id_clone
                                    ),
                                    Err(e) => error!(
                                        "Error processing user {} for guild {}: {:?}",
                                        user.id, guild_id_clone, e
                                    ),
                                }
                            });
                        }
                    }
                    Err(e) => {
                        error!("Error fetching users: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            error!("Error counting users: {:?}", e);
            return Err(e.into());
        }
    }

    Ok(())
}

async fn pull_one(
    bot: Bot,
    user_data: UserData,
    guild_id: &str,
    client_data: &ClientData,
) -> Result<(), OAuthError> {
    debug!("user data: {:?}", user_data);
    debug!("guild_id: {}", guild_id);

    let token = OAuthToken {
        access_token: user_data.access_token.unwrap(),
        token_type: user_data.token_type.unwrap(),
        expires_at: user_data.expires_at.unwrap(),
        refresh_token: Some(user_data.refresh_token.unwrap()),
    };

    let mut oclient = OAuthClient::new(client_data, &token).await.unwrap();

    const MAX_RETRIES: u8 = 3;
    let mut attempts = 0;

    loop {
        match bot
            .add_guild_member(AddGuildMember::new(
                guild_id,
                &user_data.discord_id.clone().unwrap(),
                &oclient.get_token().await?,
            ))
            .await
        {
            Ok(res) => {
                debug!("Add Guild Member Response: {:?}", res);
                return Ok(());
            }
            Err(err) => {
                attempts += 1;
                let msg = format!(
                    "Attempt {}: Failed to add guild member: {:?}",
                    attempts, err
                );
                error!(msg);

                if attempts >= MAX_RETRIES {
                    let msg = format!("Failed after {} attempts: {:?}", MAX_RETRIES, err);
                    error!("{}", msg);

                    return Err(OAuthError::RequestError(msg.to_string()));
                } else {
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
    }
}
