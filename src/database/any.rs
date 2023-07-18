// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

use sqlx::any::AnyPoolOptions;
use sqlx::AnyPool;

/// Gives a connection pool based on DATABASE_URL in the .env file
/// Could be Postgres (postgres://), Mysql(mysql://) or Sqlite(sqlite://)
pub async fn get_db_pool () -> AnyPool {
    let _ = dotenvy::dotenv().expect(".env file not found!");
    let database_url = 
        dotenvy::var("DATABASE_URL").expect("Environment variable DATABASE_URL not found in .env!");
    sqlx::any::install_default_drivers();
    AnyPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to database!")
}
