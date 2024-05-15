use std::env;

use dotenv::dotenv;
use tracing::info;

use membrs_lib::bot::Bot;
use membrs_lib::oauth::ClientData;
use membrs_lib::oauth::url::DiscordOAuthUrlBuilder;

// helper struct to share data using states
#[allow(dead_code)]
pub struct AppState {
    pub data: ClientData,
    pub oauth_url: String,
    pub bot: Bot,
    pub addr: String,
}

impl Default for AppState {
    fn default() -> Self {
        dotenv().ok();

        let data = ClientData {
            client_id: env::var("CLIENT_ID").expect("CLIENT_ID not found in .env"),
            client_secret: env::var("CLIENT_SECRET").expect("CLIENT_SECRET not found in .env"),
            redirect_uri: env::var("REDIRECT_URI").expect("REDIRECT_URI not found in .env"),
        };

        let oauth_url = DiscordOAuthUrlBuilder::new(&data.client_id, &data.redirect_uri)
            .identify()
            .email()
            .guilds()
            .guilds_members_read()
            .guilds_join()
            .build();

        info!("Discord OAuth URL: {}", &oauth_url);

        let bot_token = env::var("BOT_TOKEN").expect("BOT_TOKEN not found in .env");
        let bot = Bot::new(&bot_token);

        let addr = env::var("ADDR").expect("ADDR not found in .env");
        info!("Server listening on {}", &addr);

        AppState {
            data,
            oauth_url,
            bot,
            addr,
        }
    }
}

impl AppState {
    #[allow(dead_code)]
    pub async fn new(data: &ClientData, oauth_url: &str, bot: &Bot, addr: &str) -> Self {
        AppState {
            data: data.clone(),
            oauth_url: oauth_url.to_string(),
            bot: bot.clone(),
            addr: addr.to_string(),
        }
    }
}
