// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

use axum::{Json, extract::State};
use crate::didcomm_types::ForwardMsg;
use std::sync::Arc;
use log::{info, debug};

pub async fn handle_forward(State(arc_pool): State<Arc<sqlx::AnyPool>>, Json(forward_msg): Json<ForwardMsg>) -> Json<ForwardMsg> {
    forward_message_persist(&forward_msg, &arc_pool).await;
    Json(forward_msg)
}


pub async fn forward_message_persist(forward_msg: &ForwardMsg, pool: &sqlx::AnyPool) {
    info!("Persisting message into database");
    debug!("{forward_msg:?}");
    sqlx::query("INSERT INTO forward_raw VALUES (DEFAULT, ?, ?, DEFAULT)")
    .bind(&forward_msg.recipient_key)
    .bind(&forward_msg.msg)
    .execute(pool)
    .await
    .unwrap();
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
            .bind(forward_msg.msg)
            .execute(&pool)
            .await
            .unwrap();
    }
}