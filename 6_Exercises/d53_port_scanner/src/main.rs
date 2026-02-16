// Day 53: Project: Port Scanner
// Build a simple port scanner CLI tool in Rust that checks which TCP ports are open 
// on a target host. You’ll use low-level socket programming with TcpStream::connect_timeout 
// to test ports and implement basic network scanning logic.

// Key Concepts:
// + TcpStream::connect_timeout() — try connecting to a port with timeout.
// + ToSocketAddrs — resolve hostname + port to address.
// + Iterate over a range of ports to check availability.
// With this, you’ve built a foundation for network monitoring tools, firewalls, 
// or penetration testing utilities.

use std::io::{self, Write};
use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::time::Duration;

fn main() {
    println!("Rust Port Scanner");
 
    let host = input("Enter host (e.g., 127.0.0.1 or google.com): ");
    let start = input("Enter start port: ").parse::<u16>().unwrap_or(1);
    let end = input("Enter end port: ").parse::<u16>().unwrap_or(1024);
 
    println!("Scanning {} from port {} to {}", host, start, end);

    for port in start..=end{
        let address = format!("{}:{}", host, port);
        let timeout = Duration::from_millis(300);

        if let Ok(addrs) = address.to_socket_addrs(){
            for addr in addrs {
                if is_port_open(addr, timeout) {
                    println!("Port {} is open", port);
                }
            }
        }

    }
}

fn is_port_open(addr: SocketAddr, timeout: Duration) -> bool {
    TcpStream::connect_timeout(&addr, timeout).is_ok()
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
