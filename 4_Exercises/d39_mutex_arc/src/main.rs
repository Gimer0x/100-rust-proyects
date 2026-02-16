// Day 39: Project: Mutex and Arc Demo
// Build a shared counter using multiple threads, Arc (atomic reference counting), and Mutex (mutual exclusion). 
// This project teaches you how to safely mutate shared data across threads using Rust's concurrency primitives.

// Key Concepts:
// Arc<T>: Enables multiple threads to share ownership of data.
// Mutex<T>: Ensures only one thread mutates data at a time.
// lock().unwrap(): Acquires the lock, safely accessing the shared value.
// This project teaches safe shared mutability, the foundation of parallel computation, 
// worker pools, and shared caches in Rust.

// Arc lets multiple threads share ownership of the same data.
// Mutex provides safe, exclusive access to that data.
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("Mutex + Arc Shared Counter");
 
    // Creates a shared counter
    // wrapped in Mutex to allow safe mutation
    // wrapped in Arc to share across threads
    let counter = Arc::new(Mutex::new(0));
    // Stores thread handles so we can join later.
    let mut handles = vec![];

    // Spawns 5 threads
    for i in 0..5 {
        // Arc::clone increments the reference count (not a deep copy)
        // move transfers the clone into the thread
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Each thread increments the counter 1000 times:
            // lock() acquires the mutex (blocking if another thread holds it)
            for _ in 0..1000 {
                // num is a mutable reference to the counter
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
            println!("✅ Thread {} finished.", i);
        });
        // Saves the thread handle for later join.
        handles.push(handle);
    }
    
    // Threads don’t wait for a thread to end, they only wait to acquire the lock for each increment.
    //t1 locks -> num = 1 -> unlocks
    //t2 locks -> num = 2 -> unlocks
    //t3 locks -> num = 3 -> unlocks

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}
