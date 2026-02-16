// Day 40: Project: Concurrency-Based Data Processing
// Create a Rust program that uses multiple threads to process chunks of
// data (like numbers or strings) concurrently and then collects the results.
// This project ties together Arc, Mutex, thread, and data partitioning to simulate
// a parallel map operation.

// Concepts Applied:
// Data Partitioning: Break the work into chunks.
// Parallel Processing: Spawn threads for each chunk.
// Shared State: Use Arc<Mutex<T>> to collect results.
// Result Aggregation: Join and merge all outputs.
// This project is foundational for parallel data pipelines, batch job
// processors, or building your own concurrent map/reduce system.
use std::sync::{Arc, Mutex}; // Arc for shared ownership, Mutex for safe mutation.
use std::thread; // Thread API for spawning worker threads.

fn main() { // Program entry point.
    println!("Concurrent Data Processor"); 

    // Original data
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    // Wrap data in Arc so threads can share it safely.
    let data = Arc::new(data); 

    // Shared results container.
    let results: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];

    // Split data into chunks of 2 items each.
    for chunk in data.chunks(2) { 
        // Clone the chunk so the thread owns its data.
        let chunk = chunk.to_vec(); 
        // println!("Chunk len: {}", chunk.len());
        // Clone Arc to share results with this thread.
        let results = Arc::clone(&results); 

        // Spawn a worker thread for this chunk.
        let handle = thread::spawn(move || {
            let processed: Vec<i32> = chunk.iter().map(|n| n * n).collect();
            // Lock the shared results vector.
            let mut res = results.lock().unwrap(); 
            // Append processed numbers to the shared results.
            res.extend(processed);
        });
        // Save the handle so we can join later.
        handles.push(handle); 
    }

    // Wait for all threads to finish.
    for handle in handles { 
        // Join and propagate any panic.
        handle.join().unwrap(); 
    }
    // Lock to read final results.
    let final_results = results.lock().unwrap(); 
    // Print the collected results.
    println!("Final processed results: {:?}", *final_results); 
}
