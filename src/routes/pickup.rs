// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0
use crate::didcomm_types::PickupMsgEnum;
use crate::{didcomm_types::PickupStatusMsg, storage::MediatorPersistence};
use axum::{extract::State, Json};
use std::sync::Arc;
use log::info;

pub async fn handle_pickup<T: MediatorPersistence>(
    State(_storage): State<Arc<T>>,
    Json(pickup_message): Json<PickupMsgEnum>,
) -> String {
    if let PickupMsgEnum::PickupStatusReqMsg(status_request) = pickup_message {
        let status = PickupStatusMsg::new(5, &status_request.recipient_key);
        PickupMsgEnum::PickupStatusMsg(status);
        info!("StatusReqMsg");
        "".to_owned()
    } else {
        let status = PickupStatusMsg::new(10, "");
        PickupMsgEnum::PickupStatusMsg(status);
        info!("Something else");
        "".to_owned()
    }
}
