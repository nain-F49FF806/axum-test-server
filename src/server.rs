// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0
use crate::router::create_router;
use crate::storage;
use log::info;

pub async fn run_server() {
    // Initialize a storage struct (this handles connection to storage backend)
    let storage = storage::init().await;
    // app definition, and routings
    let app = create_router(storage);
    info!("Starting server");
    // add server task to main loop
    axum::Server::bind(&"127.0.0.1:7999".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
