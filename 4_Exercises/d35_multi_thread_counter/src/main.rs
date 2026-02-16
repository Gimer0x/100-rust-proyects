// Day 35: Project: Multi-threaded Counter
// Build a multi-threaded counter that spawns multiple threads, each incrementing 
// a shared value. This project teaches you concurrency basics in Rust using std::thread, 
// atomic types, and safe shared memory with Arc.

// Key Concepts:
// Arc (Atomic Reference Counted) = share memory between threads.
// AtomicUsize = safe mutable counter for concurrent access.
// Ordering::SeqCst = strict memory ordering for correctness.
// This project introduces thread spawning, atomic operations, and synchronization—essential 
// for building scalable concurrent systems, job schedulers, or parallel processing tools.
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;

fn main() {
    println!("Multi-threaded counter!");

    // Shared atomic counter starting at 0.
    let counter = Arc::new(AtomicUsize::new(0));
    // Store JoinHandle values for each spawned thread.
    let mut handles = vec![]; 

    // Spawn 5 threads, each identified by i.
    for i in 0..5 {
        // Clone Arc to share the counter with the thread.
        let counter_clone = Arc::clone(&counter);

        // Start a new thread and move data into it.
        let handle = thread::spawn(move || {
            // Each thread increments the counter 1000 times.
            for _ in 0..1000 { 
                // Atomically add 1 (sequentially consistent).
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }

            // Log when this thread completes.
            println!("Thread {} finished.", i); 
        });

        // Keep the handle so we can join later.
        handles.push(handle);
    }

    // Wait for all threads to finish.
    for handle in handles { 
        // Join and propagate any panic as an error.
        handle.join().expect("Thread paniched!"); 
    }

    // Read and print final counter value.
    println!("Final count: {}", counter.load(Ordering::SeqCst)); 
}
