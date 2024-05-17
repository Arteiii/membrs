use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::response::Redirect;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, error};

use membrs_lib::bot::AddGuildMember;
use membrs_lib::model::user::UserData;
use membrs_lib::oauth;
use membrs_lib::oauth::OAuthToken;

use crate::app_state::AppState;
use crate::db;

#[derive(Deserialize, Serialize)]
struct User {
    data: UserData,
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
    let app_data = db::fetch_application_data(&state.pool).await.unwrap();
    let cdata = db::fetch_client_data(&state.pool).await.unwrap();

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
                        app_data
                            .frontend_url
                            .unwrap_or_else(|| "Unknown".to_string()),
                        err
                    )));
                }
            };
            debug!("user data: {:?}", user_data);

            let auth_info = client.get_authorization_info().await.unwrap();
            debug!("auth info: {:?}", auth_info);

            match state
                .bot
                .add_guild_member(AddGuildMember::new(
                    &app_data.guild_id.unwrap_or_else(|| "Unknown".to_string()),
                    &user_data.id,
                    &client.get_token().await,
                ))
                .await
            {
                Ok(res) => {
                    debug!("Add Guild Member Response: {:?}", res);
                    Ok(Redirect::temporary(&format!(
                        "{}/login/complete?status=complete&username={}",
                        app_data
                            .frontend_url
                            .unwrap_or_else(|| "Unknown".to_string()),
                        user_data.username.unwrap_or_else(|| "Unknown".to_string())
                    )))
                }
                Err(err) => {
                    // Handle error
                    let msg = format!("Failed to add guild member: {:?}", err);
                    error!(msg);
                    Err(Redirect::temporary(&format!(
                        "{}/login/complete?status=failed&error={}",
                        app_data
                            .frontend_url
                            .unwrap_or_else(|| "Unknown".to_string()),
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
                app_data
                    .frontend_url
                    .unwrap_or_else(|| "Unknown".to_string()),
                err
            )))
        }
    }
}

pub(crate) async fn oauth_url(State(state): State<Arc<AppState>>) -> Result<String, String> {
    match sqlx::query_as!(OauthUrl, r#"SELECT oauth_url FROM application_data"#)
        .fetch_one(&state.pool)
        .await
    {
        Ok(cdata) => match cdata.oauth_url {
            Some(url) => Ok(url),
            None => {
                eprintln!("OAuth URL is NULL in the database");
                Err("OAuth URL is not set in the database".into())
            }
        },
        Err(err) => {
            eprintln!("Failed to fetch OAuth URL: {:?}", err);
            Err("Failed to fetch OAuth URL from the database".into())
        }
    }
}
