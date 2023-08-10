// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

pub mod type_uri {
    pub const FORWARD: &str = "https://didcomm.org/routing/1.0/forward";
    pub const PICKUP_STATUS_REQ: &str = "https://didcomm.org/messagepickup/2.0/status-request";
    pub const PICKUP_STATUS: &str = "https://didcomm.org/messagepickup/2.0/status";
    pub const PICKUP_DELIVERY_REQ: &str = "https://didcomm.org/messagepickup/2.0/delivery-request";
    pub const PICKUP_DELIVERY: &str = "https://didcomm.org/messagepickup/2.0/delivery";
    pub const PICKUP_RECEIVED: &str = "https://didcomm.org/messagepickup/2.0/messages-received";
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ForwardMsg {
    #[serde(rename = "@type")]
    _type: String,
    #[serde(rename = "to")]
    pub recipient_key: String,
    #[serde(rename = "msg")]
    pub message_data: String,
}

// impl ForwardMsg {
//     pub fn default_alice() -> ForwardMsg {
//         ForwardMsg {
//             _type: type_uri::FORWARD.to_owned(),
//             recipient_key: "Alice".to_owned(),
//             message: "Hello!".to_owned(),
//         }
//     }
//     pub fn new(recipient_key: &str, message: &str) -> ForwardMsg {
//         ForwardMsg {
//             _type: type_uri::FORWARD.to_string(),
//             recipient_key: recipient_key.to_string(),
//             message: message.to_string(),
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "@type")]
pub enum PickupMsgEnum {
    #[serde(rename = "https://didcomm.org/messagepickup/2.0/status")]
    PickupStatusMsg(PickupStatusMsg),
    #[serde(rename = "https://didcomm.org/messagepickup/2.0/status-request")]
    PickupStatusReqMsg(PickupStatusReqMsg),
    #[serde(rename = "https://didcomm.org/messagepickup/2.0/delivery-request")]
    PickupDeliveryReq(PickupDeliveryReqMsg),
    #[serde(rename = "https://didcomm.org/messagepickup/2.0/delivery")]
    PickupDelivery(PickupDeliveryMsg),
    #[serde(rename = "https://didcomm.org/messagepickup/2.0/messages-received")]
    MessageReceived(MessageReceivedMsg),
    #[serde(rename = "https://didcomm.org/messagepickup/2.0/live-delivery-change")]
    LiveDeliveryChange(LiveDeliveryChangeMsg),
    #[serde(rename = "https://didcomm.org/notification/1.0/problem-report")]
    ProblemReport(ProblemReportMsg),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct PickupStatusMsg {
    pub message_count: u32,
    pub recipient_key: Option<String>,
}

impl PickupStatusMsg {
    pub fn new(message_count: u32, recipient_key: Option<String>) -> PickupStatusMsg {
        PickupStatusMsg {
            message_count,
            recipient_key,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct PickupStatusReqMsg {
    pub recipient_key: Option<String>,
}

impl PickupStatusReqMsg {
    pub fn new(recipient_key: Option<String>) -> PickupStatusReqMsg {
        PickupStatusReqMsg { recipient_key }
    }
    // pub fn custom_type(self, _type: String) -> PickupStatusReqMsg {
    //     PickupStatusReqMsg {
    //          recipient_key: self.recipient_key,
    //     }
    // }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct PickupDeliveryReqMsg {
    pub limit: u32,
    pub recipient_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct __Data {
    base64: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _Attach {
    pub id: String,
    #[serde(rename = "data")]
    pub __data: __Data,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct PickupDeliveryMsg {
    pub recipient_key: Option<String>,
    #[serde(rename = "~attach")]
    pub _attach: Vec<_Attach>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LiveDeliveryChangeMsg {
    pub live_delivery: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProblemReportMsg {
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReceivedMsg {
    pub message_id_list: Vec<String>,
}
