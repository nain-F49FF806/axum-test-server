// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0
use crate::didcomm_types::{PickupMsgEnum, PickupStatusMsg, PickupStatusReqMsg};
use crate::storage::MediatorPersistence;
use axum::{extract::State, Json};
use log::info;
use std::sync::Arc;

pub async fn handle_pickup<T: MediatorPersistence>(
    State(storage): State<Arc<T>>,
    Json(pickup_message): Json<PickupMsgEnum>,
) -> Json<PickupMsgEnum> {
    match &pickup_message {
        PickupMsgEnum::PickupStatusReqMsg(status_request) => {
            handle_pickup_status_req(status_request, storage).await
        }
        PickupMsgEnum::PickupStatusMsg(status) => {
            handle_pickup_status(status, storage).await
        }
    }
}

async fn handle_pickup_status_req<T: MediatorPersistence>(
    status_request: &PickupStatusReqMsg,
    storage: Arc<T>,
) -> Json<PickupMsgEnum> {
    info!("Received {:#?}", &status_request);
    let message_count = storage
        .retrieve_pending_message_count(status_request.recipient_key.as_ref())
        .await;
    let status = PickupStatusMsg {
        message_count: message_count,
        recipient_key: status_request.recipient_key.to_owned(),
    };
    info!("Sending {:#?}", &status);
    Json(PickupMsgEnum::PickupStatusMsg(status))
}

async fn handle_pickup_status<T: MediatorPersistence>(
    status: &PickupStatusMsg,
    storage: Arc<T>,
) -> Json<PickupMsgEnum> {
    info!("Received {:#?}", &status);
    let message_count = storage
        .retrieve_pending_message_count(status.recipient_key.as_ref())
        .await;
    let status = PickupStatusMsg {
        message_count: message_count,
        recipient_key: status.recipient_key.to_owned(),
    };
    info!("Sending {:#?}", &status);
    Json(PickupMsgEnum::PickupStatusMsg(status))
}
