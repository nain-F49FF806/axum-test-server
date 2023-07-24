// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "any_db")]
mod any;
#[cfg(feature = "any_db")]
pub use any::get_db_pool;

#[cfg(feature = "postgres_db")]
mod postgres;
#[cfg(feature = "postgres_db")]
pub use postgres::get_db_pool;

#[cfg(feature = "mysql_db")]
mod mysql;
#[cfg(feature = "mysql_db")]
pub use mysql::get_db_pool;
