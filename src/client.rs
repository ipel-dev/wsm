// src/client.rs

use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use serde_json::Value;

type ClientId = String;
type ClientMeta = HashMap<String, Value>;

lazy_static! {
    static ref CONNECTED_CLIENTS: Mutex<HashMap<ClientId, ClientMeta>> = Mutex::new(HashMap::new());
}

// Register a new client with default metadata
pub fn register_client(client_id: &str) {
    let mut clients = CONNECTED_CLIENTS.lock().unwrap();

    if clients.contains_key(client_id) {
        return; // Already registered
    }

    let mut meta = HashMap::new();
    meta.insert("wsm:wait-for-response".to_string(), Value::Bool(true));
    clients.insert(client_id.to_string(), meta);
}

// Remove a client entry
pub fn unregister_client(client_id: &str) {
    let mut clients = CONNECTED_CLIENTS.lock().unwrap();
    clients.remove(client_id);
}

// Check if a client is valid (exists in registry)
pub fn is_client_valid(client_id: &str) -> bool {
    let clients = CONNECTED_CLIENTS.lock().unwrap();
    clients.contains_key(client_id)
}

// Set or update a metadata field for a client
pub fn set_client_field(client_id: &str, key: &str, value: Value) {
    let mut clients = CONNECTED_CLIENTS.lock().unwrap();
    if let Some(meta) = clients.get_mut(client_id) {
        meta.insert(key.to_string(), value);
    }
}

// Get a metadata field from a client
pub fn get_client_field(client_id: &str, key: &str) -> Option<Value> {
    let clients = CONNECTED_CLIENTS.lock().unwrap();
    clients.get(client_id).and_then(|meta| meta.get(key).cloned())
}
