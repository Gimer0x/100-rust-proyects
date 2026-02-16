use std::io::{self, Write};
 
#[derive(Debug)]
struct Account {
    id: usize,
    name: String,
    balance: f64,
}

fn main() {
    let mut accounts: Vec<Account> = Vec::new();

    let mut next_id = 1;

    loop {
        println!("\nBanking System:");
        println!("1. Create Account");
        println!("2. View Balance");
        println!("3. Deposit");
        println!("4. Withdraw");
        println!("5. Exit");

        match get_input("Select your choice: ").as_str() {
            "1" => {
                let name = get_input("Account holder name: ");
                let amount = get_input("Initial deposit: ").parse::<f64>().unwrap_or(0.0);
                accounts.push(Account {id: next_id, name, balance: amount});
                println!("Account created with ID: {}", next_id);
                next_id += 1;
            }
            "2" => {
                let id = get_input("Account Id: ").parse::<usize>().unwrap_or(0);
                match accounts.iter().find(|acc| acc.id == id) {
                    Some(acc) => { println!("{} balance is {:.2}", acc.name, acc.balance); },
                    None => println!("Account not found!")
                }
            }
            "3" => {
                let id = get_input("Account Id: ").parse::<usize>().unwrap_or(0);
                let amount = get_input("Deposit amount: ").parse::<f64>().unwrap_or(0.0);

                match accounts.iter_mut().find(|acc| acc.id == id) {
                    Some(acc) => {
                        acc.balance += amount;
                        println!("New balance: {}", acc.balance);
                    },
                    None => println!("No id found!")
                }
            }
            "4" => {
                let id = get_input("Account Id: ").parse::<usize>().unwrap_or(0);
                let amount = get_input("Amount to withdraw: ").parse::<f64>().unwrap_or(0.0);

                match accounts.iter_mut().find(|acc| acc.id == id) {
                    Some(acc) => {
                        if acc.balance >= amount {
                            acc.balance -= amount;
                            println!("New balance: {}", acc.balance);
                        } else {
                            println!("Not enough funds!");
                        }
                        
                    },
                    None => println!("No id found!")
                }
            }
            "5" => {
                println!("Good bye");
                break;
            }
            _ => {
                println!("Invalid input!");
            }
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .unwrap();
    buf.trim().to_string()
}
