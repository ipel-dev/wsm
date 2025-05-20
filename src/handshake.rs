// src/handshake.rs

use crate::client::get_client_field;
use crate::handler::{trigger_disconnect, trigger_auth};
use serde_json::Value;
use crate::client::unregister_client;
use crate::pool::delete_pool;
use crate::client::set_client_field;

// Return handshake status of a client: "wait-for-response" or "active"
pub fn get_client_handshake_status(client_id: &str) -> &'static str {
    match get_client_field(client_id, "wsm:wait-for-response") {
        Some(Value::Bool(true)) => "wait-for-response",
        Some(Value::Bool(false)) => "active",
        _ => "wait-for-response",
    }
}

// Process handshake authentication response from client
pub fn handle_handshake_auth(json: &Value, client_id: &str) {
    let payload = match json.get("p") {
        Some(Value::Object(map)) => map,
        _ => {
            trigger_disconnect(client_id);
            return;
        }
    };

    let r = payload.get("r").and_then(|v| v.as_str());
    let c = payload.get("c").and_then(|v| v.as_str());

    // check "r"
    match r {
        Some("s") => {
            // valid success response, continue
        }
        _ => {
            trigger_disconnect(client_id);
            return;
        }
    }

    // check "c"
    match c {
        Some(code) if is_base64(code) => {
            if trigger_auth(client_id, code) {
                // mark client as active
                set_client_field(client_id, "wsm:wait-for-response", Value::Bool(false));
            } else {
                trigger_disconnect(client_id);
                unregister_client(client_id);
                delete_pool(client_id);
            }
        }
        Some("Anonymous") => {
            // anonymous access: mark active and add "anonymous" flag
            set_client_field(client_id, "wsm:wait-for-response", Value::Bool(false));
            set_client_field(client_id, "anonymous", Value::Bool(true));
        }
        _ => {
            trigger_disconnect(client_id);
            return;
        }
    }
}

// Check if string looks like base64 (strictly [A-Za-z0-9=] only)
fn is_base64(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphanumeric() || c == '=')
}
