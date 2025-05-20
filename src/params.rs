// src/params.rs

use serde_json::Value;

// Constructs a JSON array `Value` from one or more string-like items.
// 
// Panics if the iterator yields no items.
// 
// # Examples
// 
// ```
// use serde_json::json;
// let v = build_params(vec!["foo"]);
// assert_eq!(v, json!(["foo"]));
//
// let v = build_params(&["a", "b", "c"]);
// assert_eq!(v, json!(["a", "b", "c"]));
// ```
pub fn build_params<I, S>(items: I) -> Value
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let arr: Vec<Value> = items
        .into_iter()
        .map(|s| Value::String(s.as_ref().to_owned()))
        .collect();

    if arr.is_empty() {
        panic!("build_params: at least one string is required");
    }

    Value::Array(arr)
}