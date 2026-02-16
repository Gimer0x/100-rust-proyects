// Day 34: Project: Command Pattern with Traits
// Implement the Command design pattern using traits. This pattern encapsulates commands as objects, 
// allowing actions to be stored, queued, undone, or executed dynamically. You'll simulate a CLI remote 
// control for devices like Light and Fan, each implementing Command.
// This project gives you a clean intro to the Command pattern, widely used in GUI systems, game engines, 
// undo/redo systems, and remote operation control.
use std::io::{self, Write};

trait Command{
    fn execute(&self);
}

struct LightOn;
impl Command for LightOn {
    fn execute(&self) {
        println!("Light turned On");
    }
}

struct LightOff;
impl Command for LightOff {
    fn execute(&self) {
        println!("Light turned Off.");
    }
}

struct FanOn;
impl Command for FanOn {
    fn execute(&self) {
        println!("Fan turned ON.");
    }
}
 
struct FanOff;
impl Command for FanOff {
    fn execute(&self) {
        println!("Fan turned OFF.");
    }
}

// Remote control that executes boxed commands
struct Remote {
    history: Vec<String>,
}

impl Remote {
    fn new() -> Self {
        Remote { history: Vec::new() }
    }

    fn press_button(&mut self, label: &str, command: &dyn Command) {
        println!("Executing {}", label);
        command.execute();
        self.history.push(label.to_string());
    }

    fn show_history(&self) {
        if self.history.is_empty() {
            println!("No commands executed!");
        } else {
            for (i, cmd) in self.history.iter().enumerate() {
                println!("{}. {}", i + 1, cmd);
            }
        }
    }
}

fn main() {
    let mut remote = Remote::new();

    loop {
        println!("\nRemote Control Menu:");
        println!("1. Light ON");
        println!("2. Light OFF");
        println!("3. Fan ON");
        println!("4. Fan OFF");
        println!("5. Show History");
        println!("6. Exit");
 
        let choice = input("Choose an action: ");

        match choice.as_str() {
            "1" => remote.press_button("Light ON", &LightOn),
            "2" => remote.press_button("Light OFF", &LightOff),
            "3" => remote.press_button("Fan ON", &FanOn),
            "4" => remote.press_button("Fan OFF", &FanOff),
            "5" => remote.show_history(),
            "6" => {
                println!("Powering down remote.");
                break;
            }
            _ => println!("Invalid choice."),
        }
    }
}

fn input(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
