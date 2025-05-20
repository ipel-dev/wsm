use tokio_tungstenite::connect_async;
use url::Url;
use futures_util::stream::StreamExt;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let url = Url::parse("ws://localhost:3333/socket").unwrap();
    println!("Connecting to {}", url);

    match connect_async(url).await {
        Ok((ws_stream, _)) => {
            println!("Connected successfully!");
            let (_write, read) = ws_stream.split();
            read.for_each(|message| async {
                if let Ok(msg) = message {
                    if msg.is_text() {
                        if let Ok(json) = serde_json::from_str::<Value>(&msg.to_string()) {
                            println!("{}", serde_json::to_string_pretty(&json).unwrap());
                        } else {
                            println!("Non-JSON text: {}", msg.to_text().unwrap());
                        }
                    }
                }
            }).await;
        }
        Err(e) => {
            eprintln!("Connection failed: {}", e);
        }
    }
}
