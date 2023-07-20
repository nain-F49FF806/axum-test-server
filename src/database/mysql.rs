// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

#[allow(dead_code)]
pub async fn get_db_pool() -> MySqlPool {
    let _ = dotenvy::dotenv().expect(".env file not found! Need .env file with MYSQL_URL variable defined");
    let database_url = 
        std::env::var("MYSQL_URL").expect("Environment variable MYSQL_URL not found in .env!");

    MySqlPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to database!")
}
