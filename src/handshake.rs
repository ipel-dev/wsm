// src/handshake.rs

use crate::client::get_client_field;
use serde_json::Value;

// Return handshake status of a client: "wait-for-response" or "active"
pub fn get_client_handshake_status(client_id: &str) -> &'static str {
    match get_client_field(client_id, "wsm:wait-for-response") {
        Some(Value::Bool(true)) => "wait-for-response",
        Some(Value::Bool(false)) => "active",
        _ => "wait-for-response", // default to waiting if missing or invalid
    }
}
