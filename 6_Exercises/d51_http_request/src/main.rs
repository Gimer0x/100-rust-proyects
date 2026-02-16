// Day 51: Project: HTTP Request Parser
// Build a simple TCP server that parses raw HTTP requests and responds with a basic 
// HTML page. This project teaches you how to understand the structure of HTTP, handle 
// request headers, and simulate a bare-bones web server.
// Key Concepts:
// + TcpListener handles incoming socket connections.
// + Manually parse HTTP headers from the raw TCP stream.
// + Craft an HTTP-compliant response.
// With this, you now understand the low-level anatomy of HTTP, setting you up to build 
// custom web servers, REST APIs, or even proxies.
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> std::io::Result<()> {
    println!("HTTP Server running at http://127.0.0.1:8080");
 
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(|| {
            handle_client(stream).unwrap_or_else(|e| eprintln!("Error {}", e));
        });
    }

    Ok(())
}

// std::io::Result<()> = Result<(), std::io::Error>
fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
 
    let request = String::from_utf8_lossy(&buffer);
    let lines: Vec<&str> = request.lines().collect();
 
    if let Some(request_line) = lines.first() {
        println!("Request: {}", request_line);
    }
 
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
        <html><body><h1>Hello from Rust HTTP Server!</h1></body></html>";
 
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
