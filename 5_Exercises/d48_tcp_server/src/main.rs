// Day 48: Project: Simple TCP Server
// Build a basic TCP server in Rust that listens for incoming client connections and 
// echoes back messages. This project introduces socket programming, threading per connection, 
// and line-based I/O—a fundamental stepping stone to chat apps, APIs, and web servers.

// Key Concepts:
// + TcpListener::bind() to create a server socket.
// + TcpStream for client connections.
// + BufReader to read lines from socket.
// + thread::spawn for handling multiple clients.
// This project lays the foundation for networked applications, from chat systems and HTTP servers to remote shells and agents.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> std::io::Result<()> {
    println!("TCP Echo Server listening on 127.0.0.1:7878");
 
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection from: {}", stream.peer_addr()?);
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let peer = stream.peer_addr().unwrap();
    let reader = BufReader::new(stream.try_clone().unwrap());
 
    for line in reader.lines() {
        match line {
            Ok(msg) => {
                println!("[{}] {}", peer, msg);
                let response = format!("Echo: {}\n", msg);
                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("Error with {}: {}", peer, e);
                break;
            }
        }
    }
 
    println!(" Connection with {} closed.", peer);
}
