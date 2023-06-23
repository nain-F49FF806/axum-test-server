// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0
use axum::{Router, routing::get};
use crate::routes::hello_world;

pub fn create_router() -> Router {
    let app = Router::new().route("/", get(hello_world::handle_get));
    return app;
}
