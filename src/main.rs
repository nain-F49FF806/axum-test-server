// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0
mod logging;
mod router;
mod routes;
mod server;
mod database;
use log::info;
use server::run_server;

#[tokio::main]
async fn main() {
    logging::init_logger();
    info!("Hello, world!");
    let _pool = database::get_db_pool().await;
    run_server().await;
}
