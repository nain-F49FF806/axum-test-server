// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

use crate::storage::MediatorPersistence;
use axum::{extract::State, Json};
use log::info;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "@type")]
pub enum MediatorCoordMsgEnum {
    #[serde(rename = "https://didcomm.org/coordinate-mediation/1.0/mediate-request")]
    MediateRequest(MediateRequest),
    #[serde(rename = "https://didcomm.org/coordinate-mediation/1.0/mediate-deny")]
    MediateDeny,
    #[serde(rename = "https://didcomm.org/coordinate-mediation/1.0/mediate-grant")]
    MediateGrant,
    #[serde(rename = "https://didcomm.org/coordinate-mediation/1.0/keylist-update")]
    KeylistUpdate,
    #[serde(rename = "https://didcomm.org/coordinate-mediation/1.0/keylist-update-response")]
    KeylistUpdateResponse,
    #[serde(rename = "https://didcomm.org/coordinate-mediation/1.0/keylist-query")]
    KeylistQuery,
    #[serde(rename = "https://didcomm.org/coordinate-mediation/1.0/keylist")]
    Keylist,
    XumErrorMsg{ error: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MediateRequest {
    auth_pubkey: String,
}

pub async fn handle_coord<T: MediatorPersistence>(
    State(storage): State<Arc<T>>,
    Json(message): Json<MediatorCoordMsgEnum>,
) -> Json<MediatorCoordMsgEnum> {
    match message {
        // MediateRequest => {
            
        // }
        _ => {
            handle_unimplemented().await
        }
    }
}

pub async fn handle_unimplemented() -> Json<MediatorCoordMsgEnum>{
    Json(MediatorCoordMsgEnum:: XumErrorMsg { error: "Unimplemented".to_owned() })
}

pub async fn handle_mediate_request<T: MediatorPersistence>(
    storage: Arc<T>,
    mediate_req: MediateRequest,
) -> Json<MediatorCoordMsgEnum> {
    Json(MediatorCoordMsgEnum::XumErrorMsg { error: "Tbd".to_owned() })

}