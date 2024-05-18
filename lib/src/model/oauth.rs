use std::fmt;

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OAuthTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_at: DateTime<Utc>,
    pub refresh_token: Option<String>,
}

impl From<OAuthTokenResponse> for OAuthToken {
    fn from(response: OAuthTokenResponse) -> Self {
        // Perform the conversion here
        OAuthToken {
            access_token: response.access_token,
            token_type: response.token_type,
            expires_at: Utc::now() + Duration::seconds(response.expires_in.into()),
            refresh_token: response.refresh_token,
        }
    }
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub enum OAuthError {
    RequestError(String),
    ParseError(String),
}

impl fmt::Display for OAuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OAuthError::RequestError(msg) => write!(f, "Request Error: {}", msg),
            OAuthError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizationInfo {
    pub application: ApplicationInfo,
    pub scopes: Vec<String>,
    pub expires: String,
    pub user: UserInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationInfo {
    id: String,
    name: String,
    icon: Option<String>,
    description: String,
    hook: bool,
    bot_public: bool,
    bot_require_code_grant: bool,
    verify_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    id: String,
    username: String,
    avatar: Option<String>,
    discriminator: String,
    global_name: Option<String>,
    public_flags: u64,
}
