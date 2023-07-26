// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0
use crate::didcomm_types::PickupMsgEnum;
use crate::{didcomm_types::PickupStatusMsg, storage::MediatorPersistence};
use axum::{extract::State, Json};
use std::sync::Arc;
use log::info;

pub async fn handle_pickup<T: MediatorPersistence>(
    State(storage): State<Arc<T>>,
    Json(pickup_message): Json<PickupMsgEnum>,
) -> Json<PickupMsgEnum> {

    match &pickup_message {
        PickupMsgEnum::PickupStatusReqMsg(status_request) => {
            info!("Received StatusReqMsg {:#?}", &status_request);
            let message_count = storage.retrieve_pending_message_count(&status_request.recipient_key).await;
            let status = PickupStatusMsg::new(message_count.try_into().unwrap(), &status_request.recipient_key);
            info!("Sending StatusMsg {:#?}", &status);
            Json(PickupMsgEnum::PickupStatusMsg(status))
        }
        PickupMsgEnum::PickupStatusMsg(status) => {
            info!("Received StatusMsg {:#?}", &status);
            let message_count = storage.retrieve_pending_message_count(&status.recipient_key).await;
            let status = PickupStatusMsg::new(message_count.try_into().unwrap(), &status.recipient_key);
            info!("Sending StatusMsg {:#?}", &status);
            Json(PickupMsgEnum::PickupStatusMsg(status))
        }
    }
}
