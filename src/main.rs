// src/main.rs

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt}; // Added SinkExt
use serde_json::Value;
use std::net::SocketAddr;

pub mod id;
pub mod pool;
pub mod message;
pub mod wsa;
pub mod method;
pub mod params;
pub mod pretty;
pub mod client;
pub mod socket;
pub mod server;
pub mod handler;
pub mod handshake;
pub mod callback;
pub mod process;
pub mod setup;
pub mod wsm;

pub use wsa::{
    create_request_form_server_to_client,
    create_request_form_client_to_server,
    create_request_form_client_to_client,
    create_success_response_form_server_to_client,
    create_fail_response_form_server_to_client,
    create_success_response_form_client_to_server,
    create_fail_response_form_client_to_server,
    create_success_response_form_client_to_client,
    create_fail_response_form_client_to_client,
    create_event_form_server_to_client
};

use crate::wsm::init_client_connection;
use crate::setup::setup;
use crate::socket::handle_wsm_message;

#[tokio::main]
async fn main() {
    setup();

    let app = Router::new()
        .route("/", get(|| async { "wsm server running" }))
        .route("/socket", get(websocket_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3333));
    println!("WebSocket server listening on ws://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    println!("WebSocket upgrade request received.");
    ws.on_upgrade(handle_individual_socket)
}

async fn handle_individual_socket(mut socket: WebSocket) {
    println!("New WebSocket connection established.");

    // 1. Initialize client connection and get initial data
    let (json_to_send, client_id) = init_client_connection();
    println!("[Client {}] Generated client_id for new connection.", client_id);

    // 2. Send the initial JSON to the client
    match serde_json::to_string(&json_to_send) {
        Ok(json_string) => {
            // Applied compiler suggestion: .into() for the String
            if socket.send(Message::Text(json_string.into())).await.is_err() { 
                eprintln!("[Client {}] Error: Failed to send initial message. Client might have disconnected.", client_id);
                return; 
            }
            println!("[Client {}] Successfully sent initial data: {:?}", client_id, json_to_send);
        }
        Err(e) => {
            eprintln!("[Client {}] Error: Failed to serialize initial JSON: {}. Disconnecting.", client_id, e);
            // Added SinkExt, so close() should be available
            let _ = socket.close().await;
            return;
        }
    }

    // 3. Main loop for receiving messages from this client
    while let Some(result) = socket.next().await { 
        match result {
            Ok(msg) => {
                match msg {
                    Message::Text(text) => {
                        println!("[Client {}] Received text message: {}", client_id, text);
                        match serde_json::from_str::<Value>(&text) {
                            Ok(json_value) => {
                                handle_wsm_message(json_value, &client_id);
                                println!("[Client {}] Processed message with handle_wsm_message.", client_id);
                            }
                            Err(e) => {
                                eprintln!("[Client {}] Error: Failed to parse JSON from client: {}. Message: '{}'", client_id, e, text);
                                // Applied compiler suggestion: .into() for the String
                                if socket.send(Message::Text(format!("Error: Invalid JSON format - {}", e).into())).await.is_err() { 
                                    eprintln!("[Client {}] Error: Failed to send error message. Client might have disconnected.", client_id);
                                    break;
                                }
                            }
                        }
                    }
                    Message::Binary(_) => {
                        println!("[Client {}] Received binary message. Not currently handled.", client_id);
                    }
                    Message::Ping(ping_data) => {
                        if socket.send(Message::Pong(ping_data)).await.is_err() { 
                             eprintln!("[Client {}] Error: Failed to send Pong. Client might have disconnected.", client_id);
                             break;
                        }
                    }
                    Message::Pong(_) => {
                        println!("[Client {}] Received Pong.", client_id);
                    }
                    Message::Close(close_frame) => {
                        if let Some(cf) = close_frame {
                            println!("[Client {}] Received Close frame: code={}, reason='{}'", client_id, cf.code, cf.reason);
                        } else {
                            println!("[Client {}] Received Close frame (no details).", client_id);
                        }
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("[Client {}] Error receiving message: {}. Disconnecting.", client_id, e);
                break;
            }
        }
    }

    println!("[Client {}] WebSocket connection closed.", client_id);
    // Added SinkExt, so close() should be available
    let _ = socket.close().await;
}