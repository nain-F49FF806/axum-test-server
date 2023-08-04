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
        KeylistUpdateResponse,
        #[serde(rename = "https://didcomm.org/coordinate-mediation/1.0/keylist-query")]
        KeylistQuery,
        #[serde(rename = "https://didcomm.org/coordinate-mediation/1.0/keylist")]
        Keylist,
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
        #[serde(rename(serialize = "updated", deserialize = "updates"))]
        updates: Vec<KeylistUpdateItem>,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct KeylistUpdateItem {
        recipient_key: String,
        action: KeylistUpdateItemAction,
        result: Option<KeylistUpdateItemResult>,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub enum KeylistUpdateItemAction {
        Add,
        Remove,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub enum KeylistUpdateItemResult {
        ClientError,
        ServerError,
        NoChange,
        Success,
    }
}

pub async fn handle_coord<T: MediatorPersistence>(
    State(storage): State<Arc<T>>,
    Json(message): Json<MediatorCoordMsgEnum>,
) -> Json<MediatorCoordMsgEnum> {
    match message {
        MediateRequest(mediate_req) => handle_mediate_request(storage, mediate_req).await,
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
    let auth_pubkey = mediate_req.auth_pubkey;
    match storage.create_account(auth_pubkey).await {
        Ok(()) => Json(MediateGrant),
        Err(msg) => Json(MediateDeny(MediateDenyData { reason: msg })),
    }
}
