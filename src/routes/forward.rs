// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

use axum::{Json, extract::State};
use crate::didcomm_types::ForwardMsg;
use crate::storage::Storage;
use std::sync::Arc;
use log::{info, debug};

pub async fn handle_forward(State(storage): State<Arc<Storage>>, Json(forward_msg): Json<ForwardMsg>) -> Json<ForwardMsg> {
    info!("Persisting forward message");
    debug!("{forward_msg:#?}");
    storage.persist_forward_message(&forward_msg).await;
    Json(forward_msg)
}

#[cfg(test)]
mod tests {
    use crate::{database::get_db_pool, didcomm_types::ForwardMsg};

    #[tokio::test]
    async fn test_forward_msg_persist() {
        let forward_msg = ForwardMsg::default_alice();
        let pool = get_db_pool().await;

        sqlx::query("INSERT INTO forward_raw VALUES (DEFAULT, ?, ?, DEFAULT)")
            .bind(forward_msg.recipient_key)
            .bind(forward_msg.message)
            .execute(&pool)
            .await
            .unwrap();
    }
}