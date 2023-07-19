// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};


pub mod TypeURI {
    pub const Forward: &str = "https://didcomm.org/routing/1.0/forward";
    pub const PickupStatusReq: &str = "https://didcomm.org/messagepickup/2.0/status";
    pub const PickupDeliveryReq: &str = "https://didcomm.org/messagepickup/2.0/delivery-request";
    pub const PickupReceived: &str = "https://didcomm.org/messagepickup/2.0/messages-received";
    pub const PickupStatus: &str = "https://didcomm.org/messagepickup/2.0/status";
    pub const PickupDelivery: &str = "https://didcomm.org/messagepickup/2.0/delivery";

}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ForwardMsg {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "to")]
    pub recipient_key: String,
    pub msg: String,
}

impl ForwardMsg {
    pub fn default_alice() -> ForwardMsg {
        ForwardMsg { 
            _type: TypeURI::Forward.to_owned(), 
            recipient_key: "Alice".to_owned(),
            msg: "Hello!".to_owned()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusMsg {
    #[serde(rename = "@type")]
    pub _type: String,
    pub message_count: u32,
    pub recipient_key: String,
}