// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0
use axum::{Router, routing::get};
use crate::routes::hello_world;
use crate::routes::json;

pub fn create_router() -> Router {
    Router::new()
      .route("/", get(hello_world::handle_get).post(hello_world::handle_get))
      .route("/json", get(json::echo_message_json))
}
