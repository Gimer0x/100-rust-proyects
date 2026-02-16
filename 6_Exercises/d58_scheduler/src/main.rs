// Day 58: Project: Scheduler Simulation
// Build a simple job/task scheduler simulation in Rust. Each task has a name and delay, 
// and the scheduler runs them with a delay-based simulation using std::thread::sleep. 
// This teaches task queues, basic scheduling, and async thinking (without async code).
// Key Concepts:
// + thread::sleep() for simulating delay.
// + Instant::now() for tracking elapsed time.
// + Vec<Task> as a basic task queue.
// You now understand the fundamentals of job scheduling, a key concept for building 
// cron replacements, CI runners, or process orchestrators.
use std::thread;
use std::time::{Duration, Instant};


#[derive(Debug)]
struct Task {
    name: String,
    delay_secs: u64,
}

fn main() {
    println!("Task scheduler simulation!");

    let mut tasks = vec![
        // "Backup".into() converts the value to whatever type the compiler expects in this place.
        Task { name: "Backup".into(), delay_secs: 3 },
        Task { name: "Clean Temp Files".into(), delay_secs: 1 },
        Task { name: String::from("Send Report"), delay_secs: 2 },
    ];

    tasks.sort_by_key(|t| t.delay_secs);

    let start = Instant::now();

    for task in tasks {
        let wait = Duration::from_secs(task.delay_secs);
        println!("Waiting {}s to run '{}'", task.delay_secs, task.name);
        thread::sleep(wait);
        let elapsed = Instant::now().duration_since(start).as_secs();
        println!("[{}s] Task '{}' completed", elapsed, task.name);
    }

    println!("All tasks completed.");

}
