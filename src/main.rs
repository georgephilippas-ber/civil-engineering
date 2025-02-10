use crate::database::create_postgres_pool;

mod database;
mod facets;

#[tokio::main]
async fn main() {
    let database_pool = create_postgres_pool().await.unwrap();

    database_pool.close().await
}
