// src/wsa.rs

use crate::message;
use serde_json::Value;

fn validate_id(id: &str) {
    let valid = id.len() == 5
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit());
    if !valid {
        panic!(
            "invalid id `{}`: must be exactly 5 lowercase letters or digits",
            id
        );
    }
}

fn validate_msg_id(msg_id: &str) {
    validate_id(msg_id);
}

fn validate_client_id(client_id: &str) {
    if client_id == "client" {
        panic!(
            "invalid id `{}`: must be exactly one client",
            client_id
        );
    }
    validate_id(client_id);
}


// Validates that `full` matches the pattern `{method}@v{version}/{endpoint}`,
// where:
// - `method` is non-empty ASCII lowercase letters or digits,
// - `version` is non-empty digits,
// - `endpoint` is non-empty and contains only lowercase letters, digits, `/` or `_`.
fn validate_full_method(full: &str) {
    let mut at_split = full.splitn(2, '@');
    let before_at = at_split.next().unwrap_or("");
    let after_at = at_split.next().expect("invalid format: missing `@`");
    if before_at.is_empty() {
        panic!("invalid format: method part is empty");
    }

    // make sure '@' follow-up 'v'
    if !after_at.starts_with('v') {
        panic!("invalid format: expected `v` after `@`");
    }
    let after_v = &after_at[1..];

    // split version and endpoint
    let mut slash_split = after_v.splitn(2, '/');
    let version = slash_split.next().unwrap_or("");
    let endpoint = slash_split.next().expect("invalid format: missing `/`");
    if version.is_empty() {
        panic!("invalid format: version part is empty");
    }
    if endpoint.is_empty() {
        panic!("invalid format: endpoint part is empty");
    }
}

fn validate_err_code(err_code: &str) {
    if err_code.is_empty() || !err_code.chars().all(|c| {
        c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'
    }) {
        panic!(
            "invalid err_code `{}`: only A–Z, 0–9, and '_' are allowed",
            err_code
        );
    }
}

pub fn create_request_form_server_to_client(
    to: &str,
    msg_id: &str,
    method: &str,
    params: Value,
) -> String {
    validate_msg_id(msg_id);
    validate_client_id(to);
    validate_full_method(method);
    message::create_request("server", to, msg_id, method, params)
}

pub fn create_request_form_client_to_server(
    from: &str,
    msg_id: &str,
    method: &str,
    params: Value,
) -> String {
    validate_msg_id(msg_id);
    validate_client_id(from);
    validate_full_method(method);
    message::create_request(from, "server", msg_id, method, params)
}

pub fn create_request_form_client_to_client(
    from: &str,
    to: &str,
    msg_id: &str,
    method: &str,
    params: Value,
) -> String {
    validate_msg_id(msg_id);
    validate_client_id(from);
    validate_client_id(to);
    validate_full_method(method);
    message::create_request(from, to, msg_id, method, params)
}

pub fn create_event_form_server_to_client(
    msg_id: &str,
    method: &str,
    params: Value,
) -> String {
    validate_msg_id(msg_id);
    validate_full_method(method);
    message::create_event(msg_id, method, params)
}

pub fn create_success_response_form_server_to_client(
    to: &str,
    msg_id: &str,
    receipt: &str,
) -> String {
    validate_msg_id(msg_id);
    validate_client_id(to);
    message::create_success_response("server", to, msg_id, receipt)
}

pub fn create_fail_response_form_server_to_client(
    to: &str,
    msg_id: &str,
    err_code: &str,
) -> String {
    validate_msg_id(msg_id);
    validate_client_id(to);
    validate_err_code(err_code);
    message::create_fail_response("server", "to", msg_id, err_code)
}

pub fn create_success_response_form_client_to_server(
    from: &str,
    msg_id: &str,
    receipt: &str,
) -> String {
    validate_msg_id(msg_id);
    validate_client_id(from);
    message::create_success_response(from, "server", msg_id, receipt)
}

pub fn create_fail_response_form_client_to_server(
    from: &str,
    msg_id: &str,
    err_code: &str,
) -> String {
    validate_msg_id(msg_id);
    validate_client_id(from);
    validate_err_code(err_code);
    message::create_fail_response(from, "server", msg_id, err_code)
}

pub fn create_success_response_form_client_to_client(
    from: &str,
    to: &str,
    msg_id: &str,
    receipt: &str,
) -> String {
    validate_msg_id(msg_id);
    validate_client_id(from);
    validate_client_id(to);
    message::create_success_response(from, to, msg_id, receipt)
}

pub fn create_fail_response_form_client_to_client(
    from: &str,
    to: &str,
    msg_id: &str,
    err_code: &str,
) -> String {
    validate_msg_id(msg_id);
    validate_client_id(from);
    validate_client_id(to);
    validate_err_code(err_code);
    message::create_fail_response(from, to, msg_id, err_code)
}

