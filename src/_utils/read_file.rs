use std::fs;

pub fn cat(path: &str) -> String {
    fs::read_to_string(path).unwrap_or("".to_string())
}