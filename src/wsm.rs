// src/wsm.rs

use crate::server::{new_client, new_client_msg_pool};
use crate::wsa::create_request_form_server_to_client;
use crate::id::gen_unique_id;
use crate::method::build_method;
use crate::params::build_params;
use crate::pool::{add_msg_id, set_msg_json};
use crate::client::{register_client, is_client_valid};
use serde_json::Value;

// Initialize a new client connection, create its msg pool, and build handshake JSON.
// Returns: (JSON object to send, client_id)
pub fn init_client_connection() -> (Value, String) {
    let client_id = new_client();
    new_client_msg_pool(&client_id);

    let msg_id = gen_unique_id(&client_id);
    add_msg_id(&client_id, &msg_id);

    let method = build_method("wsm", "1", "handshake");
    let params = build_params(vec![&client_id]);

    let msg_string = create_request_form_server_to_client(
        &client_id,
        &msg_id,
        &method,
        params,
    );

    let msg_json: Value = serde_json::from_str(&msg_string).expect("Invalid JSON");

    set_msg_json(&client_id, &msg_id, msg_json.clone());
    register_client(&client_id);

    (msg_json, client_id)
}

// Send a request from server to a client
pub fn server_request_client(
    client_id: &str,
    method: &str,
    version: &str,
    endpoint: &str,
    params: Vec<&str>,
) -> (Value, String) {
    assert!(is_client_valid(client_id), "Client not valid");

    let msg_id = gen_unique_id(client_id);
    add_msg_id(client_id, &msg_id);

    let full_method = build_method(method, version, endpoint);
    let param_value = build_params(params);

    let json_str = create_request_form_server_to_client(client_id, &msg_id, &full_method, param_value);
    let json_value: Value = serde_json::from_str(&json_str).expect("Invalid JSON format");

    set_msg_json(client_id, &msg_id, json_value.clone());

    (json_value, msg_id)
}