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
    register_client(&client_id); // handshake status -> wait-for-response
    (msg_json, client_id)
}
