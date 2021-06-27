use std::{ops::{Add, AddAssign, SubAssign}, usize};

use regex::Regex;

use crate::{day_three::Matrix, utils::read_lines};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector {
    x: i64,
    y: i64
}

impl Vector {
    fn from_string(string: &str) -> Option<Vector> {
        let regex = Regex::new(r"<([ \-]?\d+), ([ \-]?\d+)>").ok()?;
        let captures = regex.captures(string)?;
        let x = captures[1].trim_start().parse::<i64>().ok()?;
        let y = captures[2].trim_start().parse::<i64>().ok()?;
        Some(Vector { x, y })
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

struct Star {
    position: Vector,
    velocity: Vector
}

impl Star {
    fn from_string(string: &str) -> Option<Star> {
        let regex = Regex::new(r"position=(.*) velocity=(.*)").ok()?;
        let captures = regex.captures(string)?;
        let position = Vector::from_string(&captures[1])?;
        let velocity = Vector::from_string(&captures[2])?;
        Some(Star{ position, velocity })
    }

    fn update(&mut self) {
        self.position += self.velocity;
    }

    fn revert(&mut self) {
        self.position -= self.velocity;
    }
}

fn get_stars_area(stars: &Vec<Star>) -> i64 {
    let x_min = stars.iter().map(|star| star.position.x).min().unwrap();
    let x_max = stars.iter().map(|star| star.position.x).max().unwrap();
    let y_min = stars.iter().map(|star| star.position.y).min().unwrap();
    let y_max = stars.iter().map(|star| star.position.y).max().unwrap();
    (x_max - x_min + 1) * (y_max - y_min + 1)
}

fn update_stars_to_message(stars: &mut Vec<Star>) {
    let mut current_area = get_stars_area(stars);
    let mut previous_area = current_area + 1;
    while current_area < previous_area {
        for star in stars.iter_mut() {
            star.update();
        }
        previous_area = current_area;
        current_area = get_stars_area(stars);
    }
    for star in stars.iter_mut() {
        star.revert();
    }
}

fn get_message_seconds(stars: &mut Vec<Star>) -> u64 {
    let mut second: u64 = 0;
    let mut current_area = get_stars_area(stars);
    let mut previous_area = current_area + 1;
    while current_area < previous_area {
        second += 1;
        for star in stars.iter_mut() {
            star.update();
        }
        previous_area = current_area;
        current_area = get_stars_area(stars);
    }
    second - 1
}

fn get_display_matrix(stars: &Vec<Star>) -> Matrix<char> {
    let x_min = stars.iter().map(|star| star.position.x).min().unwrap();
    let x_max = stars.iter().map(|star| star.position.x).max().unwrap();
    let y_min = stars.iter().map(|star| star.position.y).min().unwrap();
    let y_max = stars.iter().map(|star| star.position.y).max().unwrap();
    let rows = (y_max - y_min + 1) as usize;
    let cols = (x_max - x_min + 1) as usize;
    let mut matrix: Matrix<char> = Matrix::new(rows, cols, ' ');
    for star in stars {
        let row = (star.position.y - y_min) as usize;
        let col = (star.position.x - x_min) as usize;
        matrix.set(row, col, '#');
    }
    matrix
}

fn display_matrix(matrix: &Matrix<char>) {
    for row in 0..matrix.rows {
        for col in 0..matrix.cols {
            print!("{}", matrix.get(row, col));
        }
        println!("")
    }
}

fn get_stars(path: &str) -> Vec<Star> {
    read_lines(path).iter().filter_map(|string| Star::from_string(string)).collect()
}

pub fn solve_part_one() {
    let mut stars = get_stars("day_ten.txt");
    update_stars_to_message(&mut stars);
    let display = get_display_matrix(&stars);
    display_matrix(&display);
}

pub fn solve_part_two() {
    let mut stars = get_stars("day_ten.txt");
    let answer = get_message_seconds(&mut stars);
    println!("{}", answer);
}