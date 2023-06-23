// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0
use axum::{Router, routing::get};
use crate::routes::hello_world;
use crate::routes::json;
use crate::routes::json::respond_message_json;

pub fn create_router() -> Router {
    Router::new()
      .route("/", get(hello_world::handle_get).post(hello_world::handle_get))
      .route("/json", get(json::echo_message_json).post(respond_message_json))
}
