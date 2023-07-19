// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0
use crate::router::create_router;
use crate::database::get_db_pool;

pub async fn run_server() {
    // app database bring up
    let pool = get_db_pool().await;
    // app definition, and routings
    let app = create_router(pool);

    // add server task to main loop
    axum::Server::bind(&"127.0.0.1:7999".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
