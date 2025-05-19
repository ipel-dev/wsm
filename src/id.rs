// src/id.rs

use rand::{thread_rng, Rng};

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
