// Day 97: Project: CLI Soundboard App (with rodio Audio Playback)
// Description:
// Build a fun CLI tool that plays sound clips using number-based selection. You’ll use 
// the rodio crate for cross-platform audio playback, and practice file I/O, audio streaming, 
// and clean CLI menus.
// Key Concepts:
// + rodio::Decoder to stream audio from file
// + Sink to control playback
// + Buffered I/O with BufReader for file decoding
// + CLI selection logic with HashMap
// You now have a working CLI soundboard, easily expandable into a GUI, game engine, or alert system.
use rodio::{Decoder, OutputStreamBuilder, Sink};
use std::fs::File;
use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    println!("CLI Soundboard");
 
    // Map of key -> filename
    let sounds: HashMap<u8, &str> = [
        (1, "sounds/birds.wav"),
        (2, "sounds/ding.mp3"),
        (3, "sounds/wow.mp3"),
    ]
    .iter()
    .cloned()
    .collect();
 
    for (key, name) in &sounds {
        println!("{}. {}", key, name);
    }

    print!("Choose a sound to play (1-3): ");
    io::stdout().flush().unwrap();
 
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
 
    match input.trim().parse::<u8>() {
        Ok(choice) if sounds.contains_key(&choice) => {
            let path = sounds.get(&choice).unwrap();

            let stream_handle = OutputStreamBuilder::open_default_stream().expect("Failed to open audio stream");
            let sink = Sink::connect_new(stream_handle.mixer());
            match File::open(path) {
                Ok(file) => {
                    let source = Decoder::try_from(file).expect("Failed to decode audio");
        
                    sink.append(source);
                    sink.sleep_until_end();
                },
                Err(e) => println!("File not found! {}", e)
            }
        }
        _ => println!("Invalid choice."),
    }
}
