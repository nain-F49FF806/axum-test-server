// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0
use crate::didcomm_types::{PickupMsgEnum, PickupStatusMsg, PickupStatusReqMsg, ProblemReportMsg};
use crate::storage::MediatorPersistence;
use axum::{extract::State, Json, http::StatusCode};
use log::info;
use std::sync::Arc;

pub async fn handle_pickup<T: MediatorPersistence>(
    State(storage): State<Arc<T>>,
    Json(pickup_message): Json<PickupMsgEnum>,
) -> (StatusCode, Json<PickupMsgEnum>) {
    match &pickup_message {
        PickupMsgEnum::PickupStatusReqMsg(status_request) => {
            (
                StatusCode::OK, 
                handle_pickup_status_req(status_request, storage).await
            )
        }
        // Why is client sending us status? That's server's job.
        PickupMsgEnum::PickupStatusMsg(_status) => {
            (   
                StatusCode::BAD_REQUEST,
                handle_pickup_type_not_implemented().await
            )
        }
        _ => {
            info!("Received {:#?}", &pickup_message);
            (   
                StatusCode::NOT_IMPLEMENTED,
                handle_pickup_type_not_implemented().await
            )
        }
    }
}

async fn handle_pickup_status_req<T: MediatorPersistence>(
    status_request: &PickupStatusReqMsg,
    storage: Arc<T>,
) -> Json<PickupMsgEnum> {
    info!("Received {:#?}", &status_request);
    let auth_pubkey = &status_request.auth_pubkey;
    let message_count = storage
        .retrieve_pending_message_count(auth_pubkey, status_request.recipient_key.as_ref())
        .await.unwrap();
    let status = PickupStatusMsg {
        message_count,
        recipient_key: status_request.recipient_key.to_owned(),
    };
    info!("Sending {:#?}", &status);
    Json(PickupMsgEnum::PickupStatusMsg(status))
}

// Returns global status message for user (not restricted to recipient key)
// async fn handle_pickup_default<T: MediatorPersistence>(
//     storage: Arc<T>,
// ) -> Json<PickupMsgEnum> {
    
//     let message_count = storage
//         .retrieve_pending_message_count(None)
//         .await;
//     let status = PickupStatusMsg {
//         message_count,
//         recipient_key: None
//     };
//     info!("Sending {:#?}", &status);
//     Json(PickupMsgEnum::PickupStatusMsg(status))
// }

async fn handle_pickup_type_not_implemented(
) -> Json<PickupMsgEnum> {
    
    let problem = ProblemReportMsg {
        description: "This pickup request type not yet implemented.\n Please try again later".to_owned(),
    };
    info!("Sending {:#?}", &problem);
    Json(PickupMsgEnum::ProblemReport(problem))
}