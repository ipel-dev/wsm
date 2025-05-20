// src/main.rs

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use std::net::SocketAddr;

// Import modules from the crate
pub mod callback;
pub mod client;
pub mod handler; // Ensure handler module is declared
pub mod handshake;
pub mod id;
pub mod message;
pub mod method;
pub mod params;
pub mod pool;
pub mod pretty;
pub mod process;
pub mod server;
pub mod setup;
pub mod socket;
pub mod wsa;
pub mod wsm;

// Use items from local modules
 // For registering app-level handlers
use crate::setup::setup;
use crate::socket::handle_wsm_message; // Crate's function to process incoming messages
use crate::wsm::init_client_connection; // Crate's function for initial client setup data

// Publicly use items from wsa module
pub use wsa::{
    create_event_form_server_to_client, create_fail_response_form_client_to_client,
    create_fail_response_form_client_to_server, create_fail_response_form_server_to_client,
    create_request_form_client_to_client, create_request_form_client_to_server,
    create_request_form_server_to_client, create_success_response_form_client_to_client,
    create_success_response_form_client_to_server, create_success_response_form_server_to_client,
};

// --- Define App-Level Callback Implementations ---

fn app_level_disconnect_handler(client_id: &str) {
    println!("[App Callback] DisconnectHandler: Client '{}' disconnected.", client_id);
    // TODO: Implement application-specific logic when a client disconnects.
    // This is called by the crate when it internally triggers a disconnect.
}

fn app_level_auth_handler(client_id: &str, auth_code: &str) -> (bool, String) {
    println!("[App Callback] AuthHandler: Client '{}' attempting auth with code '{}'.", client_id, auth_code);
    // TODO: Implement application-specific authentication logic.
    // For this example, always succeed and assign to "default_group".
    let is_ok = !auth_code.is_empty(); // Example: allow if auth_code is not empty
    let group = if is_ok { "authenticated_group".to_string() } else { "failed_auth_group".to_string() };
    println!("[App Callback] AuthHandler: Client '{}' auth result: ok={}, group='{}'.", client_id, is_ok, group);
    (is_ok, group)
}

fn app_level_client_request_handler(
    client_id: &str,
    msg_id: &str,
    method: &str,
    version: &str,
    endpoint: &str,
    params: Value,
) {
    println!(
        "[App Callback] ClientRequestHandler: client_id='{}', msg_id='{}', method='{}', version='{}', endpoint='{}', params={:?}",
        client_id, msg_id, method, version, endpoint, params
    );
    // TODO: Implement application-specific logic to handle requests from clients.
    // This is called by the crate when it processes a client request message
    // and decides to forward it to the application via this handler.
}

fn app_level_response_callback_handler(client_id: &str, msg_id: &str, success: bool, result: &str) {
    println!(
        "[App Callback] ResponseCallbackHandler: client_id='{}', msg_id='{}', success={}, result='{}'",
        client_id, msg_id, success, result
    );
    // TODO: Handle responses to requests that the application might have initiated towards clients.
}

fn app_level_client_drop_handler(client_id: &str) {
    println!("[App Callback] ClientDropHandler: Client '{}' resources are being dropped.", client_id);
    // TODO: Application logic for when a client's resources are fully released by the crate.
}


#[tokio::main]
async fn main() {
    setup(); // Call crate's general setup
    println!("[App] Crate setup initiated.");

    // --- Register App-Level Handlers with the Crate ---
    handler::register_disconnect_handler(app_level_disconnect_handler);
    println!("[App] Registered disconnect handler.");

    handler::register_auth_handler(app_level_auth_handler);
    println!("[App] Registered auth handler.");
    
    handler::register_client_request_handler(app_level_client_request_handler);
    println!("[App] Registered client request handler.");

    handler::register_response_callback_handler(app_level_response_callback_handler);
    println!("[App] Registered response callback handler.");

    handler::register_client_drop_handler(app_level_client_drop_handler);
    println!("[App] Registered client drop handler.");


    // --- Axum Server Setup ---
    let app_router = Router::new()
        .route("/", get(|| async { "[App] wsm server running" }))
        .route("/socket", get(websocket_handler_axum_adapter));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3333));
    println!("[App] WebSocket server listening on ws://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app_router).await.unwrap();
}

