// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0
use axum::{Router, routing::get};

pub fn create_router() -> Router {
    let app = Router::new().route("/", get(handle_get));
    return app;
}

async fn handle_get() -> String  {
    "hey".to_owned()
}