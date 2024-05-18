use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};

#[derive(FromRow, Debug)]
pub struct UserData {
    pub id: i32,
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
        pool: &PgPool,
        access_token: String,
        token_type: String,
        expires_at: DateTime<Utc>,
        refresh_token: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO user_data (
                access_token, token_type, expires_at, refresh_token
            ) VALUES (
                $1, $2, $3, $4
            )
            ON CONFLICT (id)
            DO UPDATE SET
                access_token = EXCLUDED.access_token,
                token_type = EXCLUDED.token_type,
                expires_at = EXCLUDED.expires_at,
                refresh_token = EXCLUDED.refresh_token
            "#,
            access_token,
            token_type,
            expires_at,
            refresh_token
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
}
