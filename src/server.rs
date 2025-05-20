// src/server.rs

use crate::id::gen_id;
use crate::pool::{is_msg_id_available, add_msg_id, create_pool};

/// Generate a new client ID and register it in the "client" pool.
pub fn new_client() -> String {
    let client_pool = "server"; //client id save at server side pool

    // Retry loop until we get a unique ID
    loop {
        let id = gen_id();
        if is_msg_id_available(client_pool, &id) {
            add_msg_id(client_pool, &id);
            return id;
        }
    }
}

pub fn new_client_msg_pool(client_id: &str) {
    create_pool(client_id); // create a msg_pool only for this client's communication
}