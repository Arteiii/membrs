use tracing::trace;
use urlencoding::encode;

/// A builder for constructing Discord OAuth2 authorization URLs.
pub struct DiscordOAuthUrlBuilder {
    client_id: String,
    redirect_uri: String,
    scopes: Vec<String>,
}

impl DiscordOAuthUrlBuilder {
    /// Constructs a new `DiscordOAuthUrlBuilder`.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The client ID provided by Discord for your application.
    ///                You can get this by creating a new application on the
    ///                Discord Developer Portal: https://discord.com/developers/applications
    /// * `redirect_uri` - The URI where Discord will redirect users after authorization.
    ///                   This should match the URI you specified when setting up your
    ///                   application on the Discord Developer Portal.
    ///
    /// # Example
    ///
    /// ```
    /// use discord_lib::oauth::url::DiscordOAuthUrlBuilder;
    ///
    /// let builder = DiscordOAuthUrlBuilder::new("your_client_id", "https://example.com/oauth");
    /// ```
    pub fn new(client_id: &str, redirect_uri: &str) -> Self {
        DiscordOAuthUrlBuilder {
            client_id: client_id.to_string(),
            redirect_uri: redirect_uri.to_string(),
            scopes: Vec::new(),
        }
    }

    /// allows [/users/@me](https://discord.com/developers/docs/resources/user#get-current-user)
    /// without email
    pub fn identify(mut self) -> Self {
        self.add_scope("identify");
        self
    }

    /// allows [/users/@me/connections](https://discord.com/developers/docs/resources/user#get-current-user-connections)
    /// to return linked third-party accounts
    pub fn connections(mut self) -> Self {
        self.add_scope("connections");
        self
    }

    /// allows
    /// (/users/@me/guilds/{guild.id}/member)[https://discord.com/developers/docs/resources/user#get-current-user-guild-member]
    /// to return a user's member information in a guild
    pub fn guilds_members_read(mut self) -> Self {
        self.add_scope("guilds.members.read");
        self
    }

    /// for local rpc server access, this allows you to receive notifications pushed out to the user -
    /// requires Discord approval
    pub fn rpc_notifications_read(mut self) -> Self {
        self.add_scope("rpc.notifications.read");
        self
    }

    pub fn rpc_video_read(mut self) -> Self {
        self.add_scope("rpc.video.read");
        self
    }

    pub fn rpc_screenshare_write(mut self) -> Self {
        self.add_scope("rpc.screenshare.write");
        self
    }

    /// this generates a webhook returned in the oauth token response for authorization code grants
    pub fn webhook_incoming(mut self) -> Self {
        self.add_scope("webhook.incoming");
        self
    }

    /// allows your app to read build data for a user's applications
    pub fn applications_builds_read(mut self) -> Self {
        self.add_scope("applications.builds.read");
        self
    }

    /// allows your app to read entitlements for a user's applications
    pub fn applications_entitlements(mut self) -> Self {
        self.add_scope("applications.entitlements");
        self
    }

    /// allows your app to know a user's friends and implicit relationships - requires Discord approval
    pub fn relationships_read(mut self) -> Self {
        self.add_scope("relationships.read");
        self
    }

    /// allows your app to see information about the user's DMs and group DMs - requires Discord approval
    pub fn dm_channels_read(mut self) -> Self {
        self.add_scope("dm_channels.read");
        self
    }

    pub fn presences_write(mut self) -> Self {
        self.add_scope("presences.write");
        self
    }

    /// allows your app to update its
    /// (commands)[https://discord.com/developers/docs/interactions/application-commands]
    /// using a Bearer token -
    /// (client credentials grant)[https://discord.com/developers/docs/topics/oauth2#client-credentials-grant] only
    pub fn applications_commands_permissions_update(mut self) -> Self {
        self.add_scope("applications.commands.permissions.update");
        self
    }

    pub fn openid(mut self) -> Self {
        self.add_scope("openid");
        self
    }

    /// allows
    /// (/users/@me/guilds)[https://discord.com/developers/docs/resources/user#get-current-user-guilds]
    /// to return basic information about all of a user's guilds
    pub fn guilds(mut self) -> Self {
        self.add_scope("guilds");
        self
    }

