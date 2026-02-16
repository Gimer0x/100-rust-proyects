// Day 99: Dockerfile Generator (Rust-Aware)
// Create a CLI tool that detects Rust project layout (Cargo.toml, src/main.rs) 
// and generates a multi-stage // Dockerfile to build and ship the app. It’s perfect 
// for CI/CD and containerized deployment.
// Key Concepts:
// + Parse Cargo.toml to extract project name
// + Write multi-stage Dockerfile optimized for minimal runtime
// + Use standard fs and Path APIs for file operations
// You’ve now built a Rust-native Dockerfile generator, ideal for automating deployment 
// and shipping microservices!

use std::fs;
use std::path::Path;
 
fn main() {
    let cargo_path = Path::new("Cargo.toml");
 
    if !cargo_path.exists() {
        eprintln!("❌ Not a Rust project (Cargo.toml not found).");
        return;
    }
 
    let app_name = infer_crate_name().unwrap_or("rust_app".to_string());
 
    let dockerfile = format!(
        r#"FROM rust:1.74 AS builder
        WORKDIR /usr/src/{0}
        COPY . .
        RUN cargo build --release
        
        FROM debian:bookworm-slim
        WORKDIR /app
        COPY --from=builder /usr/src/{0}/target/release/{0} /app/{0}
        CMD ["./{0}"]
        "#, 
        app_name);
 
    fs::write("Dockerfile", dockerfile).expect("Failed to write Dockerfile");
    println!("Dockerfile generated for `{}`!", app_name);
}

fn infer_crate_name() -> Option<String> {
    let contents = fs::read_to_string("Cargo.toml").ok()?;
    for line in contents.lines() {
        if line.trim_start().starts_with("name") {
            return line.split('=').nth(1).map(|s| s.trim_matches(&[' ', '"'][..]).to_string());
        }
    }
    None
}