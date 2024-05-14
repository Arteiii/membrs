use reqwest::{Client, Response};
use serde::de::DeserializeOwned;

use crate::oauth::{ClientData, OAuthError, OAuthTokenResponse};

pub async fn send_request(url: &str, token: &str) -> Result<Response, OAuthError> {
    let client = Client::new();
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| OAuthError::RequestError(format!("Failed to send request: {}", e)))?;

    if response.status().is_success() {
        Ok(response)
    } else {
        let status_code = response.status();
        Err(OAuthError::RequestError(format!(
            "Request failed with status code: {}",
            status_code
        )))
    }
}

pub async fn parse_response<T: DeserializeOwned>(response: Response) -> Result<T, OAuthError> {
    response
        .json::<T>()
        .await
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

    let client = Client::new();
    let token_response = client
        .post("https://discord.com/api/oauth2/token")
        .form(&data)
        .basic_auth(client_data.client_id, Some(client_data.client_secret))
        .send()
        .await
        .map_err(|e| OAuthError::RequestError(format!("Failed to send token request: {}", e)))?;

    if token_response.status().is_success() {
        parse_response(token_response).await
    } else {
        let status_code = token_response.status();
        Err(OAuthError::RequestError(format!(
            "Token request failed with status code: {}",
            status_code
        )))
    }
}
