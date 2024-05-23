use sqlx::{FromRow, PgPool};

#[allow(dead_code)]
#[derive(FromRow, Debug)]
pub struct ApplicationData {
    pub id: i32,
    pub app_name: String,
    pub backend_url: Option<String>,
    pub frontend_url: Option<String>,
    pub bot_token: Option<String>,
    pub oauth_url: Option<String>,
    pub client_id: Option<String>,
    pub redirect_uri: Option<String>,
    pub client_secret: Option<String>,
    pub guild_id: Option<String>,
}

impl ApplicationData {
    #[allow(dead_code)]
    pub async fn create_application_data_table(pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
                CREATE TABLE IF NOT EXISTS application_data (
                    id SERIAL PRIMARY KEY,
                    app_name VARCHAR(255) NOT NULL UNIQUE,
                    backend_url VARCHAR(255),
                    frontend_url VARCHAR(255),
                    bot_token VARCHAR(255),
                    oauth_url VARCHAR(255),
                    client_id VARCHAR(255),
                    redirect_uri VARCHAR(255),
                    client_secret VARCHAR(255),
                    guild_id VARCHAR(255)
                )
            "#,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    #[inline]
    async fn execute_insert_query(
        pool: &PgPool,
        data: &ApplicationData,
        soft: bool,
    ) -> Result<(), sqlx::Error> {
        let query = if soft {
            r#"
            INSERT INTO application_data (
                app_name, backend_url, frontend_url, bot_token,
                oauth_url, client_id, redirect_uri, client_secret, guild_id
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            )
            ON CONFLICT (app_name) DO UPDATE SET
                backend_url = COALESCE(EXCLUDED.backend_url, application_data.backend_url),
                frontend_url = COALESCE(EXCLUDED.frontend_url, application_data.frontend_url),
                bot_token = COALESCE(EXCLUDED.bot_token, application_data.bot_token),
                oauth_url = COALESCE(EXCLUDED.oauth_url, application_data.oauth_url),
                client_id = COALESCE(EXCLUDED.client_id, application_data.client_id),
                redirect_uri = COALESCE(EXCLUDED.redirect_uri, application_data.redirect_uri),
                client_secret = COALESCE(EXCLUDED.client_secret, application_data.client_secret),
                guild_id = COALESCE(EXCLUDED.guild_id, application_data.guild_id)
            "#
        } else {
            r#"
            INSERT INTO application_data (
                app_name, backend_url, frontend_url, bot_token,
                oauth_url, client_id, redirect_uri, client_secret, guild_id
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            )
            ON CONFLICT (app_name)
            DO UPDATE SET
                backend_url = EXCLUDED.backend_url,
                frontend_url = EXCLUDED.frontend_url,
                bot_token = EXCLUDED.bot_token,
                oauth_url = EXCLUDED.oauth_url,
                client_id = EXCLUDED.client_id,
                redirect_uri = EXCLUDED.redirect_uri,
                client_secret = EXCLUDED.client_secret,
                guild_id = EXCLUDED.guild_id
            "#
        };

        sqlx::query(query)
            .bind(&data.app_name)
            .bind(&data.backend_url)
            .bind(&data.frontend_url)
            .bind(&data.bot_token)
            .bind(&data.oauth_url)
            .bind(&data.client_id)
            .bind(&data.redirect_uri)
            .bind(&data.client_secret)
            .bind(&data.guild_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn insert_application_data(
        pool: &PgPool,
        data: &ApplicationData,
    ) -> Result<(), sqlx::Error> {
        Self::execute_insert_query(pool, data, false).await
    }

    pub async fn soft_insert_application_data(
        pool: &PgPool,
        data: &ApplicationData,
    ) -> Result<(), sqlx::Error> {
        Self::execute_insert_query(pool, data, true).await
    }
    pub async fn get_application_data(pool: &PgPool) -> Result<ApplicationData, sqlx::Error> {
        let data = sqlx::query_as::<_, ApplicationData>(
            r#"
                SELECT * FROM application_data
            "#,
        )
        .fetch_one(pool)
        .await?;

        Ok(data)
    }

    #[allow(dead_code)]
    pub async fn get_bot_token(pool: &PgPool) -> Result<Option<String>, sqlx::Error> {
        let data = sqlx::query_scalar(
            r#"
                SELECT bot_token FROM application_data
            "#,
        )
        .fetch_optional(pool)
        .await?;
        Ok(data)
    }

    pub async fn get_oauth_url(pool: &PgPool) -> Result<Option<String>, sqlx::Error> {
        let data = sqlx::query_scalar(
            r#"
                SELECT oauth_url FROM application_data
            "#,
        )
        .fetch_one(pool)
        .await?;
        Ok(data)
    }
}
