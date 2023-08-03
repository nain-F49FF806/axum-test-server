// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

use xum_test_server::didcomm_types::ForwardMsg;

const BASE_URL: &str = "http://localhost:7999";

// Test variables
const RECIPIENT_KEY: &str = "Anderson Smith n0r3t1";
const HELLO_ALICE: &str = "Hello Alice!";

// #[ignore]
#[test]
fn test_forward_message() {
    let client = reqwest::blocking::Client::new();
    let endpoint = format!("{BASE_URL}/forward");
    let forward_message = ForwardMsg::new(RECIPIENT_KEY, HELLO_ALICE);
    let res = client.post(endpoint).json(&forward_message).send().unwrap();
    res.error_for_status().unwrap();
}
