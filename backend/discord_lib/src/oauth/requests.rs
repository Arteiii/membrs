use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use tracing::trace;

use crate::oauth::{ClientData, OAuthError, OAuthToken, OAuthTokenResponse};

pub async fn send_request(url: &str, token: &OAuthToken) -> Result<Response, OAuthError> {
    trace!("Sending GET request to URL: {}", url);
    let response = Client::new()
        .get(url)
        .header("Authorization", format!("Bearer {}", token.access_token))
        .send()
        .await
        .map_err(|e| {
            trace!("Failed to send request: {}", e);
            OAuthError::RequestError(format!("Failed to send request: {}", e))
        })?;

    if response.status().is_success() {
        trace!("Request succeeded with status: {}", response.status());
        Ok(response)
    } else {
        let status_code = response.status();
        trace!("Request failed with status code: {}", status_code);
        Err(OAuthError::RequestError(format!(
            "Request failed with status code: {}",
            status_code
        )))
    }
}

pub async fn get_refresh_token(
    client_data: &ClientData,
    refresh_token: &str,
) -> Result<OAuthToken, OAuthError> {
    trace!("Preparing data for refresh token request...");
    let mut data = std::collections::HashMap::new();
    data.insert("grant_type", "refresh_token");
    data.insert("refresh_token", refresh_token);
    data.insert("client_id", &client_data.client_id);
    data.insert("client_secret", &client_data.client_secret);

    trace!("Sending refresh token request to Discord API...");
    let client = Client::new();
    let token_response = client
        .post("https://discord.com/api/oauth2/token")
        .form(&data)
        .send()
        .await
        .map_err(|e| {
            trace!("Failed to send refresh request: {}", e);
            OAuthError::RequestError(format!("Failed to send refresh request: {}", e))
        })?;

    if token_response.status().is_success() {
        trace!("Refresh token request succeeded, parsing response...");
        parse_response(token_response).await
    } else {
        let status_code = token_response.status();
        trace!("Token request failed with status code: {}", status_code);
        Err(OAuthError::RequestError(format!(
            "Token request failed with status code: {}",
            status_code
        )))
    }
}

pub async fn parse_response<T: DeserializeOwned>(response: Response) -> Result<T, OAuthError> {
    trace!("Parsing token response...");
    let body = response
        .text()
        .await
        .map_err(|e| OAuthError::ParseError(format!("Failed to read response body: {}", e)))?;

    trace!("Token parsed successfully: {:?}", body);

    serde_json::from_str::<T>(&body)
        .map_err(|e| OAuthError::ParseError(format!("Failed to parse response: {}", e)))
}

pub async fn authenticate(
    client_data: ClientData,
    code: &str,
) -> Result<OAuthTokenResponse, OAuthError> {
    let mut data = std::collections::HashMap::new();
    data.insert("grant_type", "authorization_code");
    data.insert("code", code);
    data.insert("redirect_uri", &client_data.redirect_uri);

    trace!("Sending authentication request to Discord API...");
    let client = Client::new();
    let token_response = client
        .post("https://discord.com/api/oauth2/token")
        .form(&data)
        .basic_auth(client_data.client_id, Some(client_data.client_secret))
        .send()
        .await
        .map_err(|e| OAuthError::RequestError(format!("Failed to send token request: {}", e)))?;

    if token_response.status().is_success() {
        trace!("Authentication request succeeded, parsing response...");
        parse_response(token_response).await
    } else {
        let status_code = token_response.status();
        trace!("Token request failed with status code: {}", status_code);
        Err(OAuthError::RequestError(format!(
            "Token request failed with status code: {}",
            status_code
        )))
    }
}
