// Day 79: Project: Text Adventure Game (CLI RPG)
// Create a simple command-line RPG where players move between rooms, collect items, 
// and face decisions. You'll use enums for game states, pattern matching, and a looping 
// input engine—perfect for learning Rust's powerful type system and control flow.
// Key Concepts:
// + Enums to model state (Room)
// + Structs for player and inventory
// + Pattern matching for gameplay logic
// + Game loop using loop {}
// You’ve now built a text-based game engine—a base for narrative games, roguelikes, 
// or interactive fiction.
use std::io::{self, Write};

#[derive(Debug, Clone)]
enum Room {
    Start,
    Forest,
    Cave,
    Treasure,
}
 
#[derive(Debug)]
struct Player {
    name: String,
    inventory: Vec<String>,
    current_room: Room,
}

impl Player {
    fn describe(&self) {
        println!("\nYou are in the {:?}", self.current_room);
        println!("Inventory: {:?}", self.inventory);
    }
 
    fn available_actions(&self) {
        match self.current_room {
            Room::Start => println!("Go to (forest/cave)"),
            Room::Forest => println!("Options: (explore/return)"),
            Room::Cave => println!("Options: (search/return)"),
            Room::Treasure => println!("You found the treasure! Game over."),
        }
    }
}

fn main() {
    println!("Welcome to the Rusty Adventure!");
    print!("Enter your name: ");
    io::stdout().flush().unwrap();
 
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    let mut player = Player {
        name,
        inventory: vec![],
        current_room: Room::Start,
    };

    loop {
        player.describe();
        player.available_actions();
 
        print!("\nWhat will you do? ");
        io::stdout().flush().unwrap();
 
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim().to_lowercase();
 
        match (&player.current_room, command.as_str()) {
            (Room::Start, "forest") => player.current_room = Room::Forest,
            (Room::Start, "cave") => player.current_room = Room::Cave,
 
            (Room::Forest, "explore") => {
                println!("You found a magical leaf!");
                player.inventory.push("magical leaf".to_string());
            }
            (Room::Forest, "return") => player.current_room = Room::Start,
 
            (Room::Cave, "search") => {
                if player.inventory.contains(&"magical leaf".to_string()) {
                    println!("The leaf reveals a hidden door to treasure!");
                    player.current_room = Room::Treasure;
                } else {
                    println!("It’s too dark. You need something magical...");
                }
            }
            (Room::Cave, "return") => player.current_room = Room::Start,
 
            (Room::Treasure, _) => {
                println!("Congratulations, {}! You've completed the game!", player.name);
                break;
            }
 
            _ => println!("Unknown command."),
        }
    }
}
