// Copyright 2023 Naian G.
// SPDX-License-Identifier: Apache-2.0

use serde_json::json;
const BASE_URL: &str = "http://localhost:7999";

// Test variables
const RECIPIENT_KEY: &str = "Anderson Smith n0r3t1";

#[test]
fn test_status_request_endpoint_exists() {
    let client = reqwest::blocking::Client::new();
    let endpoint = format!("{BASE_URL}/pickup");

    let (id, recipient_key) = (123, RECIPIENT_KEY);
    let status_request = json!(
        {
            "@id": id,
            "@type": "https://didcomm.org/messagepickup/2.0/status-request",
            "recipient_key": recipient_key
        }
    );
    let res = client.post(endpoint).json(&status_request).send().unwrap();
    res.error_for_status().unwrap();
}

#[test]
fn test_status_request_returns_a_valid_status() {
    let client = reqwest::blocking::Client::new();
    let endpoint = format!("{BASE_URL}/pickup");

    let status_request = json!(
        {
            "@id": 123,
            "@type": "https://didcomm.org/messagepickup/2.0/status-request",
        }
    );
    let res = client.post(endpoint).json(&status_request).send().unwrap();
    let res_msg = res.json::<serde_json::Value>().unwrap();
    assert_eq!(
        "https://didcomm.org/messagepickup/2.0/status",
        res_msg["@type"]
    );
    assert!(res_msg.get("message_count").is_some());
}

#[test]
fn test_status_request_for_key_returns_a_valid_status() {
    let client = reqwest::blocking::Client::new();
    let endpoint = format!("{BASE_URL}/pickup");

    let status_request = json!(
        {
            "@id": 123,
            "@type": "https://didcomm.org/messagepickup/2.0/status-request",
            "recipient_key": RECIPIENT_KEY
        }
    );
    let res = client.post(endpoint).json(&status_request).send().unwrap();
    let res_msg = res.json::<serde_json::Value>().unwrap();
    assert_eq!(
        "https://didcomm.org/messagepickup/2.0/status",
        res_msg["@type"]
    );
    assert!(res_msg.get("message_count").is_some());
    assert_eq!(RECIPIENT_KEY, res_msg["recipient_key"]);
}

// {
//     "@id": "123456781",
//     "@type": "https://didcomm.org/messagepickup/2.0/status",
//     "recipient_key": "<key for messages>",
//     "message_count": 7,
//     "longest_waited_seconds": 3600,
//     "newest_received_time": "2019-05-01 12:00:00Z",
//     "oldest_received_time": "2019-05-01 12:00:01Z",
//     "total_bytes": 8096,
//     "live_delivery": false
// }

#[ignore]
#[test]
fn test_delivery_request_returns_status_when_queue_empty() {
    let client = reqwest::blocking::Client::new();
    let endpoint = format!("{BASE_URL}/pickup");
    let delivery_req = json!(
        {
            "@id": "123456781",
            "@type": "https://didcomm.org/messagepickup/2.0/delivery-request",
            "limit": 10,
            "recipient_key": "<key for messages>"
        }
    );

    let res = client.post(endpoint).json(&delivery_req).send().unwrap();
    if let Err(err) = res.error_for_status_ref() {
        panic!("Error response status {:#?}", err);
    }
    let res_msg = res.json::<serde_json::Value>().unwrap();
    assert_eq!(
        "https://didcomm.org/messagepickup/2.0/status",
        res_msg["@type"]
    );
    assert_eq!(0, res_msg["message_count"]);
}

#[ignore]
#[test]
fn test_delivery_request() {
    let client = reqwest::blocking::Client::new();
    let endpoint = format!("{BASE_URL}/pickup");

    let delivery_request = json!(
        {
            "@id": 123,
            "@type": "https://didcomm.org/messagepickup/2.0/delivery-request",
            "limit": 10
        }
    );
    let res = client
        .post(endpoint)
        .json(&delivery_request)
        .send()
        .unwrap();
    if let Err(err) = res.error_for_status_ref() {
        panic!("Error response status {:#?}", err);
    }
    let res_msg = res.json::<serde_json::Value>().unwrap();
    assert_eq!(
        "https://didcomm.org/messagepickup/2.0/delivery",
        res_msg["@type"]
    );
    // assert_ne!(0, res_msg["message_count"]);
}
// {
//     "@id": "123456781",
//     "@type": "https://didcomm.org/messagepickup/2.0/delivery-request",
//     "limit": 10,
//     "recipient_key": "<key for messages>"
// }

// {
//     "@type": "https://didcomm.org/messagepickup/2.0/delivery-request",
//     "limit": 1
// }

// {
//     "@id": "123456781",
//     "~thread": {
//         "thid": "<message id of delivery-request message>"
//       },
//     "@type": "https://didcomm.org/messagepickup/2.0/delivery",
//     "recipient_key": "<key for messages>",
//     "~attach": [{
//     	"@id": "<messageid>",
//     	"data": {
//     		"base64": ""
//     	}
//     }]
// }

// {
//     "@type": "https://didcomm.org/messagepickup/2.0/messages-received",
//     "message_id_list": ["123","456"]
// }

// Multiple Recipients

// // If a message arrives at a Mediator addressed to multiple Recipients,
// // the message MUST be queued for each Recipient independently.
// // If one of the addressed Recipients retrieves a message and indicates it has been received,
// // that message MUST still be held and then removed by the other addressed Recipients.

// {
//     "@type": "https://didcomm.org/messagepickup/2.0/live-delivery-change",
//     "live_delivery": true
// }

// {
//     "@type": "https://didcomm.org/notification/1.0/problem-report",
//     "~thread": {
//       "pthid": "<message id of offending live_delivery_change>"
//     },
//     "description": "Connection does not support Live Delivery"
// }
