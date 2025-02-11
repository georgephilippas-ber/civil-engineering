use std::collections::HashMap;
use crate::database::create_postgres_pool;
use crate::facets::surveying::Surveying;

mod database;
mod facets;

fn solve(input: Vec<i32>) -> Vec<i32> {
    let g = input
        .iter()
        .enumerate()
        .map(|(i, x)| (x, i as i32));

    g.keys()
}

#[tokio::main]
async fn main() {
    let database_pool = create_postgres_pool().await.unwrap();

    let su: Surveying = Surveying::new(&database_pool);

    su.create_survey("survey3", "here", 123, "Berlin")
        .await
        .unwrap();

    database_pool.close().await
}
