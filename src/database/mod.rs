use sqlx::{Error, PgPool};

pub async fn create_postgres_pool() -> Result<PgPool, Error> {
    PgPool::connect("postgres://development:development@localhost:5432/development").await
}
