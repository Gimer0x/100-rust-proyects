// Day 52: Project: REST API Consumer
// Build a CLI tool that makes HTTP GET requests to a public REST API, parses 
// the JSON response, and prints useful data. You’ll use reqwest and serde_json 
// to learn about network requests, parsing JSON, and working with external APIs.
// Key Concepts:
// + reqwest::blocking::get() – Makes synchronous HTTP calls.
// + serde – Parses JSON into strongly typed Rust structs.
// + .json::<T>() – Automatic deserialization.
// This project gets you comfortable with consuming REST APIs, an essential skill 
// for building CLI clients, dashboards, bots, and data pipelines.
use reqwest::blocking::get;
use serde::Deserialize;
use std::io::{self, Write};

#[derive(Debug, Deserialize)]
struct Joke {
    id: u32,
    r#type: String,
    setup: String,
    punchline: String,
}

fn main()-> Result<(), Box<dyn std::error::Error>>{
    println!("Random Joke Fetcher from REST API");
 
    let _ = prompt("Press Enter to fetch a joke...");

    let url = "https://official-joke-api.appspot.com/random_joke";
    // First ? handles HTTP request errors
    // Second ? handles JSON parsing errors
    let response = get(url)?.json::<Joke>()?;

    println!("\nJoke Id: {}", response.id);
    println!("Joke Type: {}", response.r#type);
    println!("Setup: {}", response.setup);
    println!("Punchline: {} \n", response.punchline);

    Ok(())
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}
