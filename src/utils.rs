use std::fs;

pub fn read_lines(path: &str) -> Vec<String> {
    fs::read_to_string(path).expect("Error reading file")
        .split("\n")
        .map(|string| string.to_string())
        .collect()
}