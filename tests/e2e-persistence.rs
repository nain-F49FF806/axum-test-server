// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

use serde_json::json;
use std::sync::Once;
static INIT: Once = Once::new();

const BASE_URL: &str = "http://localhost:7999";

// Test variables
const RECIPIENT_KEY: &str = "Anderson Smith n0r3t1";

fn setup_account() {
    let client = reqwest::blocking::Client::new();
    let endpoint = format!("{BASE_URL}/coord");
    let new_account_req = json!(
        {
            "@type": "https://didcomm.org/coordinate-mediation/1.0/mediate-request",
            "auth_pubkey": RECIPIENT_KEY
        }
    );
    let res = client.post(endpoint).json(&new_account_req).send().unwrap();
    res.error_for_status().unwrap();
}

pub fn initialize() {
    INIT.call_once(|| {
        setup_account();
    });
}

// #[ignore]
#[test]
fn test_forward_message() {
    initialize();
    let client = reqwest::blocking::Client::new();
    let endpoint = format!("{BASE_URL}/forward");
    let forward_message = json!(
        {
            "@type" : "https://didcomm.org/routing/1.0/forward",
            "@id": "54ad1a63-29bd-4a59-abed-1c5b1026e6fd",
            "to"   : RECIPIENT_KEY,
            "msg"  : "<super secret packed DIDCOMM message>"
        }
    );
    let res = client.post(endpoint).json(&forward_message).send().unwrap();
    res.error_for_status().unwrap();
}