async fn websocket_handler_axum_adapter(ws: WebSocketUpgrade) -> impl IntoResponse {
    println!("[App] WebSocket upgrade request received.");
    ws.on_upgrade(handle_individual_websocket_connection)
}

async fn handle_individual_websocket_connection(mut socket: WebSocket) {
    println!("[App] New WebSocket connection established with a client.");

    // 1. Initialize client connection using the crate's function and get initial data + client_id
    let (json_to_send, client_id) = init_client_connection(); // This client_id is from the crate
    println!("[App] Connection assigned client_id '{}' by the crate.", client_id);

    // 2. Send the initial JSON (from crate) to the WebSocket client
    match serde_json::to_string(&json_to_send) {
        Ok(json_string) => {
            if socket.send(Message::Text(json_string.into())).await.is_err() {
                eprintln!("[App] Error sending initial message to client_id '{}'. Client might have disconnected prematurely.", client_id);
                // The crate's internal logic should handle this scenario and trigger relevant handlers if necessary.
                // For example, if init_client_connection succeeded but send failed, the crate might trigger disconnect.
                return;
            }
            println!("[App] Successfully sent initial data to client_id '{}'.", client_id);
        }
        Err(e) => {
            eprintln!("[App] Error serializing initial JSON for client_id '{}': {}. Disconnecting client.", client_id, e);
            let _ = socket.close().await;
            // Similarly, the crate should manage the aftermath of this failure.
            return;
        }
    }

    // 3. Main loop for receiving messages from this WebSocket client
    loop {
        match socket.next().await {
            Some(Ok(msg)) => {
                match msg {
                    Message::Text(text) => {
                        println!("[App] Received text message from client_id '{}': {}", client_id, text);
                        match serde_json::from_str::<Value>(&text) {
                            Ok(json_value) => {
                                // Pass the raw JSON value and client_id to the crate's message handler
                                handle_wsm_message(json_value, &client_id);
                                println!("[App] Passed message from client_id '{}' to crate's handle_wsm_message.", client_id);
                            }
                            Err(e) => {
                                eprintln!("[App] Error parsing JSON from client_id '{}': {}. Message: '{}'", client_id, e, text);
                                if socket.send(Message::Text(format!("Error: Invalid JSON format - {}", e).into())).await.is_err() {
                                    eprintln!("[App] Error sending error message to client_id '{}'. Client might have disconnected.", client_id);
                                    break; // Break loop on send error
                                }
                            }
                        }
                    }
                    Message::Binary(_) => {
                        println!("[App] Received binary message from client_id '{}'. Not handled by this app-level code.", client_id);
                    }
                    Message::Ping(ping_data) => {
                        if socket.send(Message::Pong(ping_data)).await.is_err() {
                            eprintln!("[App] Error sending Pong to client_id '{}'. Client might have disconnected.", client_id);
                            break; // Break loop on send error
                        }
                    }
                    Message::Pong(_) => {
                        println!("[App] Received Pong from client_id '{}'.", client_id);
                    }
                    Message::Close(close_frame) => {
                        if let Some(cf) = close_frame {
                            println!("[App] Received Close frame from client_id '{}': code={}, reason='{}'", client_id, cf.code, cf.reason);
                        } else {
                            println!("[App] Received Close frame (no details) from client_id '{}'.", client_id);
                        }
                        break; // Client initiated close, break loop
                    }
                }
            }
            Some(Err(e)) => {
                eprintln!("[App] Error receiving message from WebSocket for client_id '{}': {}. Connection will be closed.", client_id, e);
                break; // Break loop on receive error
            }
            None => {
                println!("[App] WebSocket stream for client_id '{}' ended (client likely disconnected without a formal close frame).", client_id);
                break; // Stream ended, break loop
            }
        }
    }
    
    println!("[App] WebSocket connection handling loop for client_id '{}' ended.", client_id);
    // Attempt to gracefully close the WebSocket from server side if not already closed by client.
    if let Err(e) = socket.close().await {
        eprintln!("[App] Error during server-side close for client_id '{}': {}", client_id, e);
    }
    println!("[App] WebSocket connection with client_id '{}' formally closed at app level.", client_id);
    // At this point, if the crate's internal logic (e.g., in handle_wsm_message or connection tracking)
    // detects this disconnection, it would be responsible for invoking the
    // `app_level_disconnect_handler` and/or `app_level_client_drop_handler` that were registered by `main`.
}