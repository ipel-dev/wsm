// src/pool.rs

use std::collections::HashMap;
use std::sync::Mutex;
use serde_json::Value;
use lazy_static::lazy_static;

lazy_static! {
    static ref POOLS: Mutex<HashMap<String, HashMap<String, Option<Value>>>> = Mutex::new(HashMap::new());
}

pub fn create_pool(name: &str) {
    if !name.chars().all(|c| c.is_ascii_alphanumeric()) {
        panic!("Invalid pool name: {}", name);
    }
    let mut pools = POOLS.lock().unwrap();
    if pools.contains_key(name) {
        panic!("Pool already exists: {}", name);
    }
    pools.insert(name.to_string(), HashMap::new());
}

pub fn delete_pool(name: &str) {
    let mut pools = POOLS.lock().unwrap();
    if pools.remove(name).is_none() {
        panic!("Pool not found: {}", name);
    }
}

pub fn add_msg_id(pool_name: &str, msg_id: &str) {
    let mut pools = POOLS.lock().unwrap();
    let pool = pools.get_mut(pool_name).unwrap_or_else(|| panic!("Pool not found: {}", pool_name));
    if pool.contains_key(msg_id) {
        panic!("msg_id already exists in pool {}: {}", pool_name, msg_id);
    }
    pool.insert(msg_id.to_string(), None);
}

pub fn set_msg_json(pool_name: &str, msg_id: &str, json: Value) {
    let mut pools = POOLS.lock().unwrap();
    let pool = pools.get_mut(pool_name).unwrap_or_else(|| panic!("Pool not found: {}", pool_name));
    let entry = pool.get_mut(msg_id).unwrap_or_else(|| panic!("msg_id not found in pool {}: {}", pool_name, msg_id));
    if entry.is_some() {
        panic!("JSON already set for msg_id {} in pool {}", msg_id, pool_name);
    }
    *entry = Some(json);
}

pub fn remove_msg_id(pool_name: &str, msg_id: &str) {
    let mut pools = POOLS.lock().unwrap();
    let pool = pools.get_mut(pool_name).unwrap_or_else(|| panic!("Pool not found: {}", pool_name));
    if pool.remove(msg_id).is_none() {
        panic!("msg_id not found in pool {}: {}", pool_name, msg_id);
    }
}

pub fn msg_count(pool_name: &str) -> usize {
    let pools = POOLS.lock().unwrap();
    let pool = pools.get(pool_name).unwrap_or_else(|| panic!("Pool not found: {}", pool_name));
    pool.len()
}

pub fn get_msg_json(pool_name: &str, msg_id: &str) -> Value {
    let pools = POOLS.lock().unwrap();
    let pool = pools.get(pool_name).unwrap_or_else(|| panic!("Pool not found: {}", pool_name));
    let entry = pool.get(msg_id).unwrap_or_else(|| panic!("msg_id not found in pool {}: {}", pool_name, msg_id));
    entry.clone().unwrap_or(Value::Null)
}

pub fn is_msg_id_available(pool_name: &str, msg_id: &str) -> bool {
    let pools = POOLS.lock().unwrap();
    let pool = pools.get(pool_name).unwrap_or_else(|| panic!("Pool not found: {}", pool_name));
    !pool.contains_key(msg_id) // Returns true if msg_id doesn't exist, false if it does
}

pub fn list_msg_id(pool_name: &str) -> Vec<String> {
    let pools = POOLS.lock().unwrap();
    let pool = pools.get(pool_name).unwrap_or_else(|| panic!("Pool not found: {}", pool_name));
    pool.keys().cloned().collect()
}