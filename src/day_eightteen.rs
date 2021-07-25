use crate::{day_three::Matrix, utils::read_matrix};
use std::{fmt, thread::current};

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

pub fn solve_part_one(num_iterations: usize) {
    let char_matrix = read_matrix("day_eightteen.txt");
    let matrix = get_acres_matrix(&char_matrix);
    let final_matrix = get_acres_matrix_after_iterations(&matrix, num_iterations);
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