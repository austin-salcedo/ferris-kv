use std::collections::HashMap;
use std::io;

fn main() {
    // Our 'Database' - a HashMap that maps Strings to Strings
    let mut store: HashMap<String, String> = HashMap::new();

    println!("Ferris-KV Version 0.1");
    println!("Commands: SET key value | GET key | REMOVE key | EXIT");

    loop {
        let mut input = String::new();

        // Read input from the terminal
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Clean up the input (remove the newline character)
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        let command = parts[0].to_lowercase();

        match command.as_str() {
            "set" => {
                if parts.len() == 3 {
                    let key = parts[1].to_string();
                    let value = parts[2].to_string();
                    store.insert(key, value);
                    println!("OK");
                } else {
                    println!("Error: SET requires a key and a value");
                }
            }
            "get" => {
                if parts.len() == 2 {
                    let key = parts[1];
                    match store.get(key) {
                        Some(val) => println!("\"{}\"", val),
                        None => println!("(nil)"),
                    }
                }
            }
            "remove" => {
                if parts.len() == 2 {
                    store.remove(parts[1]);
                    println!("OK");
                }
            }
            "exit" => break,
            _ => println!("Unknown command"),
        }
    }
}
