// src/socket.rs

use serde_json::Value;
use crate::handler::{trigger_disconnect, notify_client_dropped, trigger_client_request};
use crate::pool::{is_msg_id_available, get_msg_json, remove_msg_id};
use crate::client::{is_client_valid, unregister_client};
use crate::handshake::{get_client_handshake_status, handle_handshake_auth};
use crate::process::process_response_from_client;
use crate::method::parse_method_string;

// protocol message handler
pub fn handle_wsm_message(json: Value, client_id: &str) {
    if !validate_client_message(&json, client_id) {
        return;
    }

    match json.get("y").and_then(|v| v.as_str()) {
        Some("r") => handle_wsm_response(json, client_id),
        Some("g") => handle_wsm_request(json, client_id),
        _ => {
            // (Safe) Unknown message type, ignore or disconnect if needed
        }
    }
}

pub fn handle_wsm_request(json: Value, client_id: &str) {
    let msg_id = match json.get("i").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return,
    };

    let p = match json.get("p") {
        Some(Value::Object(obj)) => obj,
        _ => return,
    };

    let method_str = match p.get("m").and_then(|v| v.as_str()) {
        Some(m) => m,
        None => return,
    };

    let params = match p.get("p") {
        Some(v @ Value::Array(_)) => v.clone(),
        _ => return,
    };

    let (method, version, endpoint) = match parse_method_string(method_str) {
        Some(t) => t,
        None => return,
    };

    trigger_client_request(
        client_id,
        msg_id,
        &method,
        &version,
        &endpoint,
        params,
    );
}

// protocol message handler
pub fn handle_wsm_response(json: Value, client_id: &str) {
    match json.get("t").and_then(|v| v.as_str()) {
        Some("s") => {
            match get_client_handshake_status(client_id) {
                "group" => {
                    // handle response from already active and login client
                    process_response_from_client(json, client_id);
                    
                }
                "unauthorized" => {
                    handle_handshake_auth(&json, client_id);
                }
                _ => {
                    trigger_disconnect(client_id);
                    unregister_client(client_id);
                    remove_msg_id("server", client_id);
                    notify_client_dropped(client_id);
                    // fallback safety
                    return;
                }
            }
        }
        _ => {
            return;
        }
    }
}

// Master validator: composed of individual checks
fn validate_client_message(json: &Value, client_id: &str) -> bool {
    check_msg_id(json, client_id)
        && check_from_field(json, client_id)
        && check_msg_id_exists(json, client_id)
        && check_to_field(json, client_id)
}

// Check that "f" matches client_id
fn check_from_field(json: &Value, client_id: &str) -> bool {
    match json.get("f").and_then(|v| v.as_str()) {
        Some(f) if f == client_id => true,
        _ => {
            trigger_disconnect(client_id);
            unregister_client(client_id);
            remove_msg_id("server", client_id);
            notify_client_dropped(client_id);
            false
        }
    }
}

// Check that "i" field exists and is a string
fn check_msg_id(json: &Value, client_id: &str) -> bool {
    json.get("i")
        .and_then(|v| v.as_str())
        .is_some()
        .then_some(())
        .is_some()
        || {
            trigger_disconnect(client_id);
            unregister_client(client_id);
            remove_msg_id("server", client_id);
            notify_client_dropped(client_id);
            false
        }
}

// Check that msg_id exists in pool (i.e. response is to something we sent)
fn check_msg_id_exists(json: &Value, client_id: &str) -> bool {
    let msg_id = match json.get("i").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return false,
    };

    if is_msg_id_available(client_id, msg_id) {
        trigger_disconnect(client_id);
        unregister_client(client_id);
        remove_msg_id("server", client_id);
        notify_client_dropped(client_id);
        return false;
    }

    let original = get_msg_json(client_id, msg_id);

    if original.is_null() {
        trigger_disconnect(client_id);
        unregister_client(client_id);
        remove_msg_id("server", client_id);
        notify_client_dropped(client_id);
        return false;
    }

    let y = original.get("y").and_then(|v| v.as_str());
    let t = original.get("t").and_then(|v| v.as_str());

    match (y, t) {
        (Some("g"), Some(to)) if to == client_id => {
            true
        }
        (Some("e"), Some("c")) => {
            true
        }
        _ => {
            trigger_disconnect(client_id);
            unregister_client(client_id);
            remove_msg_id("server", client_id);
            notify_client_dropped(client_id);
            false
        }
    }
}

// Check that "t" is either "s" or a valid client_id
fn check_to_field(json: &Value, client_id: &str) -> bool {
    let msg_id = match json.get("i").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => {
            trigger_disconnect(client_id);
            unregister_client(client_id);
            remove_msg_id("server", client_id);
            notify_client_dropped(client_id);
            return false;
        }
    };

    match json.get("t").and_then(|v| v.as_str()) {
        Some("s") => true,
        Some(to) if to.len() == 5 && to.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) => {
            if is_client_valid(to) {
                true
            } else {
                trigger_disconnect(client_id);
                unregister_client(client_id);
                remove_msg_id("server", client_id);
                remove_msg_id(client_id, msg_id);
                notify_client_dropped(client_id);
                false
            }
        }
        _ => {
            trigger_disconnect(client_id);
            unregister_client(client_id);
            remove_msg_id("server", client_id);
            remove_msg_id(client_id, msg_id);
            notify_client_dropped(client_id);
            false
        }
    }
}