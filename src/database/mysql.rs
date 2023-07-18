// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

pub async fn setup_mysql_db() -> Result<MySqlPool, dotenvy::Error> {
    let _ = dotenvy::dotenv()?;
    let database_url = dotenvy::var("MYSQL_URL")?;
    let pool = MySqlPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to database!");
    Ok(pool)
}
