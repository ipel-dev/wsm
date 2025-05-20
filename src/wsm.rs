// src/wsm.rs

use crate::server::{new_client, new_client_msg_pool};
use crate::wsa::create_request_form_server_to_client;
use crate::id::gen_unique_id;
use crate::method::build_method;
use crate::params::build_params;
use crate::pool::{add_msg_id, set_msg_json};
use crate::client::register_client;
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

    let msg_json = create_request_form_server_to_client(
        "server",
        &client_id,
        &msg_id,
        &method,
        &params,
    );

    set_msg_json(&client_id, &msg_id, msg_json.clone());
    register_client(&client_id); // handshake status -> unauthorized
    (msg_json, client_id)
}

// Send a request from server to a client
pub fn server_request_client(
    client_id: &str,
    method: &str,
    version: &str,
    endpoint: &str,
    params: Vec<&str>,
    callback_key: &str,
) -> (Value, String) {
    assert!(is_client_valid(client_id), "Client not valid");

    let msg_id = gen_unique_id(client_id);
    add_msg_id(client_id, &msg_id);

    let full_method = build_method(method, version, endpoint);
    let json_str = create_request_form_server_to_client(client_id, &msg_id, &full_method, params);

    // store in pool
    set_msg_json(client_id, &msg_id, json_str);

    // save callback_key to registry
    CALLBACK_REGISTRY.lock().unwrap().insert(msg_id.clone(), callback_key.to_string());

    (json_str, msg_id)
}