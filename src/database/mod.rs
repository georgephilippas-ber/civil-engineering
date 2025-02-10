use sqlx::{Error, Pool, Postgres};

pub async fn create_postgres_pool() -> Result<Pool<Postgres>, Error> {
    Pool::connect("postgres://development:development@localhost:5432/development").await
}
