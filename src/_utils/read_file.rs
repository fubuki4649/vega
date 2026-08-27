use std::fs;

#[allow(dead_code)]
pub fn cat(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_default()
}