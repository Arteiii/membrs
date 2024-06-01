use sqlx::{FromRow, PgPool, Result};
use tracing::trace;

#[allow(dead_code)]
#[derive(FromRow, Debug)]
pub struct SuperUser {
    pub id: i32,
    pub password: Option<String>,
    pub username: Option<String>,
}

impl SuperUser {
    /// create superuser table
    pub async fn create_table(pool: &PgPool) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS superuser (
                id SERIAL PRIMARY KEY,
                password TEXT,
                username TEXT
            )
            "#,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    /// insert or update superuser
    pub async fn upsert(
        pool: &PgPool,
        password: Option<String>,
        username: Option<String>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO superuser (id, password, username)
            VALUES (1, $1, $2)
            ON CONFLICT (id)
            DO UPDATE SET
                password = EXCLUDED.password,
                username = EXCLUDED.username
            "#,
        )
        .bind(password)
        .bind(username)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// fetch superuser
    pub async fn fetch(pool: &PgPool) -> Result<Option<SuperUser>> {
        let superuser = sqlx::query_as::<_, SuperUser>(
            r#"
            SELECT * FROM superuser WHERE id = 1
            "#,
        )
        .fetch_optional(pool)
        .await?;
        Ok(superuser)
    }

    /// Function to check if superuser entry exists, and create if not
    pub async fn check_and_create_superuser(pool: &PgPool) -> Result<()> {
        // Check if superuser entry already exists
        if let Some(superuser) = SuperUser::fetch(pool).await? {
            trace!(
                "Superuser already exists with username: {:?}",
                superuser.username
            );
            return Ok(());
        }

        let password: String = "admin".to_string();
        let username: String = "admin".to_string();

        SuperUser::upsert(pool, Some(password.clone()), Some(username.clone())).await?;

        trace!(
            "Created superuser with username: {:?} and password: {:?}",
            username,
            password
        );

        Ok(())
    }
}
