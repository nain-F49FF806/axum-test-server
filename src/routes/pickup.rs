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


    if let PickupMsgEnum::PickupStatusReqMsg(status_request) = pickup_message {
        let status = PickupStatusMsg::new(5, &status_request.recipient_key);
        info!("StatusReqMsg");
        Json(PickupMsgEnum::PickupStatusMsg(status))
    } else {
        let status = PickupStatusMsg::new(10, "");
        info!("Something else");
        Json(PickupMsgEnum::PickupStatusMsg(status))
    }
}
