// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let app = axum::Router::new().route("/", axum::routing::get(|| async {"hey"}));

    axum::Server::bind(&"127.0.0.1:9999".parse().unwrap())
      .serve(app.into_make_service())
      .await
      .unwrap();
}
