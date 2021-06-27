use std::cell;

use crate::day_three::Matrix;



pub fn get_power_level(x: i64, y: i64, serial_num: i64) -> i64 {
    (((x + 10) * y + serial_num) * (x + 10) % 1000) / 100 - 5
}

fn get_power_level_matrix(serial_num: i64, size: usize) -> Matrix<i64> {
    let mut matrix: Matrix<i64> = Matrix::new(size, size, 0);
    for row in 0..matrix.rows {
        for col in 0..matrix.cols {
            matrix.set(row, col, get_power_level((col + 1) as i64, (row + 1) as i64, serial_num));
        }
    }
    matrix
}

fn get_cell_power_level(matrix: &Matrix<i64>, cell_row: usize, cell_col: usize, cell_size: usize) -> i64 {
    let mut result: i64 = 0;
    for offset_row in 0..cell_size {
        for offset_col in 0..cell_size {
            let row = cell_row + offset_row;
            let col = cell_col + offset_col;
            result += matrix.get(row, col);
        }
    }
    result
}

fn get_cell_coord_max_power_level(matrix: &Matrix<i64>, cell_size: usize) -> (usize, usize) {
    let mut result: (usize, usize) = (0, 0);
    let mut max_power_level = get_cell_power_level(matrix, 0, 0, cell_size);
    for row in 0..=(matrix.rows - cell_size) {
        for col in 0..=(matrix.cols - cell_size) {
            let power_level = get_cell_power_level(matrix, row, col, cell_size);
            if power_level > max_power_level {
                max_power_level = power_level;
                result = (row, col);
            }
        }
    }
    result
}

fn get_cell_coord_and_power_max_power_level(matrix: &Matrix<i64>, cell_size: usize) -> (usize, usize, i64) {
    let mut max_power_level = get_cell_power_level(matrix, 0, 0, cell_size);
    let mut result: (usize, usize, i64) = (0, 0, max_power_level);
    for row in 0..=(matrix.rows - cell_size) {
        for col in 0..=(matrix.cols - cell_size) {
            let power_level = get_cell_power_level(matrix, row, col, cell_size);
            if power_level > max_power_level {
                max_power_level = power_level;
                result = (row, col, max_power_level);
            }
        }
    }
    result
}

fn get_cell_max_power_level(matrix: &Matrix<i64>) -> (usize, usize, usize) {
    let mut sums: Matrix<i64> = Matrix::new(matrix.rows, matrix.cols, 0);
    for row in 0..sums.rows {
        for col in 0..sums.cols {
            let sum_coord = matrix.get(row, col) +
                if col != 0 { sums.get(row, col - 1) } else { 0 } +
                if row != 0 { sums.get(row - 1, col) } else { 0 } -
                if row != 0 && col != 0 { sums.get(row - 1, col - 1) } else { 0 };
            sums.set(row, col, sum_coord);
        }
    }
    let mut result: (usize, usize, usize) = (0, 0, 1);
    let mut max_power_level: i64 = matrix.get(0, 0);
    for cell_size in 1..=matrix.rows {
        for row in 0..=(matrix.rows - cell_size) {
            for col in 0..=(matrix.cols - cell_size) {
                let cell_sum = sums.get(row + cell_size - 1, col + cell_size - 1) -
                    if col != 0 { sums.get(row + cell_size - 1, col - 1) } else { 0 } -
                    if row != 0 { sums.get(row - 1, col + cell_size - 1) } else { 0 } +
                    if row != 0 && col != 0 { sums.get(row - 1, col - 1) } else { 0 };
                if cell_sum > max_power_level {
                    max_power_level = cell_sum;
                    result = (row, col, cell_size);
                }
            }
        }
    }
    result
}

pub fn solve_part_one(matrix_size: usize, cell_size: usize, serial_num: i64) {
    let matrix = get_power_level_matrix(serial_num, matrix_size);
    let (row, col) = get_cell_coord_max_power_level(&matrix, cell_size);
    println!("{},{}", col + 1, row + 1);
}

pub fn solve_part_two(matrix_size: usize, serial_num: i64) {
    let matrix = get_power_level_matrix(serial_num, matrix_size);
    let (row, col, cell_size) = get_cell_max_power_level(&matrix);
    println!("{},{},{}", col + 1, row + 1, cell_size);
}