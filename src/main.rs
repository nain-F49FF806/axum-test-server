// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0
mod router;
mod server;
use server::run_server;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    run_server().await;
}
