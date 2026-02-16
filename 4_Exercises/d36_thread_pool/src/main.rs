// Day 36: Project: Build a Thread Pool
// Implement a basic thread pool to handle queued tasks concurrently using a fixed number
// of worker threads. This project introduces you to message passing with channels, thread
// synchronization, and designing your own lightweight executor.

// Key Concepts:
// - ThreadPool: Limits thread creation, reuses workers.
// - Channels: Used for job queueing.
// - Arc<Mutex<Receiver>>: Shared, thread-safe job access.
// This project is foundational for building concurrent servers, async runtimes, or parallel
// job execution systems.

// mpsc for channels, Arc/Mutex for shared, thread-safe access.
use std::sync::{mpsc, Arc, Mutex};
// Thread API for spawning worker threads.
use std::thread; 

// ThreadPool owns workers and the job sender.
struct ThreadPool { 
    // Worker threads that will execute jobs.
    workers: Vec<Worker>, 
    // Channel sender for submitting jobs to workers.
    sender: mpsc::Sender<Job>, 
}

// A boxed task that runs once on a thread.
// 'type' defines a type alias named Job.
// so you can write 'Job' instead of the full type, like a shorthand.
type Job = Box<dyn FnOnce() + Send + 'static>; 

// A Box puts a value on the heap and gives you a pointer to it.
// Trait objects like dyn FnOnce() don’t have a known size at compile time.
// Box gives it a fixed-size pointer so it can be stored and passed around.

// dyn FnOnce()
// This is a trait object for any closure (or function) that:
// takes no arguments (()) and can be called once

// Send is a marker trait meaning the value is safe to move across threads.
// Since jobs are sent to worker threads, the closure must be Send.

// The 'static lifetime means:
// The closure does not borrow any data that will go out of scope.
// It either owns its data or borrows static data.
// This is required because the job may run later on another thread.

// A 'Job' means: "A boxed closure that is safe to send to another thread 
// and can be executed once".

impl ThreadPool { 
    // Create a pool with a fixed number of workers.
    fn new(size: usize) -> Self { 
        // Prevent zero-sized pools.
        assert!(size > 0, "Pool size must be greater than zero!"); 
        // Create a channel for job messages.
        let (sender, receiver) = mpsc::channel();
        // Share receiver safely across workers.
        let receiver = Arc::new(Mutex::new(receiver)); 

        // Create worker IDs from 0 to size-1.
        let workers = (0..size) 
            // Build each worker with shared receiver.
            .map(|id| Worker::new(id, Arc::clone(&receiver))) 
            .collect();

        ThreadPool { workers, sender } 
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, // Jobs must be sendable across threads and 'static.
    {
        // Send the boxed job to the queue.
        self.sender.send(Box::new(f)).unwrap();
    }
}

// Worker represents a single thread in the pool.
struct Worker {
    // Worker identifier for logging. 
    id: usize, 
    // Thread handle, stored for optional join.
    thread: Option<thread::JoinHandle<()>>, 
}

// Implementation of Worker methods.
impl Worker { 
    // Create and start a worker.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self { 
        // Spawn thread that repeatedly waits for jobs.
        let thread = thread::spawn(move || loop { 
            // Lock receiver and wait for next job.
            let message = receiver.lock().unwrap().recv(); 
            // Handle job or shutdown signal.
            match message { 
                // Received a job.
                Ok(job) => { 
                    // Log which worker received work.
                    println!("👷 Worker {} got a job!", id);
                    // Execute the job.
                    job(); 
                }
                // Channel closed, time to shut down.
                Err(_) => { 
                    println!("💤 Worker {} shutting down.", id); 
                    break;
                }
            }
        });

        Worker { // Build the Worker struct.
            id, // Store worker ID.
            thread: Some(thread), // Store the JoinHandle inside an Option.
        }
    }
}

fn main() { // Program entry point.
    println!("- Thread Pool Demo -"); // Print banner.

    // Create a pool with 2 worker threads.
    let pool = ThreadPool::new(8);

    // Submit 8 tasks to the pool.
    for i in 1..=8 { 
        // Move i into the closure and send it to the pool.
        pool.execute(move || { 
            println!("Task {} is running!", i);
            // Simulate work.
            std::thread::sleep(std::time::Duration::from_millis(5000)); 
            // Task finishes.
            println!("Task {} is done!", i); 
        })
    }

    // Keep main alive long enough to run tasks.
    std::thread::sleep(std::time::Duration::from_secs(10));
}

// A closure is an anonymous function that can capture variables from 
// its surrounding scope, example:
// let x = 5;
// let add = |y| x + y; // closure captures `x`
// println!("{}", add(3)); // 8