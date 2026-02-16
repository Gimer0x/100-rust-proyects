use std::sync::mpsc::{self, SyncSender};
use std::thread;
use std::time::Duration;

// MPSC stands for “multiple producer, single consumer.”
// It’s Rust’s standard channel module for sending values between threads:
// 
// use std::sync::mpsc;
// // let (tx, rx) = mpsc::channel();
// 
// + Multiple producers: you can clone tx and send from many threads.
// + Single consumer: only one receiver rx is allowed.
// So it’s a thread‑safe way to pass messages (data) from many threads into one receiving thread.
 
fn main() {
    println!("Producer-Consumer Demo");
 
    let (tx, rx) = mpsc::sync_channel(3); // bounded buffer size = 3
 
    // Producer thread
    let producer = thread::spawn(move || {
        for i in 1..=10 {
            println!("Producing task {}", i);
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(300)); // simulate work
        }
        println!("✅ Producer finished.");
    });
 
    // Consumer thread
    let consumer = thread::spawn(move || {
        while let Ok(task) = rx.recv() {
            println!("📥 Consuming task {}", task);
            thread::sleep(Duration::from_millis(500)); // simulate slower work
        }
    });
 
    producer.join().unwrap();
    thread::sleep(Duration::from_secs(2)); // allow consumer to finish
    println!("All tasks processed.");
}

// what is the difference between this mpsc::channel::<String>(); and let (tx, rx) = mpsc::sync_channel(3)
// 
// Short answer:
// + mpsc::channel::<String>() = unbounded channel (async).
// + send() never blocks (unless receiver is gone).
// - mpsc::sync_channel(3) = bounded channel with capacity 3 (sync).
// - send() blocks when the buffer is full until a receiver takes a value.
// 
// More detail
// 
// mpsc::channel::<String>()
// let (tx, rx) = mpsc::channel::<String>();
// Unlimited queue size (can grow)
// Sender doesn’t block
// Good when you don’t want backpressure
// 
// mpsc::sync_channel(3)
// let (tx, rx) = mpsc::sync_channel(3);
// Queue capacity = 3
// Sender blocks when 3 items are already queued
// Good for backpressure and controlling memory use

// In practice:
// - Use channel when you want fast, fire‑and‑forget sending.
// - Use sync_channel when you want to limit buffering and slow producers if consumers are behind.