use std::sync::{Mutex, Arc};
use lazy_static::lazy_static;

type DisconnectHandler = Arc<dyn Fn(&str) + Send + Sync>;
type AuthHandler = Arc<dyn Fn(&str, &str) -> (bool, String) + Send + Sync>;

lazy_static! {
    static ref DISCONNECT_HANDLER: Mutex<Option<DisconnectHandler>> = Mutex::new(None);
    static ref AUTH_HANDLER: Mutex<Option<AuthHandler>> = Mutex::new(None);
}

// Register a disconnect handler which will be called with client_id.
pub fn register_disconnect_handler<F>(f: F)
where
    F: Fn(&str) + Send + Sync + 'static,
{
    let mut handler = DISCONNECT_HANDLER.lock().unwrap();
    *handler = Some(Arc::new(f));
}

// Call the registered disconnect handler if exists.
pub fn trigger_disconnect(client_id: &str) {
    if let Some(handler) = &*DISCONNECT_HANDLER.lock().unwrap() {
        handler(client_id);
    }
}

// Register an authentication handler which receives (client_id, base64) and returns (ok, group)
pub fn register_auth_handler<F>(f: F)
where
    F: Fn(&str, &str) -> (bool, String) + Send + Sync + 'static,
{
    let mut handler = AUTH_HANDLER.lock().unwrap();
    *handler = Some(Arc::new(f));
}

// Trigger the registered auth handler, returns (ok, group)
pub fn trigger_auth(client_id: &str, code: &str) -> (bool, String) {
    if let Some(handler) = &*AUTH_HANDLER.lock().unwrap() {
        handler(client_id, code)
    } else {
        (false, String::from("unauthorized"))
    }
}
