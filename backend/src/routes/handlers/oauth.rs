use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::response::Redirect;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, error};

use membrs_lib::bot::AddGuildMember;
use membrs_lib::model::user;
use membrs_lib::oauth;
use membrs_lib::oauth::{ClientData, OAuthToken};

use crate::app_state::AppState;
use crate::db::{application_data::ApplicationData, users::UserData};

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
                "/login/complete?status=failed&error=unknown_error",
            ));
        }
    };

    let cdata = ClientData {
        client_id: data.client_id.unwrap(),
        client_secret: data.client_secret.unwrap(),
        redirect_uri: data.redirect_uri.unwrap(),
    };

    let authorization_code = params
        .get("code")
        .ok_or_else(|| {
            error!("Authorization code not found in query parameters");
            json!({"error": "Authorization code not found"})
        })
        .unwrap();

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
                        "{}/login/complete?status=failed&error={:?}",
                        data.frontend_url.unwrap_or_else(|| "Unknown".to_string()),
                        err
                    )));
                }
            };
            debug!("user data: {:?}", user_data);
            
            let token = client.get_token().await;
            UserData::insert_user_data(
                &state.pool,
                token.access_token,
                token.token_type,
                token.expires_at,
                token.refresh_token.unwrap_or_else(|| "".to_string()),
            )
                .await.expect("TODO: panic message");

            match state
                .bot
                .add_guild_member(AddGuildMember::new(
                    &data.guild_id.unwrap_or_else(|| "Unknown".to_string()),
                    &user_data.id,
                    &client.get_token().await,
                ))
                .await
            {
                Ok(res) => {
                    debug!("Add Guild Member Response: {:?}", res);
                    Ok(Redirect::temporary(&format!(
                        "{}/login/complete?status=complete&username={}",
                        data.frontend_url.unwrap_or_else(|| "Unknown".to_string()),
                        user_data.username.unwrap_or_else(|| "Unknown".to_string())
                    )))
                }
                Err(err) => {
                    // Handle error
                    let msg = format!("Failed to add guild member: {:?}", err);
                    error!(msg);
                    Err(Redirect::temporary(&format!(
                        "{}/login/complete?status=failed&error={}",
                        data.frontend_url.unwrap_or_else(|| "Unknown".to_string()),
                        user_data.username.unwrap_or_else(|| "Unknown".to_string())
                    )))
                }
            }
        }
        Err(err) => {
            // Handle OAuthClient::new error
            error!("OAuth error: {:?}", err);
            Err(Redirect::temporary(&format!(
                "{}/login/complete?status=failed&error={:?}",
                data.frontend_url.unwrap_or_else(|| "Unknown".to_string()),
                err
            )))
        }
    }
}

pub(crate) async fn oauth_url(State(state): State<Arc<AppState>>) -> Result<String, String> {
    match ApplicationData::get_oauth_url(&state.pool).await {
        Ok(cdata) => match cdata {
            Some(url) => Ok(url),
            None => {
                eprintln!("OAuth URL is NULL in the database");
                Err(
                    "/login/complete?status=failed&error=OAuth URL is not set in the database"
                        .into(),
                )
            }
        },
        Err(err) => {
            eprintln!("Failed to fetch OAuth URL: {:?}", err);
            Err(
                "/login/complete?status=failed&error=Failed to fetch OAuth URL from the database"
                    .into(),
            )
        }
    }
}
