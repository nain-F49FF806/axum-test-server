// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

use crate::database::get_db_pool;
use crate::didcomm_types::ForwardMsg;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "AnyDB")] {
        pub struct Storage {
            pool: sqlx::AnyPool,
        }
    } else if #[cfg(feature = "PostgresqlDB")] {
        pub struct Storage {
            pool: sqlx::PgPool,
        }
    } else if #[cfg(feature = "MysqlDB")] {
        pub struct Storage {
            pool: sqlx::MySqlPool,
        }
    }
}

impl Storage {
    pub async fn init() -> Storage {
        let pool = get_db_pool().await;
        Storage { pool }
    }
    pub async fn persist_forward_message(&self, forward_msg: &ForwardMsg) {
        sqlx::query("INSERT INTO forward_raw VALUES (DEFAULT, ?, ?, DEFAULT)")
            .bind(&forward_msg.recipient_key)
            .bind(&forward_msg.message)
            .execute(&self.pool)
            .await
            .unwrap();
    }
}
