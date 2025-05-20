// src/pretty.rs

use serde_json::Value;

pub fn pretty_print(json_str: &str) {
    let v: Value = serde_json::from_str(json_str)
        .expect("invalid JSON");
    let s = serde_json::to_string_pretty(&v)
        .expect("fail to pretty print JSON");
    println!("{}", s);
}