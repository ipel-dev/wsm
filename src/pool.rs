use std::collections::HashMap;
use std::sync::Mutex;
use serde_json::Value;
use lazy_static::lazy_static;

lazy_static! {
    static ref POOLS: Mutex<HashMap<String, HashMap<String, Option<Value>>>> = Mutex::new(HashMap::new());
}

pub fn create_pool(name: &str) {
    if !name.chars().all(|c| c.is_ascii_alphanumeric()) {
        return;
    }
    let mut pools = POOLS.lock().unwrap();
    if pools.contains_key(name) {
        return;
    }
    pools.insert(name.to_string(), HashMap::new());
}

pub fn delete_pool(name: &str) {
    let mut pools = POOLS.lock().unwrap();
    pools.remove(name);
}

pub fn add_msg_id(pool_name: &str, msg_id: &str) {
    let mut pools = POOLS.lock().unwrap();
    if let Some(pool) = pools.get_mut(pool_name) {
        if pool.contains_key(msg_id) {
            return;
        }
        pool.insert(msg_id.to_string(), None);
    }
}

pub fn set_msg_json(pool_name: &str, msg_id: &str, json: Value) {
    let mut pools = POOLS.lock().unwrap();
    if let Some(pool) = pools.get_mut(pool_name) {
        if let Some(entry) = pool.get_mut(msg_id) {
            if entry.is_none() {
                *entry = Some(json);
            }
        }
    }
}

pub fn remove_msg_id(pool_name: &str, msg_id: &str) {
    let mut pools = POOLS.lock().unwrap();
    if let Some(pool) = pools.get_mut(pool_name) {
        pool.remove(msg_id);
    }
}

pub fn msg_count(pool_name: &str) -> usize {
    let pools = POOLS.lock().unwrap();
    pools.get(pool_name).map(|pool| pool.len()).unwrap_or(0)
}

pub fn get_msg_json(pool_name: &str, msg_id: &str) -> Value {
    let pools = POOLS.lock().unwrap();
    pools
        .get(pool_name)
        .and_then(|pool| pool.get(msg_id))
        .and_then(|entry| entry.clone())
        .unwrap_or(Value::Null)
}

pub fn is_msg_id_available(pool_name: &str, msg_id: &str) -> bool {
    let pools = POOLS.lock().unwrap();
    pools
        .get(pool_name)
        .map(|pool| !pool.contains_key(msg_id))
        .unwrap_or(true)
}

pub fn list_msg_id(pool_name: &str) -> Vec<String> {
    let pools = POOLS.lock().unwrap();
    pools
        .get(pool_name)
        .map(|pool| pool.keys().cloned().collect())
        .unwrap_or_else(Vec::new)
}
