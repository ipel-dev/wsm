// src/__init__.rs

use crate::pool::create_pool;

pub fn setup() {
    create_pool("server");
    create_pool("client");
}
