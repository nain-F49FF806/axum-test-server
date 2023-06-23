// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let app = Router::new().route("/", get(handle_get));

    axum::Server::bind(&"127.0.0.1:9999".parse().unwrap())
      .serve(app.into_make_service())
      .await
      .unwrap();
}

async fn handle_get() -> String  {
    "hey".to_owned()
}