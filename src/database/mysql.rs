// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

#[allow(dead_code)]
pub async fn get_db_pool() -> MySqlPool {
    let _ = dotenvy::dotenv();
    let database_url = 
        std::env::var("MYSQL_URL").expect("Required environment variable MYSQL_URL on command line or in .env!");

    MySqlPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to database!")
}
