// Day 77: Project: Async Web Fetcher (Tokio + Reqwest)
// Build a CLI tool that fetches and prints the body of a web page asynchronously 
// using tokio and reqwest. You’ll learn to write async fn main, handle errors, 
// and perform non-blocking HTTP requests.
//Key Concepts:
//#[tokio::main] = turns main into an async runtime entrypoint
//+ reqwest::get() returns a Future
//+ await is required to get actual results
//+ .text() consumes the body as a String
//Extensions You Can Try:
//+ Accept URL as a command-line argument
//+ Save response to a file
//+ Parse the HTML with scraper
//You've built a modern, asynchronous HTTP client in Rust using best practices.


use reqwest::Error;
 
#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://www.rust-lang.org";
    println!("🌐 Fetching: {}", url);
 
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
 
    println!("✅ Response received ({} bytes)", body.len());
    println!("\nPreview:\n{}", &body[..200.min(body.len())]); // Show first 200 chars
 
    Ok(())
}