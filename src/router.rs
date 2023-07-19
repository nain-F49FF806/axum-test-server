// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0
use crate::routes::hello_world;
use crate::routes::json;
use crate::routes::json::respond_message_json;
use crate::routes::forward::handle_forward;
use axum::{routing::get, routing::post, Router};
use std::sync::Arc;

pub fn create_router(pool: sqlx::AnyPool) -> Router {
    Router::new()
        .route(
            "/",
            get(hello_world::handle_get).post(hello_world::handle_get),
        )
        .route(
            "/json",
            get(json::echo_message_json).post(respond_message_json),
        )
        .route(
            "/forward",
            post(handle_forward),
        )
        .with_state(Arc::new(pool))
}
