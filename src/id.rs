// src/id.rs

use rand::{thread_rng, Rng};
use crate::pool::is_msg_id_available;

pub fn gen_id() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    const ID_LEN: usize = 5;

    let mut rng = thread_rng();
    (0..ID_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Generate a unique ID that doesn't exist in the given pool.
pub fn gen_unique_id(pool_name: &str) -> String {
    loop {
        let id = gen_id();
        if is_msg_id_available(pool_name, &id) {
            return id;
        }
    }
}
