use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};

#[derive(FromRow, Debug)]
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
    pub async fn create_user_data_table(pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
                CREATE TABLE IF NOT EXISTS user_data (
                    id SERIAL PRIMARY KEY,
                    discord_id VARCHAR(255),
                    username VARCHAR(255),
                    avatar VARCHAR(255),
                    email VARCHAR(255),
                    banner VARCHAR(255),
                    access_token VARCHAR(255),
                    token_type VARCHAR(255),
                    expires_at TIMESTAMPTZ,
                    refresh_token VARCHAR(255)
                )
            "#,
        )
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn insert_user_data(
        &self, pool: &PgPool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO user_data (
                discord_id, username, avatar, email, banner, access_token, token_type, expires_at, refresh_token
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            )
            ON CONFLICT (id)
            DO UPDATE SET
                discord_id = EXCLUDED.discord_id,
                username = EXCLUDED.username,
                avatar = EXCLUDED.avatar,
                email = EXCLUDED.email,
                banner = EXCLUDED.banner,
                access_token = EXCLUDED.access_token,
                token_type = EXCLUDED.token_type,
                expires_at = EXCLUDED.expires_at,
                refresh_token = EXCLUDED.refresh_token
            "#,
            self.discord_id,
            self.username,
            self.avatar,
            self.email,
            self.banner,
            self.access_token,
            self.token_type,
            self.expires_at,
            self.refresh_token
        )
            .execute(pool)
            .await?;
        Ok(())
    }

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

    pub async fn get_users(pool: &PgPool, num: i32) -> Result<Vec<UserData>, sqlx::Error> {
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
}
