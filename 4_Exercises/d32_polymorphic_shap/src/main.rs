/*
Build a CLI app that uses traits and polymorphism to define different shapes (like 
Circle, Rectangle, and Triangle) and render their area dynamically. This project 
teaches trait-based polymorphism, dynamic dispatch, and runtime behavior abstraction.
This project sharpens your understanding of trait-based interfaces, dynamic polymorphism, 
and how to write flexible code that can handle multiple types with shared behavior.
*/
use std::io::{self, Write};
use std::f64::consts::PI;

trait Shape {
    fn name(&self) -> &str;
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}
impl Shape for Circle {
    fn name(&self) -> &str{
        "Circle"
    }

    fn area(&self) -> f64{
        PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64
}

impl Shape for Rectangle {
    fn name(&self) -> &str{
        "Rectangle"
    }

    fn area(&self) -> f64{
        self.width * self.height
    }
}

struct Triangle {
    base: f64,
    height: f64
}
impl Shape for Triangle{
    fn name(&self) -> &str{
        "Triangle"
    }

    fn area(&self) -> f64{
        self.base * self.height * 0.5
    }
}

fn main() {
    println!("Shape Area Calculator");

    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
    loop{
        println!("\n1. Add Circle\n2. Add Rectangle\n3. Add Triangle\n4. Show All Areas\n5. Exit");

        match input("Choose an option: ").as_str(){
            "1" => {
                let r = input("Enter radius: ").parse::<f64>().unwrap_or(0.0);
                shapes.push(Box::new(Circle {radius: r}));
            }
            "2" => {
                let w = input("Enter width: ").parse::<f64>().unwrap_or(0.0);
                let h = input("Enter height: ").parse::<f64>().unwrap_or(0.0);
                shapes.push(Box::new(Rectangle{width: w, height: h}));
            }
            "3" => {
                let b = input("Enter base: ").parse::<f64>().unwrap_or(0.0);
                let h = input("Enter height: ").parse::<f64>().unwrap_or(0.0);
                shapes.push(Box::new(Triangle{base: b, height: h}));
            }
            "4" => {
                for (i, shape) in shapes.iter().enumerate() {
                    println!("{}.- Name: {} - Area: {:.2}", i + 1, shape.name(), shape.area())
                }
            }
            "5" => break,
            _ => println!("Invalid option")
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}