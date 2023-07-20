// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "AnyDB")] {
        mod any;
        pub use any::get_db_pool;
    }
    else if #[cfg(feature = "PostgresqlDB")] {
        mod postgres;
        pub use postgres::get_db_pool;
    }
    else if #[cfg(feature = "MysqlDB")] {
        mod mysql;
        pub use mysql::get_db_pool;
    }
}
