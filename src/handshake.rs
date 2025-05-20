// src/handshake.rs

use crate::client::get_client_field;
use crate::handler::{trigger_disconnect, trigger_auth};
use serde_json::Value;
use crate::client::unregister_client;
use crate::pool::{delete_pool, remove_msg_id};
use crate::client::set_client_field;

// Return handshake status of a client: "unauthorized" or "group"
pub fn get_client_handshake_status(client_id: &str) -> &'static str {
    match get_client_field(client_id, "unauthorized") {
        Some(Value::Bool(false)) => "group",
        Some(Value::Bool(true)) => "unauthorized",
        _ => "unauthorized",
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
        Some("s") => {}
        _ => {
            trigger_disconnect(client_id);
            unregister_client(client_id);
            delete_pool(client_id);
            return;
        }
    }

    // extract msg_id
    let msg_id = match json.get("i").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => {
            trigger_disconnect(client_id);
            unregister_client(client_id);
            delete_pool(client_id);
            return;
        }
    };

    match c {
        Some(code) if is_base64(code) => {
            let (ok, group) = trigger_auth(client_id, code);
            if ok {
                set_client_field(client_id, "unauthorized", Value::Bool(false));
                set_client_field(client_id, "group", Value::String(group));
                remove_msg_id(client_id, msg_id);
            } else {
                trigger_disconnect(client_id);
                unregister_client(client_id);
                delete_pool(client_id);
            }
        }
        Some("Anonymous") => {
            set_client_field(client_id, "unauthorized", Value::Bool(false));
            set_client_field(client_id, "group", Value::String("anonymous".into()));
            remove_msg_id(client_id, msg_id);
        }
        _ => {
            trigger_disconnect(client_id);
            unregister_client(client_id);
            delete_pool(client_id);
        }
    }
}

// Check if string looks like base64 (strictly [A-Za-z0-9=] only)
fn is_base64(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphanumeric() || c == '=')
}
