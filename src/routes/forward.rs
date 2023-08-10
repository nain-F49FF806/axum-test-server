// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

use crate::didcomm_types::ForwardMsg;
use crate::storage::MediatorPersistence;
use axum::{extract::State, Json};
use log::{debug, info};
use std::sync::Arc;

pub async fn handle_forward<T>(
    State(storage): State<Arc<T>>,
    Json(forward_msg): Json<ForwardMsg>,
) -> Json<ForwardMsg>
where
    T: MediatorPersistence,
{
    info!("Persisting forward message");
    debug!("{forward_msg:#?}");
    let _ = storage.persist_forward_message(&forward_msg.recipient_key, &forward_msg.message_data).await;
    Json(forward_msg)
}

#[cfg(test)]
mod tests {
    use crate::storage::database::get_db_pool;
    use crate::didcomm_types::ForwardMsg;
    use serde_json::json;

    #[tokio::test]
    async fn test_forward_msg_persist() {
        let forward_msg = json!(
            {
            "@type": "https://didcomm.org/routing/1.0/forward",
            "to": "Alice123",
            "msg": "Hello AAlice!"
            }
        );
        let forward_msg: ForwardMsg  = serde_json::from_str(&forward_msg.to_string()).unwrap();
        let pool = get_db_pool().await;

        // storage.persist_forward_message(auth_pubkey, &forward_msg.recipient_key, &forward_msg.message).await

        sqlx::query("INSERT INTO forward_raw VALUES (DEFAULT, ?, ?, DEFAULT)")
            .bind(&forward_msg.recipient_key)
            .bind(&forward_msg.message_data)
            .execute(&pool)
            .await
            .unwrap();
    }
}
