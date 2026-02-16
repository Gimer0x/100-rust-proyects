use std::io::{self, Write};


#[derive(Debug)]
struct Contact {
    id: usize,
    name: String,
    phone: String,
    email: String
}

fn main() {
    // Create a vector of contacts
    let mut contacts: Vec<Contact> = Vec::new();
    let mut next_id = 1;

    loop {
        println!("\nContact Manager:");
        println!("1. Add Contact");
        println!("2. View Contacts");
        println!("3. Search Contact");
        println!("4. Delete Contact");
        println!("5. Exit");

        let choice = input("Enter your choice: ");
        match choice.trim() {
            "1" => {
                let name = input("Name: ");
                let phone = input("Phone: ");
                let email = input("Email: ");
                contacts.push(Contact {
                    id: next_id,
                    name,
                    phone,
                    email
                });
                println!("Contact added with Id: {}", next_id);
                next_id +=1;
            }
            "2" => {
                if contacts.len() == 0 {
                    println!("No contacts!");
                } else {
                    for c in &contacts {
                        println!("[{}] {} | {} | {}", c.id, c.name, c.phone, c.email);
                    }
                }
            }
            "3" => {
                let query = input("Enter your word ");

                let results: Vec<&Contact> = contacts
                    .iter()
                    .filter(
                        |c| c.name.contains(&query) ||
                            c.email.contains(&query))
                    .collect();
                
                if results.len() == 0 {
                    println!("No results!");
                } else {
                    for c in results {
                        println!("[{}] {} | {} | {}", c.id, c.name, c.phone, c.email);
                    }
                }
            }
            "4" => {
                let id = input("Enter Id to delete").parse::<usize>().unwrap_or(0);
                let len_before = contacts.len();
                contacts.retain(|c| c.id != id);

                if contacts.len() < len_before {
                    println!("Contact deleted!");
                } else {
                    println!("Id not found!");
                }
            }
            "5" => {
                println!("Good bye!");
                break;
            }
            _ => {
                println!("Incorrect choice!");
            }

        }
    }
}

fn input(prompt: &str) -> String{
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();

    io::stdin()
        .read_line(&mut buf)
        .unwrap();

    buf.trim().to_string()
}
