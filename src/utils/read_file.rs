use std::fs;

/// Reads the contents of a file at the specified path and returns it as a String.
pub fn cat(path: &str) -> String {
    fs::read_to_string(path).unwrap_or("".to_string())
}
