// src/process.rs

use serde_json::Value;
use crate::handler::trigger_response_callback;
use crate::pool::remove_msg_id;

/// Process a valid response JSON from client
pub fn process_response_from_client(json: Value, client_id: &str) {
    let msg_id = match json.get("i").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return,
    };

    let payload = match json.get("p").and_then(|v| v.as_object()) {
        Some(p) => p,
        None => return,
    };

    let r = match payload.get("r").and_then(|v| v.as_str()) {
        Some(r) => r,
        None => return,
    };

    let c = match payload.get("c").and_then(|v| v.as_str()) {
        Some(c) => c,
        None => "",
    };

    let success = r == "s";
    remove_msg_id(client_id, msg_id);
    trigger_response_callback(client_id, msg_id, success, c);

}
