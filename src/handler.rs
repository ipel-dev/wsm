use std::sync::{Mutex, Arc};
use lazy_static::lazy_static;

type DisconnectHandler = Arc<dyn Fn(&str) + Send + Sync>;

lazy_static! {
    static ref DISCONNECT_HANDLER: Mutex<Option<DisconnectHandler>> = Mutex::new(None);
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
