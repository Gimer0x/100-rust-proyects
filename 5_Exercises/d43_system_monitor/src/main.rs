// Day 43: Project: System Resource Monitor
// Build a simple system resource monitor that displays your machine’s CPU 
// usage, memory consumption, and running processes. You’ll use the sysinfo 
// crate to access OS-level data—great for monitoring tools and diagnostics apps.

// Key Concepts:
// + System::new_all() – Initialize system with full data.
// + refresh_all() – Updates system snapshot.
// + cpus(), processes(), used_memory() – Resource APIs.
// This project gives you the foundation for building desktop widgets, 
// devops dashboards, or embedded monitoring agents.
use sysinfo::System;
use std::thread;
use std::time::Duration;

fn main() {
    println!("System Resource Monitor!");
    let mut sys = System::new_all();

    loop {
        sys.refresh_all();

        let total_memory = sys.total_memory() / 1024;
        let used_memory = sys.used_memory() / 1024;

        println!("\n==============================");
        println!("CPU Usage:");

        for (i, cpu) in sys.cpus().iter().enumerate(){
            println!("Core {}: {:.2}%", i, cpu.cpu_usage());
        }

        println!("Memory Usage: {} MB / {} MB", used_memory, total_memory);
        println!("Total Processes: {}", sys.processes().len());

        println!("Top 5 Processes by CPU:");

        let mut processes: Vec<_> = sys.processes().values().collect();
        // Sort the processes
        processes.sort_by(|a, b| b.cpu_usage()
            .partial_cmp(&a.cpu_usage())
            .unwrap());

        for proc in processes.iter().take(5) {
            println!(
                "PID: {:<6} CPU: {:>5.1}%  Name: {:?}",
                proc.pid(),
                proc.cpu_usage(),
                proc.name()
            );
        }

        println!("==============================");
        thread::sleep(Duration::from_secs(2));
    }
}
