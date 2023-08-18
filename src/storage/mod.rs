// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

pub mod database;

use async_trait::async_trait;
use database::get_db_pool;


#[cfg(any(
    not(any(feature = "any_db", feature = "postgres_db", feature = "mysql_db")),
    all(feature = "any_db", feature = "postgres_db", feature = "mysql_db"),
    all(feature = "any_db", feature = "postgres_db"),
    all(feature = "postgres_db", feature = "mysql_db"),
    all(feature = "any_db", feature = "mysql_db")
))]
compile_error!("Pick any one of \"any_db\", \"postgresql_db\", \"mysql_db\" feature flags.");

#[cfg(feature = "mysql_db")]
pub async fn init() -> sqlx::MySqlPool {
    get_db_pool().await
}

#[async_trait]
pub trait MediatorPersistence: Send + Sync + 'static {
    async fn create_account(&self, auth_pubkey: &str) -> Result<(), String>;
    async fn get_account_id(&self, auth_pubkey: &str) -> Result<Vec<u8>, String>;
    // async fn vaporize_account(&self, auth_pubkey: String);
    async fn add_recipient(&self, auth_pubkey: &str, recipient_key: &str) ->  Result<(), String>;
    async fn remove_recipient(&self, auth_pubkey: &str, recipient_key: &str) ->  Result<(), String>;
    async fn list_recipient_keys(&self, auth_pubkey: &str) -> Result<Vec<String>, String>;
    async fn persist_forward_message(&self, recipient_key: &str, message_data: &str) -> Result<(), String>;
    async fn retrieve_pending_message_count(&self, auth_pubkey: &str, recipient_key: Option<&String>) -> Result<u32, String>;
    async fn retrieve_pending_messages(
        &self,
        auth_pubkey: &str,
        limit: u32,
        recipient_key: Option<&String>,
    ) -> Result<Vec<(String, Vec<u8>)>, String>;
    // async fn mark_messages_received(&self, message_id: Vec<u32>);
}
