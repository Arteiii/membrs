use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;
use axum::response::Html;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, error};

use membrs_lib::bot::AddGuildMember;
use membrs_lib::model::user::UserData;
use membrs_lib::oauth;
use membrs_lib::oauth::OAuthToken;

use crate::app_state::AppState;

#[derive(Deserialize, Serialize)]
struct User {
    data: UserData,
    token: OAuthToken,
}

pub(crate) async fn oauth_callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Html<String>, Json<serde_json::Value>> {
    let authorization_code = params.get("code").ok_or_else(|| {
        error!("Authorization code not found in query parameters");
        json!({"error": "Authorization code not found"})
    })?;

    let client = oauth::OAuthClient::new(&state.data, authorization_code);

    match client.await {
        Ok(client) => {
            // Assuming client.get_user_data() returns UserData struct
            let user_data = match client.get_user_data().await {
                Ok(user_data) => user_data,
                Err(err) => {
                    // Handle authentication error
                    error!("OAuth error: {:?}", err);
                    return Err(Json(json!(err)));
                }
            };

            debug!("{:?}", user_data);

            match state
                .bot
                .add_guild_member(AddGuildMember::new(
                    "1176889409325514772",
                    &user_data.id,
                    &client.get_token().await,
                ))
                .await
            {
                Ok(res) => {
                    debug!("Add Guild Member Response: {:?}", res);
                    // Return HTML response
                    let username = user_data.username.unwrap_or_else(|| "Unknown".to_string());
                    let avatar = user_data
                        .avatar
                        .map(|avatar| {
                            format!(
                                r#"<img src="https://cdn.discordapp.com/avatars/{}/{}" alt="Profile Picture">"#,
                                user_data.id, avatar
                            )
                        })
                        .unwrap_or_else(|| "".to_string());
                    let html_response = format!(
                        r#"
                        <html>
                            <head>
                                <title>Authentication Success</title>
                            </head>
                            <body>
                                <h1>Successfully Authenticated!</h1>
                                <p>Welcome, {}!</p>
                                <p>Discord Server: {}!</p>
                                {}
                            </body>
                        </html>
                        "#,
                        username, res, avatar
                    );
                    Ok(Html(html_response))
                }
                Err(err) => {
                    // Handle error
                    let msg = format!("Failed to add guild member: {:?}", err);
                    error!(msg);
                    Err(Json(json!({ "error": msg  })))
                }
            }
        }
        Err(err) => {
            // Handle OAuthClient::new error
            error!("OAuth error: {:?}", err);
            Err(Json(json!(err)))
        }
    }
}


pub(crate) async fn oauth_url(
    State(state): State<Arc<AppState>>,
) -> String {
    state.oauth_url.to_string()
}
