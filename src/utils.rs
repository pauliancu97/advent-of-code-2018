use std::fs;

use crate::day_three::Matrix;

pub fn read_lines(path: &str) -> Vec<String> {
    fs::read_to_string(path).expect("Error reading file")
        .split("\n")
        .map(|string| string.to_string())
        .collect()
}

pub fn read_matrix(path: &str) -> Matrix<char> {
    let lines = read_lines(path);
    let mut matrix: Matrix<char> = Matrix::new(lines.len(), lines[0].len(), ' ');
    for row in 0..lines.len() {
        for col in 0..lines[0].len() {
            matrix.set(row, col, lines[row].chars().nth(col).unwrap());
        }
    }
    matrix
}