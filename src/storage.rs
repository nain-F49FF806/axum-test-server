// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

pub mod database;

use async_trait::async_trait;
use database::get_db_pool;
use log::info;
use sqlx::Row;
use futures::TryStreamExt;


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

#[cfg(feature = "mysql_db")]
#[async_trait]
impl MediatorPersistence for sqlx::MySqlPool {
    async fn create_account(&self, auth_pubkey: &str) -> Result<(), String> {
        info!("Adding new account to database with auth_pubkey {:#?}", &auth_pubkey);
        let insert_result = sqlx::query("INSERT INTO accounts (auth_pubkey) VALUES (?);")
            .bind(auth_pubkey)
            .execute(self)
            .await;
        if let Err(err) = insert_result {
            info!("Error during creating new account, {:#?}", err);
            return Err(format!("{:#}", err))
        };
        let account_id = self.get_account_id(auth_pubkey).await?;
        info!("Created account {:x?} for auth_pubkey {:#?}", &account_id, &auth_pubkey);
        Ok(())
    }
    /// Get account id associated with auth_pubkey
    async fn get_account_id(&self, auth_pubkey: &str) -> Result<Vec<u8>, String> {
        let account_id: Vec<u8> = match 
        sqlx::query("SELECT (account_id) FROM accounts WHERE auth_pubkey = ?;")
        .bind(auth_pubkey)
        .fetch_one(self)
        .await
        {
            Ok(account_row) => {account_row.get("account_id") }
            Err(err) => {
                info!("Error while finding account, {:#?}", err);
                return Err(format!("{:#}", err))
            }
        };
        Ok(account_id)
    }
    // async fn vaporize_account(&self, auth_pubkey: String) {
    //     let account: Vec<u8> = self.get_account(auth_pubkey).await?;
    //     let mut recipient_rows = sqlx::query(
    //         "SELECT * FROM recipients WHERE account = ?;"
    //     )
    //         .bind(&account)
    //         .fetch(self);

    //     while let Some(recipient_row) = recipient_rows.try_next().await.unwrap() {
    //         // map the row into a user-defined domain type
    //         let recipient: Vec<u8> = recipient_row.get("recipient");  // binary decode
    //         info!("Recipient {:x?}", recipient);
    //         sqlx::query("DROP (*) FROM messages WHERE recipient = ?;")
    //         .bind(&recipient)
    //         .execute(self)
    //         .await
    //         .unwrap();
    //         sqlx::query("DROP (*) FROM recipients WHERE recipient = ?;")
    //         .bind(&recipient)
    //         .execute(self)
    //         .await
    //         .unwrap();
            
    //     }
    
