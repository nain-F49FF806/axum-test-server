// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

#[allow(dead_code)]
pub async fn setup_postgresql_db() -> Pool<Postgres> {
    let _ = dotenvy::dotenv().expect(".env file not found!");
    let database_url: String = 
        env::var("POSTGRES_URL").expect("Environment variable POSTGRES_URL not found in .env!");
    
    PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to database!")
}


#[cfg(test)]
mod tests {
    use super::setup_postgresql_db;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    pub async fn test_query() {
        let first_todo_title = "Learn SQLx";
        let pool = setup_postgresql_db().await;
    
        sqlx::query("INSERT INTO todos (title) VALUES ($1)")
            .bind(first_todo_title)
            .execute(&pool)
            .await
            .unwrap();
    
    }
}