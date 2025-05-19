// src/message.rs

use serde_json::Value;
use wsa::{build_request, build_response, build_event};

fn validate_party(name: &str) {
    if !(name == "server" || name == "client" || (name.len() == 5 && name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()))) {
        panic!("Invalid party: {}", name);
    }
}

pub fn create_request(from: &str, to: &str, msg_id: &str, method: &str, params: Value) -> String {
    validate_party(from);
    validate_party(to);
    build_request(from, to, msg_id, method, params)
}

pub fn create_event(msg_id: &str, method: &str, params: Value) -> String {
    build_event("server", "client", &msg_id, method, params)
}

pub fn create_success_response(from: &str, to: &str, msg_id: &str, receipt: &str) -> String {
    validate_party(from);
    validate_party(to);
    build_response(from, to, msg_id, "success", receipt)
}

pub fn create_fail_response(from: &str, to: &str, msg_id: &str, err_code: &str) -> String {
    validate_party(from);
    validate_party(to);
    build_response(from, to, msg_id, "fail", err_code)
}
