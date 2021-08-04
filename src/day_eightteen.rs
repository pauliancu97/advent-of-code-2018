use crate::{day_three::{Matrix, MatrixRange}, utils::read_matrix};
use core::f64;
use std::{fmt, sync::mpsc::{Receiver, Sender, channel}, thread::{self, JoinHandle, current}};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Acre {
    Open,
    Tree,
    Lumberyard
}

const OPEN_ACRE_CHAR: char = '.';
const TREE_ACRE_CHAR: char = '|';
const LUMBERYARD_ACRE_CHAR: char = '#';

impl Acre {
    fn from_char(chr: char) -> Acre {
        match chr {
            OPEN_ACRE_CHAR => Acre::Open,
            TREE_ACRE_CHAR => Acre::Tree,
            LUMBERYARD_ACRE_CHAR => Acre::Lumberyard,
            _ => Acre::Open,
        }
    }

    fn to_char(&self) -> char {
        match self {
            &Acre::Open => OPEN_ACRE_CHAR,
            &Acre::Tree => TREE_ACRE_CHAR,
            &Acre::Lumberyard => LUMBERYARD_ACRE_CHAR,
        }
    }
}

impl fmt::Display for Matrix<Acre> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::from("");
        for row in 0..self.rows {
            for col in 0..self.cols {
                string.push(self.get(row, col).to_char());
            }
            string.push('\n');
        }
        write!(f, "{}", &string)
    }
}

const OFFSETS: &[(isize, isize); 8] = &[
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1)
];

fn get_adjacent_acres_info(matrix: &Matrix<Acre>, row: usize, col: usize) -> (usize, usize, usize) {
    let mut num_open_acres: usize = 0;
    let mut num_tree_acres: usize = 0;
    let mut num_lumberyard_acres: usize = 0;
    for &(row_offset, col_offset) in OFFSETS {
        let offseted_row = (row as isize) + row_offset;
        let offseted_col = (col as isize) + col_offset;
        if offseted_row >= 0 && offseted_row < (matrix.rows as isize) &&
            offseted_col >= 0 && offseted_col < (matrix.cols as isize) {
                match matrix.get(offseted_row as usize, offseted_col as usize) {
                    Acre::Open => { num_open_acres += 1 },
                    Acre::Tree => { num_tree_acres += 1 },
                    Acre::Lumberyard => { num_lumberyard_acres += 1 },
                }
        }
    }
    (num_open_acres, num_tree_acres, num_lumberyard_acres)
}

fn get_updated_matrix(matrix: &Matrix<Acre>) -> Matrix<Acre> {
    let mut updated_matrix = matrix.clone();
    for row in 0..matrix.rows {
        for col in 0..matrix.cols {
            let (_, num_tree_acres, num_lumberyard_acres) = get_adjacent_acres_info(
                matrix, 
                row, 
                col
            );
            match matrix.get(row, col) {
                Acre::Open => {
                    if num_tree_acres >= 3 {
                        updated_matrix.set(row, col, Acre::Tree);
                    }
                },
                Acre::Tree => {
                    if num_lumberyard_acres >= 3 {
                        updated_matrix.set(row, col, Acre::Lumberyard);
                    }
                },
                Acre::Lumberyard => {
                    if !(num_lumberyard_acres >= 1 && num_tree_acres >= 1) {
                        updated_matrix.set(row, col, Acre::Open);
                    }
                },
            }
        }
    }
    updated_matrix
}

fn get_acres_matrix(char_matrix: &Matrix<char>) -> Matrix<Acre> {
    char_matrix.map(Acre::Open, |&chr| {
        Acre::from_char(chr)
    })
}

fn get_acres_matrix_after_iterations(matrix: &Matrix<Acre>, num_iterations: usize) -> Matrix<Acre> {
    let mut current_matrix = matrix.clone();
    for _ in 0..num_iterations {
        current_matrix = get_updated_matrix(&current_matrix);
    }
    current_matrix
}

fn get_acres_matrix_after_large_iterations(matrix: &Matrix<Acre>, num_iterations: usize) -> Matrix<Acre> {
    let mut current_matrix = matrix.clone();
    let mut matrices: Vec<Matrix<Acre>> = Vec::new();
    matrices.push(current_matrix.clone());
    for _ in 0..num_iterations {
        current_matrix = get_updated_matrix(&current_matrix);
        if let Some(index) = matrices.iter().position(|mat| *mat == current_matrix) {
            let cycle_length = matrices.len() - index;
            let matrix_index = (num_iterations - index) % cycle_length + index;
            return matrices[matrix_index].clone();
        } else {
            matrices.push(current_matrix.clone());
        }
    }
    current_matrix
}

fn get_matrix_ranges(matrix_range: &MatrixRange, num: usize) -> Vec<MatrixRange> {
    let mut current_matrix_ranges: Vec<MatrixRange> = Vec::new();
    current_matrix_ranges.push(matrix_range.clone());
    for index in 1..=num {
        let mut updated_matrix_ranges: Vec<MatrixRange> = Vec::new();
        for current_matrix_range in &current_matrix_ranges {
            let (first_split, second_split) = if index % 2 == 1 {
                current_matrix_range.half_horizontal()
            } else {
                current_matrix_range.half_vertical()
            };
            updated_matrix_ranges.push(first_split);
            updated_matrix_ranges.push(second_split);
        }
        current_matrix_ranges = updated_matrix_ranges;
    }
    current_matrix_ranges
}