    /// allows your app to (join users to a group dm)[https://discord.com/developers/docs/resources/channel#group-dm-add-recipient]
    pub fn gdm_join(mut self) -> Self {
        self.add_scope("gdm.join");
        self
    }

    /// for local rpc server access,
    /// this allows you to read a user's voice settings and listen for voice events -
    /// requires Discord approval
    pub fn rpc_voice_read(mut self) -> Self {
        self.add_scope("rpc.voice.read");
        self
    }

    pub fn rpc_video_write(mut self) -> Self {
        self.add_scope("rpc.video.write");
        self
    }

    /// for local rpc server access, this allows you to update a user's activity -
    /// requires Discord approval
    pub fn rpc_activities_write(mut self) -> Self {
        self.add_scope("rpc.activities.write");
        self
    }

    /// for local rpc server api access, this allows you to read messages from all client channels
    /// (otherwise restricted to channels/guilds your app creates)
    pub fn messages_read(mut self) -> Self {
        self.add_scope("messages.read");
        self
    }

    /// allows your app to add
    /// (commands)[https://discord.com/developers/docs/interactions/application-commands] to a guild -
    /// included by default with the `bot` scope
    pub fn applications_commands(mut self) -> Self {
        self.add_scope("applications.commands");
        self
    }

    pub fn activities_read(mut self) -> Self {
        self.add_scope("activities.read");
        self
    }

    pub fn relationships_write(mut self) -> Self {
        self.add_scope("relationships.write");
        self
    }

    pub fn role_connections_write(mut self) -> Self {
        self.add_scope("role_connections.write");
        self
    }

    pub fn dm_channels_messages_read(mut self) -> Self {
        self.add_scope("dm_channels.messages.read");
        self
    }

    pub fn email(mut self) -> Self {
        self.add_scope("email");
        self
    }

    pub fn guilds_join(mut self) -> Self {
        self.add_scope("guilds.join");
        self
    }

    pub fn rpc(mut self) -> Self {
        self.add_scope("rpc");
        self
    }

    pub fn rpc_voice_write(mut self) -> Self {
        self.add_scope("rpc.voice.write");
        self
    }

    pub fn rpc_screenshare_read(mut self) -> Self {
        self.add_scope("rpc.screenshare.read");
        self
    }

    /// NOT IMPLEMENTED
    pub fn bot(self) -> Self {
        todo!("bot url not implemented currently")
    }

    pub fn applications_builds_upload(mut self) -> Self {
        self.add_scope("applications.builds.upload");
        self
    }

    pub fn applications_store_update_upload(mut self) -> Self {
        self.add_scope("applications.store.update");
        self
    }

    pub fn activities_write(mut self) -> Self {
        self.add_scope("activities.write");
        self
    }

    pub fn voice(mut self) -> Self {
        self.add_scope("voice");
        self
    }

    pub fn presences_read(mut self) -> Self {
        self.add_scope("presences.read");
        self
    }

    pub fn dm_channels_messages_write(mut self) -> Self {
        self.add_scope("dm_channels.messages.write");
        self
    }

    /// Builds the OAuth2 authorization URL.
    ///
    /// # Returns
    ///
    /// A string containing the complete authorization URL.
    ///
    /// # Example
    ///
    /// ```
    /// use discord_lib::oauth::url::DiscordOAuthUrlBuilder;
    ///
    ///  let oauth_url = DiscordOAuthUrlBuilder::new("example_id", "example_redirect_url")
    ///     .identify()
    ///     .email()
    ///     .build();
    /// ```
    pub fn build(&self) -> String {
        let redirect_uri_encoded = encode(&self.redirect_uri);

        let mut url = format!(
            "https://discord.com/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code",
            self.client_id, redirect_uri_encoded
        );

        if !self.scopes.is_empty() {
            let scopes_param = self.scopes.join("+");
            url.push_str("&scope=");
            url.push_str(&scopes_param);
        }

        trace!("Built OAuth URL: {}", url);

        url
    }

    fn add_scope(&mut self, scope: &str) {
        self.scopes.push(scope.to_string());
        trace!("Added scope: {}", scope);
    }
}
