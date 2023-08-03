// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

pub mod database;

use crate::didcomm_types::ForwardMsg;
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
    // async fn create_account(&self, auth_pubkey: String);
    // async fn vaporize_account(&self, auth_pubkey: String);
    async fn persist_forward_message(&self, forward_msg: &ForwardMsg);
    async fn retrieve_pending_message_count(&self, recipient_key: Option<&String>) -> u32;
    // async fn retrieve_pending_messages(
    //     &self,
    //     limit: u32,
    //     recipient_key: Option<&String>,
    // ) -> Vec<(u32, Vec<u8>)>;
    // async fn mark_messages_recieved(&self, message_id: Vec<u32>);
}

#[cfg(feature = "mysql_db")]
#[async_trait]
impl MediatorPersistence for sqlx::MySqlPool {
    // async fn create_account(&self, auth_pubkey: String) {
    //     info!("Adding new account to database with auth_pubkey {:#?}", &auth_pubkey);
    //     let insert_res = sqlx::query("INSERT INTO accounts (auth_pubkey) VALUES (?);")
    //         .bind(&auth_pubkey)
    //         .execute(self)
    //         .await;
    //     if let Err(err) = insert_res {
    //         info!("Error during creating new account {:#?}", err);
    //         return
    //     };
    //     let account: Vec<u8> = sqlx::query("SELECT (account) FROM accounts WHERE auth_pubkey = ?;")
    //         .bind(&auth_pubkey)
    //         .fetch_one(self)
    //         .await
    //         .unwrap()
    //         .get("account");
    //     info!("Created account {:#?}, Adding auth_pubkey {:#?} as a default recipient_key to account", &account, &auth_pubkey);
    //     sqlx::query("INSERT INTO recipients (account, recipient_key) VALUES (?, ?);")
    //     .bind(&account)
    //     .bind(&auth_pubkey)
    //     .execute(self)
    //     .await
    //     .unwrap();
    // }
    // async fn vaporize_account(&self, auth_pubkey: String) {
    //     let account: Vec<u8> = sqlx::query("SELECT (account) FROM accounts WHERE auth_pubkey = ?;")
    //     .bind(&auth_pubkey)
    //     .fetch_one(self)
    //     .await
    //     .unwrap()
    //     .get("account");

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
    async fn persist_forward_message(&self, forward_msg: &ForwardMsg) {
        // Fetch recipients with given recipient_key
        let mut rows = sqlx::query(
            "SELECT * FROM recipients WHERE recipient_key = ?"
        )
            .bind(&forward_msg.recipient_key)
            .fetch(self);
        // Save message for each recipient
        while let Some(row) = rows.try_next().await.unwrap() {
            // map the row into a user-defined domain type
            let recipient: Vec<u8> = row.get("recipient");  // binary decode
            info!("Persisting message for recipient {:x?}", recipient);
            sqlx::query("INSERT INTO messages (recipient, message_data) VALUES (?, ?)")
            .bind(&recipient)
            .bind(&forward_msg.message)
            .execute(self)
            .await
            .unwrap();
        }
    }
    async fn retrieve_pending_message_count(&self, recipient_key: Option<&String>) -> u32 {
        if let Some(recipient_key) = recipient_key {
            // Fetch *one* recipient with recipient_key
            // This should later be redone to use auth_pubkey registered recipient
            let recipient: Vec<u8> = sqlx::query(
                "SELECT * FROM recipients WHERE recipient_key = ?"
            )
            .bind(&recipient_key)
            .fetch_one(self)
            .await.unwrap()
            .get("recipient");
            info!("Getting count for Recipient {:x?}", recipient);
            let message_count = sqlx::query(
                "SELECT COUNT(*)
                    FROM messages
                    WHERE recipient = ?; -- AND received = 0;"
            )
            .bind(&recipient)
            .fetch_one(self)
            .await
            .unwrap()
            .get::<i32, &str>("COUNT(*)");
            message_count.try_into().unwrap()
        }
        else {
            let mut recipient_rows = sqlx::query(
                "SELECT * FROM recipients WHERE recipient_key = ?"
            )
            .bind(&recipient_key)
            .fetch(self);
            let mut total_message_count: i32 = 0;
            while let Some(recipient_row) = recipient_rows.try_next().await.unwrap() {
                let recipient: Vec<u8> = recipient_row.get("recipient");
                info!("Getting count for Recipient {:x?}", recipient);
                let message_count = sqlx::query(
                    "SELECT COUNT(*)
                        FROM messages;
                        -- WHERE received = 0;"
                )
                .fetch_one(self)
                .await
                .unwrap()
                .get::<i32, &str>("COUNT(*)");
                total_message_count = total_message_count + message_count;
            } 
            info!("Total message count of all recipients {:#?}", &total_message_count);
            total_message_count.try_into().unwrap()
        }
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
}
