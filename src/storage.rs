// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

pub mod database;

use crate::didcomm_types::ForwardMsg;
use async_trait::async_trait;
use database::get_db_pool;

#[cfg(any(
    not(any(feature = "any_db", feature = "postgres_db", feature = "mysql_db")),
    all(feature = "any_db", feature = "postgres_db", feature = "mysql_db"),
    all(feature = "any_db", feature = "postgres_db"),
    all(feature = "postgres_db", feature = "mysql_db"),
    all(feature = "any_db", feature = "mysql_db")
))]
compile_error!("Pick any one of \"any_db\", \"postgresql_db\", \"mysql_db\" feature flags.");

#[cfg(feature = "mysql_db")]
pub async fn init() -> sqlx::MySqlPool {
    get_db_pool().await
}

#[async_trait]
pub trait MediatorPersistence: Send + Sync + 'static {
    async fn persist_forward_message(&self, forward_msg: &ForwardMsg);
}

#[cfg(feature = "mysql_db")]
#[async_trait]
impl MediatorPersistence for sqlx::MySqlPool {
    async fn persist_forward_message(&self, forward_msg: &ForwardMsg) {
        sqlx::query("INSERT INTO forward_raw VALUES (DEFAULT, ?, ?, DEFAULT)")
            .bind(&forward_msg.recipient_key)
            .bind(&forward_msg.message)
            .execute(self)
            .await
            .unwrap();
    }
}
