use std::collections::HashMap;
use std::io;
mod journal;

const JOURNAL_PATH: &str = "journal.txt";
const SET_CMD: &str = "SET";
const GET_CMD: &str = "GET";
const REMOVE_CMD: &str = "REMOVE";

// A clean, isolated function that handles the SET logic
fn handle_set(store: &mut HashMap<String, String>, key: String, value: String) -> String {
    store.insert(key.clone(), value.clone());
    if let Err(e) = journal::append_to_journal(JOURNAL_PATH, SET_CMD, &key, Some(&value)) {
        eprintln!("Warning: Failed to persist to disk: {}", e)
    }
    String::from("OK")
}

fn handle_get(store: &HashMap<String, String>, key: String) -> String {
    match store.get(&key) {
        Some(val) => {
            if let Err(e) = journal::append_to_journal(JOURNAL_PATH, GET_CMD, &key, None) {
                eprintln!("Warning: Failed to persist to disk: {}", e)
            }
            format!("\"{}\"", val)
        },
        None => String::from("(nil)"),
    }
}

fn handle_remove(store: &mut HashMap<String, String>, key: String) -> String {
    match store.remove(&key) {
        Some(val) => {
            if let Err(e) = journal::append_to_journal(JOURNAL_PATH, REMOVE_CMD, &key, None) {
                eprintln!("Warning: Failed to persist to disk: {}", e)
            }
            format!("Removed: \"{}\"", val)
        },
        None => String::from("(nil)"),
    }
}

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
                    
                    let response = handle_set(&mut store, key, value);
                    println!("{}", response);
                } else {
                    println!("Error: SET requires a key and a value");
                }
            }
            "get" => {
                if parts.len() == 2 {
                    let key = parts[1];
                    let response = handle_get(&store, key.to_string());
                    println!("{}", response);
                } else {
                    println!("Error: GET requires a key");
                }
            }
            "remove" => {
                if parts.len() == 2 {
                    let response = handle_remove(&mut store, parts[1].to_string());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_set_overwrites_existing_key() {
        let map = &mut HashMap::new();
        let response = handle_set(map, "name".to_string(), "austin".to_string());
        assert_eq!(response, "OK");
        assert_eq!(map.get("name"), Some(&"austin".to_string()));

        let response = handle_set(map, "name".to_string(), "bob".to_string());
        assert_eq!(response, "OK");
        assert_eq!(map.get("name"), Some(&"bob".to_string()));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_handle_get_existing_and_missing_keys() {
        let map = &mut HashMap::new();
        
        // 1. Test missing key state (Should return the Redis-style "(nil)" string)
        let missing_response = handle_get(map, "ghost_key".to_string());
        assert_eq!(missing_response, "(nil)");

        // 2. Test happy path (Should return the value wrapped in literal quotes)
        map.insert("framework".to_string(), "rust".to_string());
        let happy_response = handle_get(map, "framework".to_string());
        assert_eq!(happy_response, "\"rust\"");
    }

    #[test]
    fn test_handle_remove_existing_and_missing_keys() {
        let map = &mut HashMap::new();

        // 1. Test removing a missing key (Should return "(nil)" and map length stays 0)
        let missing_remove = handle_remove(map, "ghost_key".to_string());
        assert_eq!(missing_remove, "(nil)");
        assert_eq!(map.len(), 0);

        // 2. Test removing an existing key (Should return confirmation and clear the key)
        map.insert("target_key".to_string(), "delete_me".to_string());
        assert_eq!(map.len(), 1);

        let happy_remove = handle_remove(map, "target_key".to_string());
        assert_eq!(happy_remove, "Removed: \"delete_me\"");
        assert_eq!(map.get("target_key"), None);
        assert_eq!(map.len(), 0); // Confirms memory was cleanly freed
    }
}