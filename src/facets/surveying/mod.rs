use sqlx::PgPool;
use std::sync::Arc;

struct Surveying {
    pg_pool: PgPool,
}

impl Surveying {
    pub fn new(pg_pool: &PgPool) -> Self {
        Self {
            pg_pool: pg_pool.clone(),
        }
    }

    pub async fn create_survey(&self, name: &str, street: &str, number: i32, city: &str) {
        let address_id_ = sqlx::query_scalar(
            r#"
            insert into address (street, number, city) values ($1, $2, $3) returning id
        "#,
        )
        .bind(street.to_string())
        .bind(number)
        .bind(city.to_string())
        .fetch_one(self.pg_pool.clone())
        .await;


    }
}
