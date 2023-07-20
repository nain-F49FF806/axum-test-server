// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

use sqlx::{postgres::PgPoolOptions, PgPool};

#[allow(dead_code)]
pub async fn get_db_pool() -> PgPool {
    let _ = dotenvy::dotenv().expect(".env file not found! Need .env file with POSTGRES_URL variable defined");
    let database_url: String = 
        dotenvy::var("POSTGRES_URL").expect("Environment variable POSTGRES_URL not found in .env!");
    
    PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to database!")
}


#[cfg(test)]
mod tests {
    use super::get_db_pool;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    #[ignore]
    pub async fn test_query() {
        let first_todo_title = "Learn SQLx";
        let pool = get_db_pool().await;
    
        sqlx::query("INSERT INTO todos (title) VALUES ($1)")
            .bind(first_todo_title)
            .execute(&pool)
            .await
            .unwrap();
    
    }
}