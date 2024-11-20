// Team: AJ Valentino and Lauren Ferlito
use std::sync::mpsc;
use std::thread;
use std::io::{self, Write};

fn main() {
    // Create channels for message passing between servers
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let (tx3, rx3) = mpsc::channel();

    // Spawn the server processes
    thread::spawn(move || serv1(rx1, tx2));
    thread::spawn(move || serv2(rx2, tx3));
    thread::spawn(move || serv3(rx3));

    // Main loop to read user input and send messages to serv1
    loop {
        let mut input = String::new();
        
        // Wait for user input before processing results
        print!("Enter a message (or 'all_done' to exit): ");
        io::stdout().flush().unwrap(); // Ensure the prompt is shown

        io::stdin().read_line(&mut input).unwrap();
        let message = input.trim().to_string();

        if message == "all_done" {
            tx1.send(Message::Halt).unwrap();
            println!("Shutting down...");
            break;
        } else {
            match parse_message(&message) {
                Ok(msg) => tx1.send(msg).unwrap(),
                Err(err) => println!("Not Handled: {}", err),
            }
        }
    }
}

// Message types that the servers will handle
#[derive(Debug)]
enum Message {
    Halt,
    Add(i32, i32),
    Sub(i32, i32),
    Mult(i32, i32),
    Div(i32, i32),
    Neg(i32),
    Sqrt(i32),
    List(Vec<f64>),
    Error(String),
    Unhandled,
}

// Function to parse user input into a message
fn parse_message(input: &str) -> Result<Message, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return Err("Empty input".to_string());
    }

    match parts[0] {
        "add" => {
            if parts.len() == 3 {
                let left = parts[1].parse::<i32>().map_err(|_| "Invalid number".to_string())?;
                let right = parts[2].parse::<i32>().map_err(|_| "Invalid number".to_string())?;
                Ok(Message::Add(left, right))
            } else {
                Err("Invalid add command".to_string())
            }
        }
        "sub" => {
            if parts.len() == 3 {
                let left = parts[1].parse::<i32>().map_err(|_| "Invalid number".to_string())?;
                let right = parts[2].parse::<i32>().map_err(|_| "Invalid number".to_string())?;
                Ok(Message::Sub(left, right))
            } else {
                Err("Invalid sub command".to_string())
            }
        }
        "mult" => {
            if parts.len() == 3 {
                let left = parts[1].parse::<i32>().map_err(|_| "Invalid number".to_string())?;
                let right = parts[2].parse::<i32>().map_err(|_| "Invalid number".to_string())?;
                Ok(Message::Mult(left, right))
            } else {
                Err("Invalid mult command".to_string())
            }
        }
        "div" => {
            if parts.len() == 3 {
                let left = parts[1].parse::<i32>().map_err(|_| "Invalid number".to_string())?;
                let right = parts[2].parse::<i32>().map_err(|_| "Invalid number".to_string())?;
                Ok(Message::Div(left, right))
            } else {
                Err("Invalid div command".to_string())
            }
        }
        "neg" => {
            if parts.len() == 2 {
                let num = parts[1].parse::<i32>().map_err(|_| "Invalid number".to_string())?;
                Ok(Message::Neg(num))
            } else {
                Err("Invalid neg command".to_string())
            }
        }
        "sqrt" => {
            if parts.len() == 2 {
                let num = parts[1].parse::<i32>().map_err(|_| "Invalid number".to_string())?;
                Ok(Message::Sqrt(num))
            } else {
                Err("Invalid sqrt command".to_string())
            }
        }
        "list" => {
            let list: Vec<f64> = parts[1..]
                .iter()
                .map(|&s| s.parse::<f64>().unwrap_or(0.0))
                .collect();
            Ok(Message::List(list))
        }
        "Error" => {
            // Handle the 'Error' command and return it as an Error message
            if parts.len() > 1 {
                let error_message = parts[1..].join(" ");
                Ok(Message::Error(error_message))
            } else {
                Err("Error message must contain a description".to_string())
            }
        }
        _ => Err("Unknown command".to_string()),
    }
}

// Server 1 that handles arithmetic operations and forwards messages
fn serv1(rx: mpsc::Receiver<Message>, tx: mpsc::Sender<Message>) {
    for msg in rx {
        match msg {
            Message::Halt => {
                println!("(serv1) Halting...");
                tx.send(Message::Halt).unwrap();
                break;
            }
            Message::Add(x, y) => {
                println!("(serv1) Adding: {} + {} = {}", x, y, x + y);
            }
            Message::Sub(x, y) => {
                println!("(serv1) Subtracting: {} - {} = {}", x, y, x - y);
            }
            Message::Mult(x, y) => {
                println!("(serv1) Multiplying: {} * {} = {}", x, y, x * y);
            }
            Message::Div(x, y) => {
                if y != 0 {
                    println!("(serv1) Dividing: {} / {} = {}", x, y, x as f64 / y as f64);
                } else {
                    println!("(serv1) Division by zero error");
                }
            }
            Message::Neg(x) => {
                println!("(serv1) Negating: -{} = {}", x, -x);
            }
            Message::Sqrt(x) => {
                if x >= 0 {
                    println!("(serv1) Sqrt: sqrt({}) = {}", x, (x as f64).sqrt());
                } else {
                    println!("(serv1) Cannot compute sqrt of a negative number");
                }
            }
            _ => tx.send(msg).unwrap(), // Forward unhandled messages to serv2
        }
    }
}

// Server 2 that handles lists and forwards unhandled messages to serv3
fn serv2(rx: mpsc::Receiver<Message>, tx: mpsc::Sender<Message>) {
    for msg in rx {
        match msg {
            Message::Halt => {
                println!("(serv2) Halting...");
                tx.send(Message::Halt).unwrap();
                break;
            }
            Message::List(list) => {
                if let Some(first) = list.first() {
                    if *first == first.floor() {
                        // It's an integer, sum the list
                        let sum: f64 = list.iter().sum();
                        println!("(serv2) Summing: {:?}", list);
                        println!("(serv2) Sum: {}", sum);
                    } else {
                        // It's not an integer, multiply the list
                        let product: f64 = list.iter().product();
                        println!("(serv2) Multiplying: {:?}", list);
                        println!("(serv2) Product: {}", product);
                    }
                }
            }
            _ => tx.send(msg).unwrap(), // Forward unhandled messages to serv3
        }
    }
}

// Server 3 that handles unprocessed messages and error messages
fn serv3(rx: mpsc::Receiver<Message>) {
    fn handle_messages(rx: mpsc::Receiver<Message>, unhandled_count: usize) {
        match rx.recv() {
            Ok(msg) => {
                match msg {
                    // Handle the halt message
                    Message::Halt => {
                        println!("(serv3) Halting...");
                        println!("(serv3) Total unhandled messages: {}", unhandled_count);
                    }
                    // Handle the error message
                    Message::Error(e) => {
                        println!("(serv3) Error: {}", e);
                        handle_messages(rx, unhandled_count); // continue with recursion
                    }
                    // Handle unprocessed messages
                    _ => {
                        println!("(serv3) Not Handled: {:?} | Unhandled command number: {}", msg, unhandled_count + 1);
                        handle_messages(rx, unhandled_count + 1); // increment unhandled count and recurse
                    }
                }
            }
            Err(_) => {
                // If there is an error in receiving a message, exit gracefully
                println!("(serv3) Receiver error.");
            }
        }
    }

    // Start the recursive function with the initial unhandled count set to 0
    handle_messages(rx, 0);
}
