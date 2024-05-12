use serde::{Deserialize, Serialize};
use std::fmt;

use crate::model::oauth::OAuthToken;

#[derive(Debug, Deserialize, Serialize)]
pub struct Guild {
    id: Option<String>,
    name: Option<String>,
    icon: Option<String>,
    owner: Option<bool>,
    features: Option<Vec<String>>,
    approximate_member_count: Option<u32>,
    approximate_presence_count: Option<u32>,
}

#[allow(dead_code)]
pub struct AddGuildMember {
    pub guild_id: String,
    pub user_id: String,
    pub oauth_token: OAuthToken,
    pub nickname: Option<String>,
    pub roles: Option<Vec<String>>,
    pub mute: Option<bool>,
    pub deaf: Option<bool>,
}

#[allow(dead_code)]
impl AddGuildMember {
    pub fn new(guild_id: &str, user_id: &str, oauth_token: &OAuthToken) -> Self {
        AddGuildMember {
            guild_id: guild_id.to_string(),
            user_id: user_id.to_string(),
            oauth_token: oauth_token.clone(),
            nickname: None,
            roles: None,
            mute: None,
            deaf: None,
        }
    }

    pub fn nickname(mut self, name: &str) -> Self {
        self.nickname = Some(name.to_string());
        self
    }

    pub fn roles(mut self, roles: Vec<&str>) -> Self {
        let role_strings: Vec<String> = roles.iter().map(|s| s.to_string()).collect();
        self.roles = Some(role_strings);
        self
    }

    pub fn mute(mut self, mute: bool) -> Self {
        self.mute = Some(mute);
        self
    }
    pub fn deaf(mut self, deaf: bool) -> Self {
        self.deaf = Some(deaf);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AddGuildMemberResponse {
    /// Represents a successful creation with the guild member data
    AddedToServer,
    /// Represents a successful addition where the user is already a member
    AlreadyOnServer,
}

impl fmt::Display for AddGuildMemberResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddGuildMemberResponse::AddedToServer => {
                write!(f, "User successfully added to the server")
            }
            AddGuildMemberResponse::AlreadyOnServer => write!(f, "User is already on the server"),
        }
    }
}
