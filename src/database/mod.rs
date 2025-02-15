use sqlx::{Error, PgPool};

static USERNAME: &str = "postgres";
static PASSWORD: &str = "development";

fn postgres_connection_string() -> String {
    format!(
        "postgres://{}:{}@localhost:5432/development",
        USERNAME, PASSWORD
    )
}

pub async fn create_postgres_pool() -> Result<PgPool, Error> {
    PgPool::connect(postgres_connection_string().as_str()).await
}
