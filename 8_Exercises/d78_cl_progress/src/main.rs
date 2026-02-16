// Day 78: Project: Command-Line Progress Bar (with indicatif)
// Today you’ll build a CLI app that simulates work using a beautiful progress 
// bar from the indicatif crate. This teaches you how to improve terminal UX 
// and handle synchronous and async progress tracking.
// Key Concepts:
// + ProgressBar::new(n) creates a counter
// + ProgressStyle::with_template() customizes display
// + bar.inc(1) and bar.set_message(...) animate progress
// + bar.finish_with_message() finalizes it
// 
// Optional Enhancements:
// + Add a CLI flag for total steps
// + Use tokio::time::sleep() for async simulation
// + Show file download progress using .wrap_read()
// You now know how to add professional polish to CLI tools using elegant 
// progress bars and real-time updates.
use indicatif::{ProgressBar, ProgressStyle};
use std::{thread, time::Duration};
 
fn main() {
    println!("🚀 Starting simulated task...");
 
    let bar = ProgressBar::new(100);
    bar.set_style(
        ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>3}/{len} {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );
 
    for i in 0..100 {
        bar.set_message(format!("step {}", i));
        bar.inc(1);
        thread::sleep(Duration::from_millis(50));
    }
 
    bar.finish_with_message("✅ Done!");
}