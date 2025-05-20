// src/method.rs

// Validates that `method` contains only ASCII lowercase letters or digits.
fn validate_method_name(method: &str) {
    if method.is_empty() || !method.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) {
        panic!(
            "invalid method `{}`: only lowercase letters and digits are allowed",
            method
        );
    }
}

// Validates that `version` is a non-empty string of digits.
fn validate_version_str(version: &str) {
    if version.is_empty() || !version.chars().all(|c| c.is_ascii_digit()) {
        panic!(
            "invalid version `{}`: only integer digits are allowed",
            version
        );
    }
}

// Validates that `endpoint` contains only lowercase letters, digits, '/' or '_'.
fn validate_endpoint_str(endpoint: &str) {
    if endpoint.is_empty()
        || !endpoint.chars().all(|c| {
            c.is_ascii_lowercase()
                || c.is_ascii_digit()
                || c == '/'
                || c == '_'
        })
    {
        panic!(
            "invalid endpoint `{}`: only a–z, 0–9, '/' and '_' are allowed",
            endpoint
        );
    }
}

// Builds a fully-qualified method string of the form `{method}@v{version}/{endpoint}`
// after validating each component.
pub fn build_method(method: &str, version: &str, endpoint: &str) -> String {
    validate_method_name(method);
    validate_version_str(version);
    validate_endpoint_str(endpoint);

    format!("{}@v{}/{}", method, version, endpoint)
}