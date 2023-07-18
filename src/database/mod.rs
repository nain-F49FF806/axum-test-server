// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

mod postgres;
mod mysql;
mod any;

pub use postgres::setup_postgresql_db;
pub use mysql::setup_mysql_db;
pub use any::get_db_pool;
