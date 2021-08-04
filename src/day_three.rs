use std::ops::Range;
use std::usize;
use std::cmp::Eq;

use regex::Regex;
use crate::utils::read_lines;

struct Rectangle {
    id: i32,
    top: i32,
    left: i32,
    width: i32,
    height: i32
}

impl Rectangle {
    fn from_string(input: &str) -> Option<Rectangle> {
        let regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        let capture = regex.captures(input)?;
        let id = capture[1].parse::<i32>().ok()?;
        let left = capture[2].parse::<i32>().ok()?;
        let top = capture[3].parse::<i32>().ok()?;
        let width = capture[4].parse::<i32>().ok()?;
        let height = capture[5].parse::<i32>().ok()?;
        Some(
            Rectangle {
                id,
                top,
                left,
                width,
                height
            }
        )
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct MatrixRange {
    pub row_range: Range<usize>,
    pub col_range: Range<usize>
}

impl MatrixRange {
    pub fn half_horizontal(&self) -> (MatrixRange, MatrixRange) {
        (
            MatrixRange {
                row_range: 0..(self.row_range.len() / 2),
                col_range: self.col_range.clone()
            },
            MatrixRange {
                row_range: (self.row_range.len() / 2)..(self.row_range.len()),
                col_range: self.col_range.clone()
            }
        )
    }

    pub fn half_vertical(&self) -> (MatrixRange, MatrixRange) {
        (
            MatrixRange {
                row_range: self.row_range.clone(),
                col_range: 0..(self.col_range.len() / 2)
            },
            MatrixRange {
                row_range: self.row_range.clone(),
                col_range: (self.col_range.len() / 2)..(self.col_range.len())
            }
        )
    }

    pub fn rows(&self) -> usize {
        self.row_range.len()
    }

    pub fn cols(&self) -> usize {
        self.col_range.len()
    }

    pub fn first_row(&self) -> usize {
        self.row_range.start
    }

    pub fn first_col(&self) -> usize {
        self.col_range.start
    }
}

pub struct Matrix<T> {
    pub data: Vec<Vec<T>>,
    pub rows: usize,
    pub cols: usize
}

impl<T: Eq + Clone> PartialEq for Matrix<T> {
    fn eq(&self, other: &Matrix<T>) -> bool {
        if self.rows == other.rows && self.cols == other.cols {
            let mut is_equal = true;
            for row in 0..self.rows {
                for col in 0..self.cols {
                    if self.get(row, col) != other.get(row, col) {
                        is_equal = false;
                    }
                }
            }
            is_equal
        } else {
            false
        }
    }
}

impl<T: Clone + Eq> Matrix<T> {
    pub fn new(rows: usize, cols: usize, default: T) -> Matrix<T> {
        let mut data: Vec<Vec<T>> = Vec::with_capacity(rows);
        for _ in 0..rows {
            let col: Vec<T> = vec![default.clone(); cols];
            data.push(col);
        }
        Matrix {
            data, 
            rows,
            cols
        }
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        self.data[row][col].clone()
    }

    pub fn get_ref(&self, row: usize, col: usize) -> &T {
        &self.data[row][col]
    }

    pub fn get_mut_ref(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.data[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row][col] = value;
    }

    pub fn count(&self, value: &T) -> usize {
        let mut num: usize = 0;
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.data[row][col] == *value {
                    num += 1;
                }
            }
        }
        num
    }

    pub fn count_predicate<P>(&self, predicate: P) -> usize
        where P: Fn(&T) -> bool
    {
        let mut num: usize = 0;
        for row in 0..self.rows {
            for col in 0..self.cols {
                if predicate(&self.data[row][col]) {
                    num += 1;
                }
            }
        }
        num
    }

    pub fn map<R, F>(&self, default: R, func: F) -> Matrix<R> 
        where R: Clone + Eq,
              F: Fn(&T) -> R
    {
        let mut result: Matrix<R> = Matrix::new(self.rows, self.cols, default.clone());
        for row in 0..self.rows {
            for col in 0..self.cols {
                result.set(row, col, func(&self.data[row][col]));
            }
        }
        result
    }

    pub fn get_range(&self) -> MatrixRange {
        MatrixRange {
            row_range: 0..self.rows,
            col_range: 0..self.cols
        }
    }
}

impl<T: Clone> Clone for Matrix<T> {
    fn clone(&self) -> Matrix<T> {
        let cloned_data = self.data.clone();
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: cloned_data
        }
    }
}

fn get_rectangles(input: &Vec<String>) -> Vec<Rectangle> {
    input.iter().filter_map(|string_ref| Rectangle::from_string(string_ref)).collect()
}

fn get_fabric_size(rectangles: &Vec<Rectangle>) -> (usize, usize) {
    let cols = rectangles.iter()
        .map(|rectangle_ref| rectangle_ref.left + rectangle_ref.width)
        .max().unwrap();
    let rows = rectangles.iter()
        .map(|rectangle_ref| rectangle_ref.top + rectangle_ref.height)
        .max().unwrap();
    (rows as usize, cols as usize)
}


fn update_matrix_with_one_rectangle(matrix: &mut Matrix<i32>, rectangle: &Rectangle) {
    for row in rectangle.top..(rectangle.top + rectangle.height) {
        for col in rectangle.left..(rectangle.left + rectangle.width) {
            matrix.data[row as usize][col as usize] += 1;
        }
    }
}

fn update_matrix(matrix: &mut Matrix<i32>, rectangles: &Vec<Rectangle>) {
    for rect in rectangles {
        update_matrix_with_one_rectangle(matrix, rect);
    }
}

fn get_shared_tiles(matrix: &Matrix<i32>) -> i32 {
    let mut num_shared_tiles = 0;
    for row in 0..matrix.rows {
        for col in 0..matrix.cols {
            if matrix.data[row][col] > 1 {
                num_shared_tiles += 1;
            }
        }
    }
    num_shared_tiles
}

fn is_rectangle_shared(matrix: &Matrix<i32>, rectangle: &Rectangle) -> bool {
    let mut is_shared = false;
    for row in rectangle.top..(rectangle.top + rectangle.height) {
        for col in rectangle.left..(rectangle.left + rectangle.width) {
            if matrix.data[row as usize][col as usize] != 1 {
                is_shared = true;
            }
        }
    }
    is_shared
}

fn get_not_shared_rectangle<'a>(matrix: &Matrix<i32>, rectangles: &'a Vec<Rectangle>) -> &'a Rectangle {
    rectangles.iter().find(|rectangle| !is_rectangle_shared(matrix, rectangle)).unwrap()
}

pub fn solve_part_one() {
    let input = read_lines("day_three.txt");
    let rectangles = get_rectangles(&input);
    let (rows, cols) = get_fabric_size(&rectangles);
    let mut matrix = Matrix::new(rows, cols, 0);
    update_matrix(&mut matrix, &rectangles);
    let answer = get_shared_tiles(&matrix);
    println!("{}", answer);
}

pub fn solve_part_two() {
    let input = read_lines("day_three.txt");
    let rectangles = get_rectangles(&input);
    let (rows, cols) = get_fabric_size(&rectangles);
    let mut matrix = Matrix::new(rows, cols, 0);
    update_matrix(&mut matrix, &rectangles);
    let answer = get_not_shared_rectangle(&matrix, &rectangles);
    println!("{}", answer.id);
}