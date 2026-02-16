// Day 91: Project: ChatGPT API CLI Tool (OpenAI-powered)
// Build a terminal-based tool that sends user input to OpenAI's ChatGPT API and prints 
// the assistant's response. You'll learn to work with HTTP APIs, JSON structures, and 
// environment-based secrets.
// Key Concepts:
// + dotenvy loads secrets from .env
// + reqwest makes HTTP calls with JSON
// + serde for clean API request/response mapping
// + tokio enables async execution
// You now have a lightweight terminal AI assistant that leverages OpenAI’s API — perfect 
// for scripts, bots, or interactive CLI tools.
use dotenvy::dotenv;
use std::env;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}
 
#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}
 
#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

/// OpenAI API error payload (returned when status is not 2xx).
#[derive(Deserialize)]
struct ApiErrorResponse {
    error: ApiError,
}

#[derive(Deserialize)]
struct ApiError {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY")?;
 
    let user_input = prompt("You: ");
 
    let req_body = ChatRequest {
        model: "gpt-3.5-turbo",
        messages: vec![Message {
            role: "user",
            content: user_input.as_str(),
        }],
    };
 
    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&req_body)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        let msg = serde_json::from_str::<ApiErrorResponse>(&body)
            .map(|e| e.error.message)
            .unwrap_or(body);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("OpenAI API error ({}): {}", status, msg),
        )
        .into());
    }

    let res: ChatResponse = serde_json::from_str(&body)?;
 
    let response_text = &res.choices[0].message.content;
    println!("🤖 Assistant: {}", response_text.trim());
 
    Ok(())
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}