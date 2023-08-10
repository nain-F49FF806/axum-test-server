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
    async fn get_account(&self, auth_pubkey: &str) -> Result<Vec<u8>, String>;
    // async fn vaporize_account(&self, auth_pubkey: String);
    async fn add_recipient(&self, auth_pubkey: &str, recipient_key: &str) ->  Result<(), String>;
    async fn remove_recipient(&self, auth_pubkey: &str, recipient_key: &str) ->  Result<(), String>;
    async fn list_recipient_keys(&self, auth_pubkey: &str) -> Result<Vec<String>, String>;
    async fn persist_forward_message(&self, recipient_key: &str, message_data: &str) -> Result<(), String>;
    async fn retrieve_pending_message_count(&self, auth_pubkey: &str, recipient_key: Option<&String>) -> Result<u32, String>;
    // async fn retrieve_pending_messages(
    //     &self,
    //     limit: u32,
    //     recipient_key: Option<&String>,
    // ) -> Vec<(u32, Vec<u8>)>;
    // async fn mark_messages_received(&self, message_id: Vec<u32>);
}

#[cfg(feature = "mysql_db")]
#[async_trait]
impl MediatorPersistence for sqlx::MySqlPool {
    async fn create_account(&self, auth_pubkey: &str) -> Result<(), String> {
        info!("Adding new account to database with auth_pubkey {:#?}", &auth_pubkey);
        let insert_res = sqlx::query("INSERT INTO accounts (auth_pubkey) VALUES (?);")
            .bind(auth_pubkey)
            .execute(self)
            .await;
        if let Err(err) = insert_res {
            info!("Error during creating new account, {:#?}", err);
            return Err(format!("{:#}", err))
        };
        let account: Vec<u8> = sqlx::query("SELECT (account) FROM accounts WHERE auth_pubkey = ?;")
            .bind(auth_pubkey)
            .fetch_one(self)
            .await
            .unwrap()
            .get("account");
        info!("Created account {:#?}, Adding auth_pubkey {:#?} as a default recipient_key to account", &account, &auth_pubkey);
        sqlx::query("INSERT INTO recipients (account, recipient_key) VALUES (?, ?);")
        .bind(&account)
        .bind(auth_pubkey)
        .execute(self)
        .await
        .unwrap();
        Ok(())
    }
    /// Get account id associated with this auth_pubkey
    async fn get_account(&self, auth_pubkey: &str) -> Result<Vec<u8>, String> {
        let account: Vec<u8> = match 
        sqlx::query("SELECT (account) FROM accounts WHERE auth_pubkey = ?;")
        .bind(auth_pubkey)
        .fetch_one(self)
        .await
        {
            Ok(account_row) => {account_row.get("account") }
            Err(err) => {
                info!("Error while finding account, {:#?}", err);
                return Err(format!("{:#}", err))
            }
        };
        Ok(account)
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
        let mut rows = sqlx::query(
            "SELECT * FROM recipients WHERE recipient_key = ?"
        )
            .bind(recipient_key)
            .fetch(self);
        // Save message for each recipient
        while let Some(row) = rows.try_next().await.unwrap() {
            // map the row into a user-defined domain type
            let recipient: Vec<u8> = row.get("recipient");  // binary decode
            info!("Persisting message for recipient {:x?}", recipient);
            sqlx::query("INSERT INTO messages (recipient, message_data) VALUES (?, ?)")
            .bind(&recipient)
            .bind(message_data)
            .execute(self)
            .await
            .unwrap();
        }
        Ok(())
    }
    async fn retrieve_pending_message_count(&self, auth_pubkey: &str, recipient_key: Option<&String>) -> Result<u32, String> {
        let account: Vec<u8> = self.get_account(auth_pubkey).await?;
        let mut recipient_rows_stream = if let Some(recipient_key) = recipient_key {
            sqlx::query(
                "SELECT * FROM recipients WHERE (recipient_key = ?) AND (account = ?)"
            )
            .bind(recipient_key)
            .bind(account)
            .fetch(self)
        }
        else {
            sqlx::query(
                "SELECT * FROM recipients WHERE (account =  ?)"
            )
            .bind(account)
            .fetch(self)
        };
        let mut total_message_count: u32 = 0;
        while let Some(recipient_row) = recipient_rows_stream.try_next().await.unwrap() {
            let recipient: Vec<u8> = recipient_row.get("recipient");
            let message_count = sqlx::query(
                "SELECT COUNT(*) FROM messages 
                WHERE (recipient = ?) 
                -- AND (received = 0);"
            )
            .bind(&recipient)
            .fetch_one(self)
            .await
            .unwrap()
            .get::<i32, &str>("COUNT(*)"); // MySQL BIGINT can be converted to i32 only, not u32
            info!("Got count for Recipient {:x?}: {:#?} ", &recipient, &message_count);
            total_message_count += u32::try_from(message_count).unwrap();
        } 
        info!("Total message count of all recipients {:#?}", &total_message_count);
        Ok(total_message_count)
    }
    // async fn retrieve_pending_messages(&self, limit: u32, recipient_key: Option<&String>) -> Vec<(u32, Vec<u8>)> {
    //     info!("recipient key request {:#?}", recipient_key);
    //     let mut recipient_rows = if let Some(recipient_key) = recipient_key {
    //         sqlx::query(
    //             "SELECT * FROM recipients WHERE recipient_key = ?"
    //         )
    //             .bind(recipient_key)
    //             .fetch(self)
    //     }
    //     else {
    //         sqlx::query(
    //             "SELECT * FROM recipients"
    //         )
    //             .bind(recipient_key)
    //             .fetch(self)
    //     };
    //     let mut messages: Vec<(u32, Vec<u8>)> = Vec::new();
    //     let i: u32 = 0;
    //     while let Some(recipient_row) = recipient_rows.try_next().await.unwrap() {
    //         if i>= limit {break;}
    //         let recipient: Vec<u8> = recipient_row.get("recipient");  // binary decode
    //         let mut message_rows = sqlx::query(
    //             "SELECT * FROM messages WHERE recipient = ?"
    //         )
    //         .bind(&recipient).fetch(self);
    //         while let Some(message_row) = message_rows.try_next().await.unwrap() {
    //             let id: u32 = message_row.get("id");
    //             let msg : Vec<u8> = message_row.get("message_data");
    //             info!("id {:#?}", id);
    //             info!("recipient {:x?}", recipient);
    //             info!("message {:x?}", msg); 
    //             messages.push((id, msg));
    //         }
    //     }
    //     messages
    // }
    async fn add_recipient(&self, auth_pubkey: &str, recipient_key: &str) ->  Result<(), String> {
        info!("Adding recipient_key to account with auth_pubkey {:#?}", auth_pubkey);
        let account: Vec<u8> = self.get_account(auth_pubkey).await?;
        info!(
            "Found matching account {:x?}. Proceeding with attempt to add recipient recipient_key {:#?} ",
            account,
            recipient_key
        );
        match sqlx::query("INSERT INTO recipients (account, recipient_key) VALUES (?, ?);")
            .bind(&account)
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
        let account: Vec<u8> = self.get_account(auth_pubkey).await?;
        info!(
            "Found matching account {:x?}. Proceeding with attempt to remove recipient recipient_key {:#?} ",
            account,
            recipient_key
        );
        match sqlx::query("DELETE FROM recipients WHERE (account = ?) AND (recipient_key = ?);")
            .bind(&account)
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
        let account: Vec<u8> = self.get_account(auth_pubkey).await?;
        let recipient_keys: Vec<String> = match
            sqlx::query("SELECT (recipient_key) FROM recipients WHERE account = ?;")
            .bind(&account)
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
