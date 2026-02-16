// Day 87: Project: YouTube Audio Downloader (via yt-dlp + Rust CLI)
// Create a Rust CLI tool that lets users download audio from a YouTube video using the 
// popular yt-dlp tool under the hood. You’ll use std::process::Command to execute shell 
// commands and handle arguments and output from Rust.
// Key Concepts:
// + std::process::Command to call shell tools from Rust
// + yt-dlp flags: --extract-audio, --audio-format, --output
// + Minimal Rust wrapper over powerful system commands
// You've built a Rust-powered interface for YouTube audio downloads—easily extensible 
// for playlists, format options, or GUI.
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let url = prompt("Enter YouTube URL: ");

    if url.is_empty() {
        println!("No url provided!");
        return
    }

    println!("Downloading audio...");
    let status = Command::new("yt-dlp")
        .args([
            "--extract-audio",
            "--audio-format", "mp3",
            "--output", "%(title)s.%(ext)s",
            &url,
        ])
        .status();

    match status {
        Ok(s) if s.success() => println!("Audio downloaded successfully!"),
        Err(e) => println!("Failed to download! {}", e),
        Ok(_) => println!("No value!")
    }

}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}
