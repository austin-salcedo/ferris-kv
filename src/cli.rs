use crate::JOURNAL_PATH;
use crate::journal;
use std::collections::HashMap;

pub const SET_CMD: &str = "SET";
pub const REMOVE_CMD: &str = "REMOVE";

// A clean, isolated function that handles the SET logic
pub fn handle_set(store: &mut HashMap<String, String>, key: String, value: String) -> String {
    store.insert(key.clone(), value.clone());
    if let Err(e) = journal::append_to_journal(JOURNAL_PATH, SET_CMD, &key, Some(&value)) {
        eprintln!("Warning: Failed to persist to disk: {}", e)
    }
    String::from("OK")
}

pub fn handle_get(store: &HashMap<String, String>, key: String) -> String {
    match store.get(&key) {
        Some(val) => format!("\"{}\"", val),
        None => String::from("(nil)"),
    }
}

pub fn handle_remove(store: &mut HashMap<String, String>, key: String) -> String {
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