use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main () {
    println!("Enter time duration: ");

    let duration = match get_time_input() {
        Some(dur) => dur,
        None => {
            println!{"Invalid input!"};
            return;
        }
    };

    println!("Timer set for: {} hours {} minutes {} seconds", duration.0, duration.1, duration.2);
    start_timer(duration.0, duration.1, duration.2);
    println!("Time's up!");
}

fn start_timer(hours: u64, minutes:u64, seconds:u64) {
    let total_seconds = seconds + minutes * 60 + hours * 3600;

    for i in (0..=total_seconds).rev() {
        let hrs = i / 3600;
        let mins = ( i % 3600) / 60;
        let secs = i % 60;

        print!("\rTime Remaining: {:02}:{:02}:{:02}", hrs, mins, secs);
        io::stdout().flush().unwrap();
        if i != 0 { thread::sleep(Duration::from_secs(1)); }
    }
    println!();

}

fn get_time_input() -> Option<(u64, u64, u64)>{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input!");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 3{
        return None;
    }

    // Returns None if parse fails
    // This is equivalent to this:
    // let hours = match parts[0].parse::<u64>() {
    //     Ok(h) => h,
    //     Err(_) => return None,  // Early return
    // }; 
    let hours = parts[0].parse::<u64>().ok()?; 
    let minutes = parts[1].parse::<u64>().ok()?;
    let seconds = parts[2].parse::<u64>().ok()?;

    Some((hours, minutes, seconds))
}