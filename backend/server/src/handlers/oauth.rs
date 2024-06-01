use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::response::Redirect;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, error};

use membrs_lib::bot::{AddGuildMember, Bot};
use membrs_lib::model::user;
use membrs_lib::oauth;
use membrs_lib::oauth::{ClientData, OAuthToken};

use crate::db::{application_data::ApplicationData, users::UserData};
use crate::AppState;

#[derive(Deserialize, Serialize)]
struct User {
    data: user::UserData,
    token: OAuthToken,
}

#[derive(Debug, Deserialize, Serialize)]
struct OauthUrl {
    oauth_url: Option<String>,
}

pub(crate) async fn oauth_callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Redirect, Redirect> {
    let data = match ApplicationData::get_application_data(&state.pool).await {
        Ok(data) => data,
        Err(err) => {
            error!("db error during get first application: {}", err);
            return Err(Redirect::temporary(
                "/complete?status=failed&error=unknown_error",
            ));
        }
    };

    let bot = match state.bot.clone() {
        Some(bot) => bot,
        None => {
            match ApplicationData::get_bot_token(&state.pool).await {
                Ok(Some(token)) => Bot::new(&token),
                Ok(None) => {
                    error!("Bot is not set up correctly. Please visit the admin dashboard.");
                    return Err(Redirect::temporary("/complete?status=failed&error=bot_setup_not_completed"));
                }
                Err(_) => {
                    error!("Failed to retrieve bot token from the database.");
                    return Err(Redirect::temporary("/complete?status=failed&error=failed_to_retrieve_bot_token_from_database"));
                }
            }
        }
    };

    let cdata = ClientData {
        client_id: data.client_id.ok_or_else(|| {
            error!("Client ID not found");
            Redirect::temporary("/complete?status=failed&error=client_id_not_found")
        })?,
        client_secret: data.client_secret.ok_or_else(|| {
            error!("Client secret not found");
            Redirect::temporary("/complete?status=failed&client_secret_not_found")
        })?,
        redirect_uri: data.redirect_uri.ok_or_else(|| {
            error!("Redirect URI not found");
            Redirect::temporary("/complete?status=failed&redirect_uri_not_found")
        })?,
    };

    let authorization_code = params
        .get("code")
        .ok_or_else(|| {
            error!("Authorization code not found in query parameters");
            Redirect::temporary("/complete?status=failed&authorization_code_not_found")
        })?;

    let client = oauth::OAuthClient::new(&cdata, authorization_code);

    match client.await {
        Ok(client) => {
            // Assuming client.get_user_data() returns UserData struct
            let user_data = match client.get_user_data().await {
                Ok(user_data) => user_data,
                Err(err) => {
                    // Handle authentication error
                    error!("OAuth error: {:?}", err);
                    return Err(Redirect::temporary(&format!(
                        "/complete?status=failed&error={:?}",
                        err
                    )));
                }
            };
            debug!("user data: {:?}", user_data);

            let token = client.get_token().await;

            let ud = UserData {
                id: 0,
                discord_id: Some(user_data.id.clone()),
                username: user_data.username.clone(),
                avatar: user_data.avatar,
                email: user_data.email,
                banner: user_data.banner,
                access_token: Some(token.access_token),
                token_type: Some(token.token_type),
                expires_at: Some(token.expires_at),
                refresh_token: Some(token.refresh_token.unwrap_or_else(|| "".to_string())),
            };

            if let Err(err) = ud.insert_user_data(&state.pool).await {
                error!("Failed to insert user data: {:?}", err);
            }

            let guild_id = if let Some(id) = &data.guild_id {
                id
            } else {
                error!("Guild ID not found");
                return Err(Redirect::temporary("/complete?status=failed&missing_guild_id"));
            };

            match bot
                .add_guild_member(AddGuildMember::new(
                    guild_id,
                    &user_data.id,
                    &client.get_token().await,
                ))
                .await
            {
                Ok(res) => {
                    debug!("Add Guild Member Response: {:?}", res);
                    Ok(Redirect::temporary(&format!(
                        "/complete?status=complete&username={}",
                        user_data.username.unwrap_or_else(|| "Unknown".to_string())
                    )))
                }
                Err(err) => {
                    // Handle error
                    let msg = format!("Failed to add guild member: {:?}", err);
                    error!(msg);
                    Err(Redirect::temporary(&format!(
                        "/complete?status=failed&error={}",
                        user_data.username.unwrap_or_else(|| "Unknown".to_string())
                    )))
                }
            }
        }
        Err(err) => {
            // Handle OAuthClient::new error
            error!("OAuth error: {:?}", err);
            Err(Redirect::temporary(&format!(
                "/complete?status=failed&error={:?}",
                err
            )))
        }
    }
}

pub(crate) async fn oauth_url(State(state): State<Arc<AppState>>) -> Result<Redirect, Redirect> {
    match ApplicationData::get_oauth_url(&state.pool).await {
        Ok(cdata) => match cdata {
            Some(url) => Ok(Redirect::temporary(&url)),
            None => {
                eprintln!("OAuth URL is NULL in the database");
                Err(Redirect::temporary(
                    "/complete?status=failed&error=OAuth URL is Null",
                ))
            }
        },
        Err(err) => {
            eprintln!("Failed to fetch OAuth URL: {:?}", err);
            Err(Redirect::temporary(
                "/complete?status=failed&error=Failed to fetch OAuth URL",
            ))
        }
    }
}
