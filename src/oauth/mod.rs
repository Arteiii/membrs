//! Module for interacting with OAuth flows.
//!
//! This module provides functionality for handling OAuth authentication flows
//! with Discord, including obtaining user data and guild memberships.
//!
//! # Example
//!
//! ```rust
//! use membrs_lib::oauth::{ClientData, OAuthClient};
//!
//!
//! #[tokio::main]
//! async fn main() {
//!     // Initialize OAuth client data with incorrect credentials
//!     let client_data = ClientData {
//!         client_id: "incorrect_client_id".to_string(), // Provide incorrect client ID
//!         client_secret: "incorrect_client_secret".to_string(), // Provide incorrect client secret
//!         redirect_uri: "https://your.redirect.uri/callback".to_string(),
//!     };
//!
//!     // Assume `auth_code` is obtained through OAuth authentication flow
//!
//!     // Initialize OAuth client
//!     // Check if OAuth client initialization resulted in an error
//!     match OAuthClient::new(&client_data, "auth_code").await {
//!         Ok(client) => {
//!             // Retrieve user data
//!             let user_data = client.get_user_data().await.unwrap();
//!             println!("User Data: {:?}", user_data);
//!         }
//!         Err(err) => {
//!             if err.to_string().contains("status code: 401 Unauthorized") {
//!                 // This is the expected error, you can handle it or print a message
//!                 println!("Error: Unauthorized access, check your credentials.");
//!                 # assert!(true);
//!                 # return;
//!             }
//!         }
//!     };
//! }
//! ```

use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::api;
use crate::model::guild::Guild;
pub use crate::model::oauth::{OAuthError, OAuthToken, OAuthTokenResponse};
use crate::model::user::UserData;
use crate::oauth::requests::{authenticate, parse_response, send_request};

mod requests;

pub mod url;

/// Struct representing client data required for OAuth operations
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ClientData {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

/// Struct representing an OAuth client
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct OAuthClient {
    client_data: ClientData,
    api: api::DiscordAPi,
    token: OAuthToken,
}

impl OAuthClient {
    /// Creates a new OAuthClient instance.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to client data required for authentication.
    /// * `auth_code` - The authorization code obtained during OAuth flow.
    ///
    /// # Returns
    ///
    /// A Result containing the OAuthClient instance if successful, or an OAuthError if authentication fails
    pub async fn new(data: &ClientData, auth_code: &str) -> Result<OAuthClient, OAuthError> {
        let api = api::DiscordAPi::new("https://discord.com/api", api::DiscordApiVersion::V10);

        info!("API Endpoint: {}", api.build_url());

        debug!("Authenticating with OAuth");
        let token = authenticate(data.clone(), auth_code).await?;

        info!("Authentication successful");

        Ok(OAuthClient {
            client_data: data.clone(),
            api,
            token: token.into(),
        })
    }

    /// Retrieves user data using the OAuth access token.
    ///
    /// # Returns
    ///
    /// A Result containing the user data if successful, or an OAuthError if the request fails
    pub async fn get_user_data(&self) -> Result<UserData, OAuthError> {
        let response = send_request(
            &self.api.append_path("/users/@me"),
            &self.token.access_token,
        )
        .await?;
        parse_response(response).await
    }

    /// Retrieves the user's guild memberships using the OAuth access token.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of Guilds if successful, or an OAuthError if the request fails
    pub async fn get_user_guilds(&self) -> Result<Vec<Guild>, OAuthError> {
        let response = send_request(
            &self.api.append_path("/users/@me/guilds"),
            &self.token.access_token,
        )
        .await?;
        parse_response(response).await
    }

    /// Retrieves the OAuth access token.
    ///
    /// # Returns
    ///
    /// The OAuthToken instance containing the access token
    pub async fn get_token(&self) -> OAuthToken {
        self.token.clone()
    }
}
