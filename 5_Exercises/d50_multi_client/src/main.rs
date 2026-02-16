// Day 50: Project: Multi-Client Chat Server
// Extend your TCP server to support multiple clients chatting together. 
// This project introduces shared state across threads, broadcasting messages, 
// and synchronizing sockets using Arc<Mutex<>>.
// Key Concepts:
// + HashMap<String, TcpStream> – Track all active clients.
// + Arc<Mutex<>> – Share and synchronize access safely.
// + writeln!() – Send broadcast to each stream.
// You now have a working real-time multi-user chat server—a foundation for building 
// group messaging apps, collaboration tools, or game lobbies.
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

type Clients = Arc<Mutex<HashMap<String, TcpStream>>>;

fn main() -> std::io::Result<()>{
    println!("Multi-Client Chat Server listening on 127.0.0.1:7878");
 
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        let stream = stream?;
        let addr = stream.peer_addr()?.to_string();
        println!("New connection: {}", addr);
 
        let clients = Arc::clone(&clients);
        clients.lock().unwrap().insert(addr.clone(), stream.try_clone()?);
 
        thread::spawn(move || handle_client(stream, addr, clients));
    }
 
    Ok(())
}

fn handle_client(stream: TcpStream, addr: String, clients: Clients) { 
    let reader = BufReader::new(stream.try_clone().unwrap());

    for line in reader.lines() {
        let msg = match line {
            Ok(msg) => msg,
            Err(_) => break,
        };
 
        let full_msg = format!("[{}]: {}", addr, msg);
        println!("{}", full_msg);
 
        let mut clients_lock = clients.lock().unwrap();
        let mut disconnected = vec![];
 
        for (peer, client_stream) in clients_lock.iter_mut() {
            if peer != &addr {
                if let Err(_) = writeln!(client_stream, "{}", full_msg) {
                    disconnected.push(peer.clone());
                }
            }
        }
 
        // Remove disconnected clients
        for peer in disconnected {
            clients_lock.remove(&peer);
        }
    }

    println!("{} disconnected.", addr);
    clients.lock().unwrap().remove(&addr);
}