use chrono::{DateTime, Utc};
use sqlx::{Error, FromRow, PgPool};

#[allow(dead_code)]
#[derive(FromRow, Debug, Clone)]
pub struct UserData {
    pub id: i32,
    pub discord_id: Option<String>,
    pub username: Option<String>,
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub banner: Option<String>,
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub refresh_token: Option<String>,
}

impl UserData {
    #[allow(dead_code)]
    pub async fn create_user_data_table(pool: &PgPool) -> Result<(), Error> {
        let create_table_query = r#"
            CREATE TABLE IF NOT EXISTS user_data (
                id SERIAL PRIMARY KEY,
                discord_id VARCHAR(255) UNIQUE,
                username VARCHAR(255),
                avatar VARCHAR(255),
                email VARCHAR(255),
                banner VARCHAR(255),
                access_token VARCHAR(255),
                token_type VARCHAR(255),
                expires_at TIMESTAMPTZ,
                refresh_token VARCHAR(255)
            )
        "#;

        sqlx::query(create_table_query).execute(pool).await?;
        Ok(())
    }

    pub async fn insert_user_data(&self, pool: &PgPool) -> Result<(), Error> {
        let insert_query = r#"
            INSERT INTO user_data (
                discord_id, username, avatar, email, banner, access_token, token_type, expires_at, refresh_token
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            )
            ON CONFLICT (discord_id)
            DO UPDATE SET
                username = EXCLUDED.username,
                avatar = EXCLUDED.avatar,
                email = EXCLUDED.email,
                banner = EXCLUDED.banner,
                access_token = EXCLUDED.access_token,
                token_type = EXCLUDED.token_type,
                expires_at = EXCLUDED.expires_at,
                refresh_token = EXCLUDED.refresh_token
        "#;

        match sqlx::query(insert_query)
            .bind(&self.discord_id)
            .bind(&self.username)
            .bind(&self.avatar)
            .bind(&self.email)
            .bind(&self.banner)
            .bind(&self.access_token)
            .bind(&self.token_type)
            .bind(self.expires_at)
            .bind(&self.refresh_token)
            .execute(pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    #[allow(dead_code)]
    pub async fn get_user_data_by_id(
        pool: &PgPool,
        id: i32,
    ) -> Result<Option<UserData>, sqlx::Error> {
        let data = sqlx::query_as::<_, UserData>(
            r#"
                SELECT * FROM user_data WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(data)
    }

    pub async fn get_users(pool: &PgPool, num: i64) -> Result<Vec<UserData>, sqlx::Error> {
        let data = sqlx::query_as::<_, UserData>(
            r#"
            SELECT * FROM user_data ORDER BY id LIMIT $1
        "#,
        )
        .bind(num)
        .fetch_all(pool)
        .await?;
        Ok(data)
    }

    pub async fn count_users(pool: &PgPool) -> Result<i64, Error> {
        let count_query = r#"
            SELECT COUNT(*) FROM user_data
        "#;

        let count: (i64,) = sqlx::query_as(count_query).fetch_one(pool).await?;

        Ok(count.0)
    }
}
