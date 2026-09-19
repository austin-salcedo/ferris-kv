use std::collections::HashMap;
use std::io;
mod journal;
mod cli;

const JOURNAL_PATH: &str = "journal.txt";

fn main() {
    // Our 'Database' - a HashMap that maps Strings to Strings
    let mut store: HashMap<String, String> = HashMap::new();

    println!("Ferris-KV Version 0.2");

    if let Err(e) = journal::restore_journal(JOURNAL_PATH, &mut store) {
        eprintln!("Warning: error while restoring memory from previous run: {}", e);
    } else {
        println!("Successfully restored memory from previous run.")
    };

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
                    
                    let response = cli::handle_set(&mut store, key, value);
                    println!("{}", response);
                } else {
                    println!("Error: SET requires a key and a value");
                }
            }
            "get" => {
                if parts.len() == 2 {
                    let key = parts[1];
                    let response = cli::handle_get(&store, key.to_string());
                    println!("{}", response);
                } else {
                    println!("Error: GET requires a key");
                }
            }
            "remove" => {
                if parts.len() == 2 {
                    let response = cli::handle_remove(&mut store, parts[1].to_string());
                    println!("{}", response);
                } else {
                    println!("Error: REMOVE requires a key");
                }
            }
            "exit" => break,
            _ => println!("Unknown command"),
        }
    }
}
