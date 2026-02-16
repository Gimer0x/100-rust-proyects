use std::fmt::Debug;

struct Stack<T> {
    elements: Vec<T>
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { elements: Vec::new() }
    }

    fn push(&mut self, item: T){
        self.elements.push(item);
        println!("Item pushed!");
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }
 
    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
 
    fn size(&self) -> usize {
        self.elements.len()
    }
}

impl<T: Debug> Stack<T> {
    fn print(&self) {
        println!("Stack: {:?}", self.elements);
    }
}

fn main() {
    let mut stack: Stack<String> = Stack::new();

    loop {
        println!("\n📚 Generic Stack Menu:");
        println!("1. Push");
        println!("2. Pop");
        println!("3. Peek");
        println!("4. Size");
        println!("5. Print Stack");
        println!("6. Exit");
 
        let choice = input("Enter your choice: ");

        match choice.as_str() {
            "1" => {
                let val = input("Enter value to push: ");
                stack.push(val);
            }
            "2" => {
                if stack.is_empty() {           
                    println!("Stack empty!");
                    continue;
                }

                if let Some(val) = stack.pop() {
                    println!("Value popped: {}", val);
                }
            }
            "3" => match stack.peek() {
                Some(val) => println!("Top: {}", val),
                None => println!("Stack is empty."),
            },
            "4" => println!("Stack size: {}", stack.size()),
            "5" => stack.print(),
            "6" => break,
            _ => println!("Incorrect choice!")
        }
    }
}

fn input(prompt: &str) -> String {
    use std::io::{self, Write};    
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
