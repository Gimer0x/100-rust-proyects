// Day 54: Project: WebSocket Echo Server
// Build a basic WebSocket echo server that handles real-time communication with 
// clients. This project introduces bidirectional async messaging, using the tokio 
// and tokio-tungstenite crates to support modern WebSocket connections.
// Key Concepts:
// + tokio-tungstenite: WebSocket over async TCP.
// + StreamExt and SinkExt: For reading/writing messages.
// + tokio::spawn: Handle multiple clients concurrently.
// This project sets you up to build real-time systems like chat apps, game lobbies, live dashboards, 
// and collaborative tools.
// Run with: websocat ws://127.0.0.1:9001
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};

#[tokio::main]
async fn main() {
    println!("WebSocket Echo Server running on ws://127.0.0.1:9001");
 
    let listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();

    while let Ok((stream, addr)) = listener.accept().await {
        println!("New connection from: {}", addr);
 
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await;
            match ws_stream {
                Ok(mut websocket) => {
                    while let Some(msg) = websocket.next().await {
                        match msg {
                            Ok(msg) if msg.is_text() || msg.is_binary() => {
                                println!("Echoing message: {:?}", msg);
                                if let Err(e) = websocket.send(msg).await {
                                    eprintln!("Send error: {}", e);
                                    break;
                                }
                            }
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("WebSocket error: {}", e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Handshake error: {}", e),
            }
        });
    }
}
