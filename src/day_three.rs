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

struct Matrix<T> {
    data: Vec<Vec<T>>,
    rows: usize,
    cols: usize
}

impl<T: Clone> Matrix<T> {
    fn new(rows: usize, cols: usize, default: T) -> Matrix<T> {
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

pub fn solve_part_one() {
    let input = read_lines("day_three.txt");
    let rectangles = get_rectangles(&input);
    let (rows, cols) = get_fabric_size(&rectangles);
    let mut matrix = Matrix::new(rows, cols, 0);
    update_matrix(&mut matrix, &rectangles);
    let answer = get_shared_tiles(&matrix);
    println!("{}", answer);
}