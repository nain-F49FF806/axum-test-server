// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0
mod logging;
mod router;
mod routes;
mod server;
use log::info;
use server::run_server;

#[tokio::main]
async fn main() {
    logging::init_logger();
    info!("Hello, world!");
    run_server().await;
}
