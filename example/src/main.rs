use tokio_tungstenite::connect_async;
use futures_util::{stream::StreamExt, SinkExt};
use serde_json::{Value, json};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    let url = "ws://localhost:3333/socket";

    match connect_async(url).await {
        Ok((mut ws_stream, _)) => {
            println!("Connected successfully!");

            if let Some(Ok(msg)) = ws_stream.next().await {
                if msg.is_text() {
                    let text = msg.to_text().unwrap_or_default();
                    if let Ok(json_val) = serde_json::from_str::<Value>(text) {
                        println!("{}", serde_json::to_string_pretty(&json_val).unwrap());

                        let client_id_opt = json_val.get("t").and_then(|v| v.as_str());
                        let msg_id_opt = json_val.get("i").and_then(|v| v.as_str());

                        if let (Some(client_id), Some(msg_id)) = (client_id_opt, msg_id_opt) {
                            println!("client_id: {}", client_id);
                            println!("msg_id: {}", msg_id);

                            let reply = json!({
                                "f": client_id,
                                "t": "s",
                                "y": "r",
                                "i": msg_id,
                                "p": {
                                    "r": "s",
                                    "c": "1edqdJIgd73hjbdja="
                                }
                            });

                            let json_str = serde_json::to_string(&reply).unwrap();
                            println!("Sending:\n{}", serde_json::to_string_pretty(&reply).unwrap());

                            if let Err(e) = ws_stream.send(Message::Text(json_str.into())).await {
                                eprintln!("Failed to send message: {}", e);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Connection failed: {}", e);
        }
    }
}
