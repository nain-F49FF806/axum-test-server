// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

pub mod database;
use database::get_db_pool;
use crate::didcomm_types::ForwardMsg;


#[cfg(any(
    not(any(feature = "any_db", feature = "postgres_db", feature = "mysql_db")),
    all(feature = "any_db", feature = "postgres_db", feature = "mysql_db"),
    all(feature = "any_db", feature = "postgres_db"),
    all(feature = "postgres_db", feature = "mysql_db"),
    all(feature = "any_db", feature = "mysql_db")
))]
compile_error!("Pick any one of \"any_db\", \"postgresql_db\", \"mysql_db\" feature flags.");

pub struct Storage {
    #[cfg(feature = "any_db")]
    pool: sqlx::AnyPool,
    #[cfg(feature = "postgres_db")]
    pool: sqlx::PgPool,
    #[cfg(feature = "mysql_db")]
    pool: sqlx::MySqlPool
}

impl Storage {
    pub async fn init() -> Storage {
        let pool = get_db_pool().await;
        Storage { pool }
    }
}

// #[async_trait]
// pub trait MediatorPersistence {
//     async fn persist_forward_message(&self, forward_msg: &ForwardMsg);
// }

// #[async_trait]
// impl MediatorPersistence for Storage {

impl Storage {
    pub async fn persist_forward_message(&self, forward_msg: &ForwardMsg) {
        sqlx::query("INSERT INTO forward_raw VALUES (DEFAULT, ?, ?, DEFAULT)")
            .bind(&forward_msg.recipient_key)
            .bind(&forward_msg.message)
            .execute(&self.pool)
            .await
            .unwrap();
    }
}
