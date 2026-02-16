// Day 47: Project: Process Manager
// Create a CLI tool that lists currently running processes, lets you search by name, 
// and optionally terminate a process by PID. This project teaches you process introspection 
// and control, using the sysinfo crate.

// Key Concepts:
// + sysinfo::System gives access to process table.
// + process.kill_with(Signal::Kill) for safe termination.
// + Process filtering and sorting logic.
// This project sets you up for building your own system monitors, task managers, or even 
// remote process controllers.
use sysinfo::{System, Signal, Pid};
use std::io::{self, Write};

fn main() {
    println!("Rust Process Manager");

    let mut sys = System::new_all();
    sys.refresh_all();

    loop {
        println!("\n1. List all processes");
        println!("2. Search process by name");
        println!("3. Kill process by PID");
        println!("4. Exit");
 
        let choice = prompt("Choose an option: ");
 
        match choice.as_str() {
            "1" => list_processes(&sys),
            "2" => { 
                let name = prompt("Enter name to search: ");
                search_processes(&mut sys, &name);
            }
            "3" => {
                let pid_str = prompt("Enter PID to kill: ");
                if let Ok(pid) = pid_str.parse::<i32>() {
                    kill_process(pid);
                } else {
                    println!("Invalid PID");
                }
            }
            "4" => break,
            _ => println!("Invalid Option!")
        }
    }
}

fn list_processes(sys: &System) {
    println!("{:<8} {:<20} {:<10}", "PID", "Name", "CPU%");
    for proc in sys.processes().values() {
        println!("{:<8} {:<20} {:>6.2}", proc.pid(), proc.name().to_string_lossy(), proc.cpu_usage());
    }
}

fn search_processes(sys: &mut System, keyword: &str) {
    let keyword = keyword.to_lowercase();
    sys.refresh_all();

    let found: Vec<_> = sys
        .processes()
        .values()
        .filter(|p| p.name().to_ascii_lowercase().to_string_lossy().contains(&keyword))
        .collect();

    if found.is_empty() {
        println!("No processes found with name containing '{}'.", keyword);
    } else {
        println!("{:<8} {:<20} {:<10}", "PID", "Name", "CPU%");
        for p in found {
            println!("{:<8} {:<20} {:>6.2}", p.pid(), p.name().to_string_lossy(), p.cpu_usage());
        }
    }
}

fn kill_process(pid: i32) {
    let pid: Pid = Pid::from(pid as usize);
    //let pid: Pid = (pid as usize).into();

    let mut sys = System::new();
    sys.refresh_all();

    if let Some(process) = sys.process(pid) {
        if process.kill_with(Signal::Kill).is_some(){
            println!("Killed process {} - {:?}", pid, process.name());
        } else {
            println!("Failed to send kill signal!");
        }
    } else {
        println!("Process with PID: {}, not found!", pid)
    }
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
