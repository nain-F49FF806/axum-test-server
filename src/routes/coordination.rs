// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

use crate::storage::MediatorPersistence;
use axum::{extract::State, Json};
use mediator_coord_structs::MediatorCoordMsgEnum::*;
use mediator_coord_structs::*;
use std::sync::Arc;

mod mediator_coord_structs {
    use serde::{Deserialize, Serialize};
    // use serde_with::skip_serializing_none;

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(tag = "@type")]
    pub enum MediatorCoordMsgEnum {
        #[serde(rename = "https://didcomm.org/coordinate-mediation/1.0/mediate-request")]
        MediateRequest(MediateRequestData),
        #[serde(rename = "https://didcomm.org/coordinate-mediation/1.0/mediate-deny")]
        MediateDeny(MediateDenyData),
        #[serde(rename = "https://didcomm.org/coordinate-mediation/1.0/mediate-grant")]
        MediateGrant,
        #[serde(rename = "https://didcomm.org/coordinate-mediation/1.0/keylist-update")]
        KeylistUpdate(KeylistUpdateData),
        #[serde(rename = "https://didcomm.org/coordinate-mediation/1.0/keylist-update-response")]
        KeylistUpdateResponse(KeylistUpdateData),
        #[serde(rename = "https://didcomm.org/coordinate-mediation/1.0/keylist-query")]
        KeylistQuery(KeylistQueryData),
        #[serde(rename = "https://didcomm.org/coordinate-mediation/1.0/keylist")]
        Keylist(KeylistData),
        XumErrorMsg {
            error: String,
        },
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MediateRequestData {
        pub auth_pubkey: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MediateDenyData {
        pub reason: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct KeylistUpdateData {
        pub auth_pubkey: String,
        #[serde(rename(serialize = "updated", deserialize = "updates"))]
        pub updates: Vec<KeylistUpdateItem>,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct KeylistUpdateItem {
        pub recipient_key: String,
        pub action: KeylistUpdateItemAction,
        pub result: Option<KeylistUpdateItemResult>,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub enum KeylistUpdateItemAction {
        #[serde(rename = "add")]
        Add,
        #[serde(rename = "remove")] 
        Remove,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub enum KeylistUpdateItemResult {
        ClientError,
        ServerError,
        NoChange,
        Success,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct KeylistQueryData {
        pub auth_pubkey: String,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct KeylistData {
        pub keys: Vec<KeylistItem>,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct KeylistItem {
        pub recipient_key: String,
    } 
}

pub async fn handle_coord<T: MediatorPersistence>(
    State(storage): State<Arc<T>>,
    Json(message): Json<MediatorCoordMsgEnum>,
) -> Json<MediatorCoordMsgEnum> {
    match message {
        MediateRequest(mediate_req) => handle_mediate_request(storage, mediate_req).await,
        KeylistUpdate(keylist_update_data) => handle_keylist_update(storage, keylist_update_data).await,
        KeylistQuery(keylist_query_data) => handle_keylist_query(storage, keylist_query_data).await,
        _ => handle_unimplemented().await,
    }
}

pub async fn handle_unimplemented() -> Json<MediatorCoordMsgEnum> {
    Json(MediatorCoordMsgEnum::XumErrorMsg {
        error: "Unimplemented".to_owned(),
    })
}

pub async fn handle_mediate_request<T: MediatorPersistence>(
    storage: Arc<T>,
    mediate_req: MediateRequestData,
) -> Json<MediatorCoordMsgEnum> {
    let auth_pubkey = &mediate_req.auth_pubkey;
    match storage.create_account(auth_pubkey).await {
        Ok(()) => Json(MediateGrant),
        Err(msg) => Json(MediateDeny(MediateDenyData { reason: msg })),
    }
}

pub async fn handle_keylist_query<T: MediatorPersistence>(
    storage: Arc<T>,
    keylist_query_data: KeylistQueryData,
) -> Json<MediatorCoordMsgEnum> {
    let auth_pubkey = &keylist_query_data.auth_pubkey;
    let keylist_items: Vec<KeylistItem> = match storage.list_recipient_keys(auth_pubkey).await {
        Ok(recipient_keys) => {
            recipient_keys
                .into_iter()
                .map(|recipient_key| KeylistItem{recipient_key})
                .collect()
        },
        Err(err) => {
            return Json(MediatorCoordMsgEnum::XumErrorMsg{error: err})
        }
    };
    Json(MediatorCoordMsgEnum::Keylist( KeylistData { keys: keylist_items }))
}

pub async fn handle_keylist_update<T: MediatorPersistence>(
    storage: Arc<T>,
    keylist_update_data: KeylistUpdateData,
) -> Json<MediatorCoordMsgEnum> {
    let auth_pubkey = &keylist_update_data.auth_pubkey;
    let updates: Vec<KeylistUpdateItem> = keylist_update_data.updates;
    let mut updated: Vec<KeylistUpdateItem> = Vec::new();
    for update_item in updates.into_iter() {
        let result = match &update_item.action {
            KeylistUpdateItemAction::Add => {
                storage
                    .add_recipient(auth_pubkey, &update_item.recipient_key)
                    .await
            }
            KeylistUpdateItemAction::Remove => {
                storage
                    .remove_recipient(auth_pubkey, &update_item.recipient_key)
                    .await
            }
        };
        let update_item_result = match result {
            Ok(()) => KeylistUpdateItemResult::Success,
            Err(_msg) => KeylistUpdateItemResult::ServerError,
        };
        updated.push(KeylistUpdateItem {
            recipient_key: update_item.recipient_key,
            action: update_item.action,
            result: Some(update_item_result),
        });
    }
    Json(MediatorCoordMsgEnum::KeylistUpdateResponse(
        KeylistUpdateData {
            auth_pubkey: keylist_update_data.auth_pubkey,
            updates: updated,
        },
    ))
}
