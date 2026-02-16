// Day 49: Project: TCP Client
// Now that you’ve built a TCP server, let’s create a TCP client that connects to it 
// and sends messages from the command line. This project teaches you how to use TCP 
// streams, read/write from stdin/stdout, and communicate with networked services.
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

fn main() -> std::io::Result<()> {
    println!("TCP Client Connecting to 127.0.0.1:7878...");

    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    let reader = BufReader::new(stream.try_clone()?);

    // Thread to listen for messages from server
    thread::spawn(move || {
        for line in reader.lines() {
            match line {
                Ok(msg) => println!("Server: {}", msg),
                Err(_) => {
                    println!("Server disconnected!");
                    break;
                }
            }
        }
    });

    // Main thread: send input to server
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let msg = line?;
        if msg == "exit" {
            println!("Disconnecting.");
            break;
        }
        stream.write_all(msg.as_bytes())?;
        stream.write_all(b"\n")?;
    }

    Ok(())
}
