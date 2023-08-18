// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

pub mod database;
/// Database backend is used for default implementation of MediatorPersistence trait
pub use database::get_db_pool as get_persistence;

use async_trait::async_trait;

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