    // }
    async fn persist_forward_message(&self, recipient_key: &str, message_data: &str) -> Result<(), String> {
        // Fetch recipients with given recipient_key
        info!("Fetching recipients with recipient_key {:#?}", recipient_key);
        let recipient_row = sqlx::query(
            "SELECT * FROM recipients WHERE recipient_key = ?"
        )
            .bind(recipient_key)
            .fetch_one(self)
            .await;
        if let Err(err) = recipient_row {
            info!("Error while finding target recipient, {:#}", err);
            return Err(format!("{:#}", err))
        }
        let account_id: Vec<u8> = recipient_row.unwrap().get("account_id");
        // Save message for recipient
        info!("Persisting message for account {:x?}", account_id);
        let insert_result = sqlx::query("INSERT INTO messages (account_id, recipient_key, message_data) VALUES (?, ?, ?)")
            .bind(&account_id)
            .bind(recipient_key)
            .bind(message_data)
            .execute(self)
            .await;
        if let Err(err) = insert_result {
            info!("Error while saving message for recipient {:x?}, {:#}", recipient_key, err);
            return Err(format!("{:#}", err))
        }
        Ok(())
    }
    async fn retrieve_pending_message_count(&self, auth_pubkey: &str, recipient_key: Option<&String>) -> Result<u32, String> {
        let account_id: Vec<u8> = self.get_account_id(auth_pubkey).await?;
        let mut recipient_rows_stream = if let Some(recipient_key) = recipient_key {
            sqlx::query(
                "SELECT * FROM recipients WHERE (account_id = ?) and (recipient_key = ?)"
            )
            .bind(&account_id)
            .bind(recipient_key)
            .fetch(self)
        }
        else {
            sqlx::query(
                "SELECT * FROM recipients WHERE (account_id =  ?)"
            )
            .bind(&account_id)
            .fetch(self)
        };
        let mut total_message_count: u32 = 0;
        while let Some(recipient_row) = recipient_rows_stream.try_next().await.unwrap() {
            let recipient_key: String = recipient_row.get("recipient_key");
            let message_count = sqlx::query(
                "SELECT COUNT(*) FROM messages 
                WHERE (account_id = ?) AND (recipient_key = ?)
                -- AND (received = 0);"
            )
            .bind(&account_id)
            .bind(&recipient_key)
            .fetch_one(self)
            .await
            .unwrap()
            .get::<i32, &str>("COUNT(*)"); // MySQL BIGINT can be converted to i32 only, not u32
            info!("Got count for recipient_key {:x?}: {:#?} ", &recipient_key, &message_count);
            total_message_count += u32::try_from(message_count).unwrap();
        } 
        info!("Total message count of all requested recipients {:#?}", &total_message_count);
        Ok(total_message_count)
    }
    async fn retrieve_pending_messages(
        &self,
        auth_pubkey: &str,
        limit: u32,
        recipient_key: Option<&String>,
    ) -> Result<Vec<(String, Vec<u8>)>, String> {
        info!("Processing retrieve for messages to recipient_key {:#?} of auth_pubkey {:#?}", recipient_key, auth_pubkey);
        let account_id: Vec<u8> = self.get_account_id(auth_pubkey).await?;
        let mut messages: Vec<(String, Vec<u8>)> = Vec::new();
        let mut message_rows = if let Some(recipient_key) = recipient_key {
            sqlx::query(
                "SELECT * FROM messages WHERE (account_id = ?) AND (recipient_key = ?)"
            )
                .bind(&account_id)
                .bind(recipient_key)
                .fetch(self)
        } else {
            sqlx::query(
                "SELECT * FROM messages WHERE (account_id = ?)"
            )
                .bind(&account_id)
                .fetch(self)
        };
        while let Some(message_row) = message_rows.try_next().await.unwrap() {
            let id: String = message_row.get("message_id");
            let msg : Vec<u8> = message_row.get("message_data");
            // debug!("id {:#?}", id);
            // debug!("recipient {:x?}", recipient);
            // debug!("message {:x?}", msg); 
            messages.push((id, msg));
            if u32::try_from(messages.len()).unwrap() >= limit {
                info!("Found enough messages {:#?}", limit);
                break;
            }
        }
        info!("Found total of {:#?} messages, returning them", messages.len());
        Ok(messages)
    }
    async fn add_recipient(&self, auth_pubkey: &str, recipient_key: &str) ->  Result<(), String> {
        info!("Adding recipient_key to account with auth_pubkey {:#?}", auth_pubkey);
        let account_id: Vec<u8> = self.get_account_id(auth_pubkey).await?;
        info!(
            "Found matching account {:x?}. Proceeding with attempt to add recipient recipient_key {:#?} ",
            account_id,
            recipient_key
        );
        match sqlx::query("INSERT INTO recipients (account_id, recipient_key) VALUES (?, ?);")
            .bind(&account_id)
            .bind(recipient_key)
            .execute(self)
            .await
        {
            Ok(_result) => Ok(()),
            Err(err) => {
                info!("Error while adding recipient, {:#}", err);
                Err(format!("{:#}", err))
            }
        }
    }
    async fn remove_recipient(&self, auth_pubkey: &str, recipient_key: &str) ->  Result<(), String> {
        info!("Removing recipient_key from account with auth_pubkey {:#?}", auth_pubkey);
        let account_id: Vec<u8> = self.get_account_id(auth_pubkey).await?;
        info!(
            "Found matching account {:x?}. Proceeding with attempt to remove recipient recipient_key {:#?} ",
            account_id,
            recipient_key
        );
        match sqlx::query("DELETE FROM recipients WHERE (account_id = ?) AND (recipient_key = ?);")
            .bind(&account_id)
            .bind(recipient_key)
            .execute(self)
            .await
        {
            Ok(_result) => Ok(()),
            Err(err) => {
                info!("Error while removing recipient, {:#}", err);
                Err(format!("{:#}", err))
            }
        }
    }
    async fn list_recipient_keys(&self, auth_pubkey: &str) -> Result<Vec<String>, String> {
        info!("Retrieving recipient_keys for account with auth_pubkey {:#?}", auth_pubkey);
        let account_id: Vec<u8> = self.get_account_id(auth_pubkey).await?;
        let recipient_keys: Vec<String> = match
            sqlx::query("SELECT (recipient_key) FROM recipients WHERE account_id = ?;")
            .bind(&account_id)
            .fetch_all(self)
            .await
        {
            Ok(recipient_key_rows) => {
                recipient_key_rows
                    .into_iter()
                    .map(|row| row.get("recipient_key"))
                    .collect()
            }
            Err(err) => {
                info!("Error while getting recipient_keys, {:#}", err);
                return Err(format!("{:#}", err))
            }
        };
        Ok(recipient_keys)
    }
}
