use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::error;

#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationData {
    pub frontend_url: Option<String>,
    pub backend_url: Option<String>,
    pub oauth_url: Option<String>,
    pub guild_id: Option<String>,
}

#[derive(Debug)]
pub struct ClientData {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub redirect_uri: Option<String>,
}

impl From<ClientData> for membrs_lib::oauth::ClientData {
    fn from(data: ClientData) -> Self {
        membrs_lib::oauth::ClientData {
            client_id: data.client_id.unwrap_or_default(),
            client_secret: data.client_secret.unwrap_or_default(),
            redirect_uri: data.redirect_uri.unwrap_or_default(),
        }
    }
}

pub async fn fetch_client_data(pool: &PgPool) -> Result<membrs_lib::oauth::ClientData, String> {
    match sqlx::query_as!(
        ClientData,
        r#"
        SELECT client_id, client_secret, redirect_uri
        FROM application_data
        LIMIT 1
        "#
    )
    .fetch_optional(pool)
    .await
    {
        Ok(Some(data)) => Ok(data.into()),
        Ok(None) => {
            eprintln!("No client data found in the database");
            Err("No client data found in the database".into())
        }
        Err(err) => {
            eprintln!("Failed to fetch client data: {:?}", err);
            Err("Failed to fetch client data from the database".into())
        }
    }
}

pub async fn fetch_application_data(pool: &PgPool) -> Result<ApplicationData, String> {
    match sqlx::query_as!(
        ApplicationData,
        r#"
        SELECT frontend_url, backend_url, oauth_url, guild_id
        FROM application_data
        LIMIT 1
        "#
    )
    .fetch_one(pool)
    .await
    {
        Ok(data) => Ok(data),
        Err(sqlx::Error::RowNotFound) => {
            error!("No application data found in the database");
            Err("No application data found in the database".into())
        }
        Err(err) => {
            error!("Failed to fetch application data: {:?}", err);
            Err("Failed to fetch application data from the database".into())
        }
    }
}
