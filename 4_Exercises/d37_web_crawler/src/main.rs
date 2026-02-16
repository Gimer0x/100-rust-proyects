// Day 37: Project: Simple Parallel Web Crawler
// Build a simple multi-threaded web crawler that takes a list of URLs 
// and fetches them in parallel using a thread pool. This project combines 
// HTTP requests, concurrency, and error handling, simulating a lightweight 
// scraper or link checker.

// Concepts Reinforced:
// Concurrency: Use threads to handle multiple tasks in parallel.
// HTTP: Basic GET requests with reqwest.
// Synchronization: Use mpsc + Arc<Mutex> to coordinate jobs.
// This project prepares you to build parallel scrapers, uptime monitors, 
// and async-aware tools in Rust.
use reqwest::blocking::get;
use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};

fn main() {
    println!("Parallel Web Crawler");

    let urls = vec![
        "https://www.rust-lang.org",
        "https://www.google.com",
        "https://docs.rs",
        "https://httpbin.org/delay/1",
        "https://thisurldoesnotexist.com", // intentionally fails
    ];

    let pool_size = 2;

// These two lines set up a thread-safe job queue for the workers.
// 
// - `mpsc::channel::<String>()` creates a channel for sending URLs from the main thread to worker threads. 
//    The `<String>` makes the type explicit so the receiver isn’t inferred as `str` (which is unsized).
// - `Arc::new(Mutex::new(rx))` wraps the receiver so multiple worker threads can safely share it:
//   + `Arc` = shared ownership across threads
//   + `Mutex` = only one worker reads from the receiver at a time
// 
// Without this, each thread couldn’t safely access the same receiver.

    let (tx, rx) = mpsc::channel::<String>();
    let rx = Arc::new(Mutex::new(rx));

    for i in 0..pool_size {
        let rx = Arc::clone(&rx);

        // Generador de hilo
        // thread::spawn(...) starts a new OS thread and runs the provided closure inside it.
        // move || { ... } is a closure that takes ownership of any variables it uses from the outer scope.
        // This is required because the closure runs on another thread, so it can’t borrow local variables safely.
        // loop { ... } The thread will keep running and repeatedly execute the loop body until you break.
        // It means: “Start a new thread. Move the needed data into it. Then keep looping forever, usually waiting 
        // for jobs.” That’s exactly how each worker thread stays alive and keeps pulling work from the channel.
        thread::spawn(move || loop {
            let url = match rx.lock().unwrap().recv() {
                Ok(u) => u,
                Err(_) => break,
            };

            println!("Worker {} fetching: {}", i, url);
            match fetch_url(&url) {
                Ok(status) => println!("✅ {} => {}", url, status),
                Err(e) => println!("❌ {} => {}", url, e),
            }
        });
    }

    for url in urls {
        // tx.send(...) sends the String through the channel to the workers.
        // tx.send(url.to_string()).unwrap(); --> This panicks when fails!
        if let Err(e) = tx.send(url.to_string()) {
            eprintln!("Failed to send URL: {}", e);
        }
    }
    thread::sleep(Duration::from_secs(10));
}

fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let response = get(url)?;
    Ok(format!("Status {}", response.status()))
}

// We use ? because get(url) returns a Result<>. The `?` operator says:
// 
// - If it’s `Ok(value)`, unwrap it and continue.
// - If it’s `Err(e)`, return early from this function with that error.
// 
// So:
// 
// let response = get(url)?;
// 
// is shorthand for:
//
// let response = match get(url) {
//     Ok(v) => v,
//     Err(e) => return Err(e),
// };
// 
// This only works because your function returns a `Result<..., reqwest::Error>`.