fn get_updated_matrix_in_range(matrix: &Matrix<Acre>, matrix_range: &MatrixRange) -> Matrix<Acre> {
    let mut updated_matrix: Matrix<Acre> = Matrix::new(matrix_range.rows(), matrix_range.cols(), Acre::Open);
    for row in matrix_range.row_range.clone() {
        for col in matrix_range.col_range.clone() {
            updated_matrix.set(
                row - matrix_range.first_row(),
                col - matrix_range.first_col(),
                matrix.get(row, col)
            );
        }
    }
    for row in matrix_range.row_range.clone() {
        for col in matrix_range.col_range.clone() {
            let (_, num_tree_acres, num_lumberyard_acres) = get_adjacent_acres_info(
                matrix, 
                row, 
                col
            );
            match matrix.get(row, col) {
                Acre::Open => {
                    if num_tree_acres >= 3 {
                        updated_matrix.set(row - matrix_range.first_row(), col - matrix_range.first_col(), Acre::Tree);
                    }
                },
                Acre::Tree => {
                    if num_lumberyard_acres >= 3 {
                        updated_matrix.set(row - matrix_range.first_row(), col - matrix_range.first_col(), Acre::Lumberyard);
                    }
                },
                Acre::Lumberyard => {
                    if !(num_lumberyard_acres >= 1 && num_tree_acres >= 1) {
                        updated_matrix.set(row - matrix_range.first_row(), col - matrix_range.first_col(), Acre::Open);
                    }
                },
            }
        }
    }
    updated_matrix
}

fn replace_matrix(matrix: &mut Matrix<Acre>, matrix_range: &MatrixRange, partial_matrix: &Matrix<Acre>) {
    for row in matrix_range.row_range.clone() {
        for col in matrix_range.col_range.clone() {
            let value = partial_matrix.get(row - matrix_range.first_row(), col - matrix_range.first_col());
            matrix.set(row, col, value);
        }
    }
}

fn get_updated_matrix_with_threads(matrix: &Matrix<Acre>, num_threads: usize) -> Matrix<Acre> {
    let mut updated_matrix = matrix.clone();
    let num = ((num_threads as f64).ln() / (2.0f64).ln()).floor() as usize;
    let mut matrix_ranges = get_matrix_ranges(&matrix.get_range(), num);
    let (aux_tx, rx): (Sender<(MatrixRange, Matrix<Acre>)>, Receiver<(MatrixRange, Matrix<Acre>)>) = 
        channel();
    let mut threads_handle = Vec::new();
    for _ in 0..num_threads {
        let tx = aux_tx.clone();
        let original_matrix = matrix.clone();
        let matrix_range = matrix_ranges.remove(0);
        let join_handle = thread::spawn(move || {
            let updated_matrix = get_updated_matrix_in_range(&original_matrix, &matrix_range);
            tx.send((matrix_range, updated_matrix)).unwrap();
        });
        threads_handle.push(join_handle);
    }
    for _ in 0..num_threads {
        let (matrix_range, partial_matrix) = rx.recv().unwrap();
        replace_matrix(&mut updated_matrix, &matrix_range, &partial_matrix);
    }
    for handle in threads_handle {
        handle.join().unwrap();
    }
    updated_matrix
}

fn get_acres_matrix_after_iterations_with_threads(matrix: &Matrix<Acre>, num_iterations: usize, num_threads: usize) -> Matrix<Acre> {
    let mut current_matrix = matrix.clone();
    for _ in 0..num_iterations {
        current_matrix = get_updated_matrix_with_threads(&current_matrix, num_threads);
    }
    current_matrix
}

pub fn solve_part_one(num_iterations: usize) {
    let char_matrix = read_matrix("day_eightteen.txt");
    let matrix = get_acres_matrix(&char_matrix);
    let final_matrix = get_acres_matrix_after_iterations(&matrix, num_iterations);
    let num_tree_acres = final_matrix.count(&Acre::Tree);
    let num_lumberyard_acres = final_matrix.count(&Acre::Lumberyard);
    let answer = num_tree_acres * num_lumberyard_acres;
    println!("{}", answer);
}

pub fn solve_part_one_with_channels(num_iterations: usize, num_threads: usize) {
    let char_matrix = read_matrix("day_eightteen.txt");
    let matrix = get_acres_matrix(&char_matrix);
    let final_matrix = get_acres_matrix_after_iterations_with_threads(&matrix, num_iterations, num_threads);
    let num_tree_acres = final_matrix.count(&Acre::Tree);
    let num_lumberyard_acres = final_matrix.count(&Acre::Lumberyard);
    let answer = num_tree_acres * num_lumberyard_acres;
    println!("{}", answer);
}

pub fn solve_part_two(num_iterations: usize) {
    let char_matrix = read_matrix("day_eightteen.txt");
    let matrix = get_acres_matrix(&char_matrix);
    let final_matrix = get_acres_matrix_after_large_iterations(&matrix, num_iterations);
    let num_tree_acres = final_matrix.count(&Acre::Tree);
    let num_lumberyard_acres = final_matrix.count(&Acre::Lumberyard);
    let answer = num_tree_acres * num_lumberyard_acres;
    println!("{}", answer);
}