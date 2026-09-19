use std::fs::{OpenOptions};
use std::io::{BufReader, BufRead, Write, Error, ErrorKind};
use std::collections::HashMap;
use crate::cli::{REMOVE_CMD, SET_CMD};

pub fn restore_journal(path: &str, store: &mut HashMap<String, String>) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(path)?;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        let mut parts = line.split('|');

        let command = parts.next().ok_or_else(|| {
            Error::new(ErrorKind::InvalidData, "missing command")
        })?;

        let key = parts.next().ok_or_else(|| {
            Error::new(ErrorKind::InvalidData, "missing key")
        })?;

            match command {
                SET_CMD => {
                    let value = parts.next().ok_or_else(|| {
                    Error::new(ErrorKind::InvalidData, "SET missing value")
                })?;

                store.insert(key.to_string(), value.to_string());
                }

                REMOVE_CMD => {
                    store.remove(key);
                }

                _ => {
                    return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("unknown journal command: {command}"),
                    ));
                }           
            }
    }   
    Ok(())
}

pub fn append_to_journal(path: &str, command: &str, key: &str, value: Option<&str>) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(path)?;

    let log_entry = match value {
        Some(val) => format!("{}|{}|{}\n", command, key, val),
        None => format!("{}|{}|\n", command, key),
    };

    file.write_all(log_entry.as_bytes())?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // Helper function to remove test files after running tests
    fn cleanup(filename: &str) {
        let _ = fs::remove_file(filename);
    }

    #[test]
    fn test_append_to_journal_creates_and_appends() {
        // Guarantee clean state before test runs
        let test_file = "test_journal.txt";

        
        // 1. Act: Append a SET command
        let res = append_to_journal(test_file, "SET", "user", Some("austin"));
        assert!(res.is_ok());

        // 2. Act: Append a REMOVE command
        let res_remove = append_to_journal(test_file, "REMOVE", "user", None);
        assert!(res_remove.is_ok());

        // 3. Assert: Read file back and check contents
        let contents = fs::read_to_string(test_file).expect("Failed to read journal file");
        
        let expected = "SET|user|austin\nREMOVE|user|\n";
        assert_eq!(contents, expected);

        // Clean up afterwards so we don't pollute local workspace
        cleanup(test_file);
    }

   #[test]
    fn test_restore_journal_restores_set_value() {
        let test_file = "test_restore_set_journal.txt";
        cleanup(test_file);

        let contents = format!("{}|user|austin\n", SET_CMD);
        fs::write(test_file, contents).expect("Failed to create test journal");

        let mut store = HashMap::new();

        let result = restore_journal(test_file, &mut store);

        assert!(result.is_ok());
        assert_eq!(store.get("user"), Some(&"austin".to_string()));

        cleanup(test_file);
    }

    #[test]
    fn test_restore_journal_applies_remove() {
        let test_file = "test_restore_remove_journal.txt";
        cleanup(test_file);

        let contents = format!(
            "{}|user|austin\n{}|user|\n",
            SET_CMD, REMOVE_CMD
        );

        fs::write(test_file, contents).expect("Failed to create test journal");

        let mut store = HashMap::new();

        let result = restore_journal(test_file, &mut store);

        assert!(result.is_ok());
        assert_eq!(store.get("user"), None);

        cleanup(test_file);
    }

    #[test]
    fn test_restore_journal_replays_multiple_operations_in_order() {
        let test_file = "test_restore_multiple_journal.txt";
        cleanup(test_file);

        let contents = format!(
            "{}|name|austin\n{}|color|blue\n{}|name|\n{}|color|green\n",
            SET_CMD,
            SET_CMD,
            REMOVE_CMD,
            SET_CMD,
        );

        fs::write(test_file, contents).expect("Failed to create test journal");

        let mut store = HashMap::new();

        let result = restore_journal(test_file, &mut store);

        assert!(result.is_ok());

        assert_eq!(store.get("name"), None);
        assert_eq!(store.get("color"), Some(&"green".to_string()));

        cleanup(test_file);
    }

    #[test]
    fn test_restore_journal_errors_on_unknown_command() {
        let test_file = "test_restore_invalid_journal.txt";
        cleanup(test_file);

        fs::write(test_file, "BANANA|user|austin\n")
            .expect("Failed to create test journal");

        let mut store = HashMap::new();

        let result = restore_journal(test_file, &mut store);

        assert!(result.is_err());

        cleanup(test_file);
    }

    #[test]
    fn test_restore_journal_errors_when_set_has_no_value() {
        let test_file = "test_restore_missing_value_journal.txt";
        cleanup(test_file);

        let contents = format!("{}|user\n", SET_CMD);
        fs::write(test_file, contents).expect("Failed to create test journal");

        let mut store = HashMap::new();

        let result = restore_journal(test_file, &mut store);

        assert!(result.is_err());

        cleanup(test_file);
    }
}
