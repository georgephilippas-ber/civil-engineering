use sqlx::{Error, Pool, Postgres};

async fn create_postgres_pool() -> Result<Pool<Postgres>, Error> {
    Pool::connect("postgres://postgres:postgres@localhost:5432/postgres").await
}

#[tokio::main]
async fn main() {
    let c = create_postgres_pool().await.unwrap();

    c.close()
}
