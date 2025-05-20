// src/socket.rs

use serde_json::Value;
use crate::handler::trigger_disconnect;
use crate::pool::is_msg_id_available;
use crate::client::{is_client_valid, unregister_client};

// protocol message handler
pub fn handle_wsm_message(json: Value, client_id: &str) {
    // get msg_id "i"
    let msg_id = match json.get("i").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => {
            trigger_disconnect(client_id);
            return;
        }
    };

    // check "f" (from) field
    match json.get("f").and_then(|v| v.as_str()) {
        Some(f) if f == client_id => {} // valid
        _ => {
            trigger_disconnect(client_id);
            unregister_client(client_id);
            return;
        }
    }

    // check msg_id in server client pool is vaild or not
    if is_msg_id_available(client_id, msg_id) {
        trigger_disconnect(client_id);
        return;
    }

    // check "t" (to) field
    match json.get("t").and_then(|v| v.as_str()) {
        Some("s") => {} // to server, always allowed
        Some(to) if to.len() == 5 && to.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) => {
            if !is_client_valid(to) {
                trigger_disconnect(client_id);
                unregister_client(client_id);
                return;
            }
        }
        _ => {
            trigger_disconnect(client_id);
            unregister_client(client_id);
            return;
        }
    }

    // TODO: handle valid response
}
