use std::io::{Error, ErrorKind};

use reqwest::Client;
use serde_json::json;
use tracing::error;

use crate::api;
pub use crate::model::guild::{AddGuildMember, AddGuildMemberResponse};

#[derive(Debug, Clone)]
pub struct Bot {
    api: api::DiscordAPi,
    client: Client,
    token: String,
}

impl Bot {
    pub fn new(token: &str) -> Self {
        Bot {
            api: api::DiscordAPi::new("https://discord.com/api", api::DiscordApiVersion::V10),
            client: Client::new(),
            token: token.to_string(),
        }
    }

    pub async fn add_guild_member(
        &self,
        guild_member: AddGuildMember,
    ) -> Result<AddGuildMemberResponse, Error> {
        let url = self.api.append_path(&format!(
            "/guilds/{}/members/{}",
            guild_member.guild_id, guild_member.user_id
        ));

        let mut json_body = json!({});
        json_body["access_token"] = json!(guild_member.oauth_token.access_token);

        if let Some(nickname) = guild_member.nickname {
            json_body["nick"] = json!(nickname);
        }
        if let Some(roles) = guild_member.roles {
            json_body["roles"] = json!(roles);
        }
        if let Some(mute) = guild_member.mute {
            json_body["mute"] = json!(mute);
        }
        if let Some(deaf) = guild_member.deaf {
            json_body["deaf"] = json!(deaf);
        }

        let response = match self
            .client
            .put(&url)
            .header("Authorization", format!("Bot {}", self.token))
            .json(&json_body)
            .send()
            .await
        {
            Ok(res) => res,
            Err(err) => {
                error!("Failed to send request: {:?}", err);
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Failed to send request: {:?}", err),
                ));
            }
        };

        match response.status().as_u16() {
            201 => Ok(AddGuildMemberResponse::AddedToServer),
            204 => Ok(AddGuildMemberResponse::AlreadyOnServer),
            _ => Err(Error::new(
                ErrorKind::Other,
                format!("Failed to add guild member: {}", response.status()),
            )),
        }
    }
}
