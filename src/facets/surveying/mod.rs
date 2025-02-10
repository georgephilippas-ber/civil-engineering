use sqlx::{Error, PgPool};

pub struct Surveying {
    pg_pool: PgPool,
}

impl Surveying {
    pub fn new(pg_pool: &PgPool) -> Self {
        Self {
            pg_pool: pg_pool.clone(),
        }
    }

    pub async fn create_survey(
        &self,
        name: &str,
        street: &str,
        number: i32,
        city: &str,
    ) -> Result<i32, Error> {
        let address_id_: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO address (street, number, city) VALUES ($1, $2, $3) RETURNING id
        "#,
        )
        .bind(street.to_string())
        .bind(number)
        .bind(city.to_string())
        .fetch_one(&self.pg_pool)
        .await?;

        let survey_id_: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO survey (name, address_id) VALUES ($1, $2) RETURNING id
        "#,
        )
        .bind(name.to_string())
        .bind(address_id_)
        .fetch_one(&self.pg_pool)
        .await?;

        Ok(survey_id_)
    }
}
