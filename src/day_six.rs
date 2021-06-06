use std::usize;
use std::collections::HashSet;
use crate::day_three::Matrix;
use crate::utils::read_lines;

use regex::Regex;

struct Point {
    x: i32,
    y: i32
}

struct Coordinate {
    id: i32,
    point: Point
}

impl Point {
    fn from_string(string: &str) -> Option<Point> {
        let regex = Regex::new(r"(\d*), (\d*)").ok()?;
        let captures = regex.captures(string)?;
        let x = captures[1].parse::<i32>().ok()?;
        let y = captures[2].parse::<i32>().ok()?;
        Some( Point{ x, y } )
    }

    fn get_manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn get_coordinates(strings: &Vec<String>) -> Vec<Coordinate> {
    strings.iter()
        .filter_map(|string| Point::from_string(string))
        .enumerate()
        .map(|(index, point)| 
            Coordinate { 
                id: (index + 1) as i32,
                point
             }
        )
        .collect()
}

fn get_board_size(coordinates: &Vec<Coordinate>) -> (usize, usize) {
    let rows = coordinates.iter()
        .map(|coord| &coord.point)
        .map(|point| point.y)
        .max()
        .map(|r| (r + 1) as usize)
        .unwrap();
    let cols = coordinates.iter()
        .map(|coord| &coord.point)
        .map(|point| point.x)
        .max()
        .map(|c| (c + 1) as usize)
        .unwrap();
    (rows, cols)
}

fn get_coordinate_closest_to_point<'a>(point: &Point, coordinates: &'a Vec<Coordinate>) -> Option<&'a Coordinate> {
    let min_distance = coordinates.iter()
        .map(|coordinate| coordinate.point.get_manhattan_distance(point))
        .min()?;
    let coordinates_min_distance: Vec<_> = coordinates.iter()
        .filter(|coordinate| coordinate.point.get_manhattan_distance(point) == min_distance)
        .collect();
    if coordinates_min_distance.len() != 1 {
        None
    } else {
        coordinates_min_distance.first().map(|f| *f)
    }
}

fn fill_matrix(matrix: &mut Matrix<i32>, coordinates: &Vec<Coordinate>) {
    for row in 0..matrix.rows {
        for col in 0..matrix.cols {
            let point = Point { x: col as i32, y: row as i32 };
            if let Some(coordinate) = get_coordinate_closest_to_point(&point, coordinates) {
                matrix.data[row][col] = coordinate.id
            }
        }
    }
}

fn get_filled_matrix(coordinates: &Vec<Coordinate>) -> Matrix<i32> {
    let (rows, cols) = get_board_size(coordinates);
    let mut matrix = Matrix::<i32>::new(rows, cols, 0);
    fill_matrix(&mut matrix, coordinates);
    matrix
}

fn get_interior_coordinates<'a>(matrix: &Matrix<i32>, coordinates: &'a Vec<Coordinate>) -> Vec<&'a Coordinate> {
    let mut coordinates_ids_on_edges: HashSet<i32> = HashSet::new();
    for row in 0..matrix.rows {
        let first_id = matrix.data[row][0];
        let second_id = matrix.data[row][matrix.cols - 1];
        if first_id != 0 && !coordinates_ids_on_edges.contains(&first_id) {
            coordinates_ids_on_edges.insert(first_id);
        }
        if second_id != 0 && !coordinates_ids_on_edges.contains(&second_id) {
            coordinates_ids_on_edges.insert(second_id);
        }
    }
    for col in 1..(matrix.cols - 1) {
        let first_id = matrix.data[0][col];
        let second_id = matrix.data[matrix.rows - 1][col];
        if first_id != 0 && !coordinates_ids_on_edges.contains(&first_id) {
            coordinates_ids_on_edges.insert(first_id);
        }
        if second_id != 0 && !coordinates_ids_on_edges.contains(&second_id) {
            coordinates_ids_on_edges.insert(second_id);
        }
    }
    coordinates.iter()
        .filter(|coordinate| !coordinates_ids_on_edges.contains(&coordinate.id))
        .collect()
}

fn get_num_points_safe_region(matrix: &Matrix<i32>, coordinates: &Vec<Coordinate>, radius: i32) -> usize {
    let mut num_points_safe_region: usize = 0;
    for row in 0..matrix.rows {
        for col in 0..matrix.cols {
            let point = Point { x: col as i32, y: row as i32 };
            let total_distances: i32 =  coordinates.iter()
                .map(|coordinate| coordinate.point.get_manhattan_distance(&point))
                .sum();
            if total_distances < radius {
                num_points_safe_region += 1;
            }
        }
    }
    num_points_safe_region
}

pub fn solve_part_one() {
    let strings = read_lines("day_six.txt");
    let coordinates = get_coordinates(&strings);
    let matrix = get_filled_matrix(&coordinates);
    let interior_coordinates = get_interior_coordinates(&matrix, &coordinates);
    let answer = interior_coordinates.iter()
        .map(|coordinate| matrix.count(&coordinate.id))
        .max()
        .unwrap();
    println!("{}", answer);
}

pub fn solve_part_two() {
    let strings = read_lines("day_six.txt");
    let coordinates = get_coordinates(&strings);
    let matrix = get_filled_matrix(&coordinates);
    let answer = get_num_points_safe_region(&matrix, &coordinates, 10000);
    println!("{}", answer);
}