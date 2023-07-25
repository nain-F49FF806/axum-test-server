// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

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
    pub message: String,
}

impl ForwardMsg {
    pub fn default_alice() -> ForwardMsg {
        ForwardMsg {
            _type: type_uri::FORWARD.to_owned(),
            recipient_key: "Alice".to_owned(),
            message: "Hello!".to_owned(),
        }
    }
    pub fn new(recipient_key: &str, message: &str) -> ForwardMsg {
        ForwardMsg {
            _type: type_uri::FORWARD.to_string(),
            recipient_key: recipient_key.to_string(),
            message: message.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "@type")]
pub enum PickupMsgEnum {
    #[serde(rename = "https://didcomm.org/messagepickup/2.0/status")]
    PickupStatusMsg(PickupStatusMsg),
    #[serde(rename = "https://didcomm.org/messagepickup/2.0/status-request")]
    PickupStatusReqMsg(PickupStatusReqMsg),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PickupStatusMsg {
    #[serde(rename = "@type")]
    _type: String,
    pub message_count: u32,
    pub recipient_key: String,
}

impl PickupStatusMsg {
    pub fn new(message_count: u32, recipient_key: &str) -> PickupStatusMsg {
        PickupStatusMsg {
            _type: type_uri::PICKUP_STATUS.to_owned(),
            message_count: message_count,
            recipient_key: recipient_key.to_owned(),
        }
    }
    pub fn custom_type(self, _type: String) -> PickupStatusMsg {
        PickupStatusMsg { 
            _type:  _type,
             message_count: self.message_count, 
             recipient_key: self.recipient_key, 
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PickupStatusReqMsg {
    #[serde(rename = "@type")]
    _type: String,
    pub recipient_key: String,
}

impl PickupStatusReqMsg {
    pub fn new(recipient_key: &str) -> PickupStatusReqMsg {
        PickupStatusReqMsg {
            _type: type_uri::PICKUP_STATUS_REQ.to_owned(),
            recipient_key: recipient_key.to_owned(),
        }
    }
    pub fn custom_type(self, _type: String) -> PickupStatusReqMsg {
        PickupStatusReqMsg { 
            _type:  _type,
             recipient_key: self.recipient_key, 
        }
    }
}