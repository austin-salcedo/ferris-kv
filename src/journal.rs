use std::fs::OpenOptions;
use std::io::Write;

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
}