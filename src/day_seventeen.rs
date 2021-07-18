use std::{collections::HashMap, fs::File, hash::Hash, io::Write, ops::Range, thread::current};

use regex::Regex;
use lazy_static::lazy_static;

use crate::utils::read_lines;

const SAND_CHAR: char = '.';
const CLAY_CHAR: char = '#';
const RUNNING_CHAR: char = '|';
const DRY_CHAR: char = '~';

#[derive(PartialEq, Eq, Clone, Copy)]
enum WaterDirection {
    Left,
    Right
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Square {
    Sand,
    Clay,
    Running,
    Dry
}

impl Square {
    fn is_solid(&self) -> bool {
        *self == Square::Clay || *self == Square::Dry
    }
}

struct VerticalSlice {
    squares: HashMap<(i64, i64), Square>,
    row_min: i64,
    row_max: i64,
    col_min: i64,
    col_max: i64,
    water_row_min: i64,
    water_row_max: i64,
    water_col_min: i64,
    water_col_max: i64
}

impl VerticalSlice {
    fn get(&self, row: i64, col: i64) -> Square {
        self.squares.get(&(row, col)).map(|&square| square).unwrap_or(Square::Sand)
    }

    fn set(&mut self, row: i64, col: i64, square: Square) {
        self.squares.insert((row, col), square);
        if row < self.row_min {
            self.row_min = row;
        }
        if row > self.row_max {
            self.row_max = row;
        }
        if col < self.col_min {
            self.col_min = col;
        }
        if col > self.col_max {
            self.col_max = col;
        }
        if square == Square::Running || square == Square::Dry {
            if row < self.water_row_min {
                self.water_row_min = row;
            }
            if row > self.water_row_max {
                self.water_row_max = row;
            }
            if col < self.water_col_min {
                self.water_col_min = col;
            }
            if col > self.water_col_max {
                self.water_col_max = col;
            }
        }
    }

    fn new() -> VerticalSlice {
        VerticalSlice {
            squares: HashMap::new(),
            row_min: i64::MAX,
            row_max: i64::MIN,
            col_min: i64::MAX,
            col_max: i64::MIN,
            water_row_min: i64::MAX,
            water_row_max: i64::MIN,
            water_col_min: i64::MAX,
            water_col_max: i64::MIN
        }
    }

    fn from_input(clay_regions: &Vec<(Range<i64>, Range<i64>)>) -> VerticalSlice {
        let mut vertical_slice = VerticalSlice::new();
        for (row_range, col_range) in clay_regions.iter() {
            for row in row_range.clone() {
                for col in col_range.clone() {
                    vertical_slice.set(row, col, Square::Clay);
                }
            }
        }
        vertical_slice
    }

    fn print_water(&self) {
        let mut file = File::create("map.txt").unwrap();
        let mut string: String = String::new();
        for row in self.row_min..(self.row_max + 1) {
            for col in self.col_min..(self.col_max + 1) {
                let chr = match self.get(row, col) {
                    Square::Sand => SAND_CHAR,
                    Square::Clay => CLAY_CHAR,
                    Square::Running => RUNNING_CHAR,
                    Square::Dry => DRY_CHAR,
                };
                string.push(chr);
            }
            string.push('\n');
        }
        file.write_all(string.as_bytes()).unwrap();
    }
}

fn get_edge(vertical_slice: &VerticalSlice, direction: WaterDirection, row: i64, col: i64) -> (i64, bool) {
    let mut current_col = col;
    let offset: i64 = match direction {
        WaterDirection::Left => -1,
        WaterDirection::Right => 1
    };
    while !vertical_slice.get(row, current_col + offset).is_solid() && vertical_slice.get(row + 1, current_col).is_solid() {
        current_col += offset;
    }
    let hit_wall = vertical_slice.get(row, current_col + offset).is_solid();
    (current_col, hit_wall)
}

fn get_num_water_squares(vertical_slice: &VerticalSlice) -> usize {
    let mut res: usize = 0;
    for row in vertical_slice.row_min..(vertical_slice.row_max + 1) {
        for col in vertical_slice.col_min..(vertical_slice.col_max + 1) {
            let square = vertical_slice.get(row, col);
            if square == Square::Running || square == Square::Dry {
                res += 1;
            }
        }
    }
    res
}

fn get_num_still_water_squares(vertical_slice: &VerticalSlice) -> usize {
    let mut res: usize = 0;
    for row in vertical_slice.row_min..(vertical_slice.row_max + 1) {
        for col in vertical_slice.col_min..(vertical_slice.col_max + 1) {
            let square = vertical_slice.get(row, col);
            if square == Square::Dry {
                res += 1;
            }
        }
    }
    res
}

fn get_hole_top(vertical_slice: &VerticalSlice, first_col: i64, second_col: i64, row: i64) -> i64 {
    let mut current_row = row - 1;
    let mut is_not_done = true;
    while is_not_done {
        if vertical_slice.get(current_row, first_col - 1).is_solid() && vertical_slice.get(current_row, second_col + 1).is_solid() {
            for col in first_col..(second_col + 1) {
                let square = vertical_slice.get(current_row, col);
                if !(square == Square::Sand || square == Square::Running) {
                    is_not_done = false;
                }
            }
        } else {
            is_not_done = false;
        }
        if is_not_done {
            current_row -= 1;
        }
    }
    current_row + 1
}

fn fill_vertical_slice(vertical_slice: &mut VerticalSlice, spring_col: i64){
    let spring_row = vertical_slice.row_min - 1;
    let mut current_queue: Vec<(i64, i64)> = Vec::new();
    vertical_slice.set(spring_row, spring_col, Square::Running);
    current_queue.push((spring_row, spring_col));
    let original_row_min = vertical_slice.row_min;
    let original_row_max = vertical_slice.row_max;
    while !current_queue.is_empty() {
        let mut updated_queue: Vec<(i64, i64)> = Vec::new();
        for &(row, col) in &current_queue {
            let mut current_row = row;
            let mut current_col = col;
            while current_row + 1 <= vertical_slice.row_max 
                && vertical_slice.get(current_row + 1, current_col) == Square::Sand {
                    current_row += 1;
                    vertical_slice.set(current_row, current_col, Square::Running);
            }
            if current_row + 1 <= vertical_slice.row_max && vertical_slice.get(current_row + 1, current_col).is_solid() {
                let (left_col, hit_left_wall) = get_edge(
                    vertical_slice, 
                    WaterDirection::Left, 
                    current_row, 
                    current_col
                );
                let (right_col, hit_right_wall) = get_edge(
                    vertical_slice,
                    WaterDirection::Right,
                    current_row,
                    current_col
                );
                if hit_left_wall && hit_right_wall {
                    let top_row = get_hole_top(vertical_slice, left_col, right_col, current_row);
                    for dry_row in top_row..(current_row + 1) {
                        for dry_col in left_col..(right_col + 1) {
                            vertical_slice.set(dry_row, dry_col, Square::Dry);
                        }
                    }
                    let cell = (top_row - 1, current_col);
                    if !updated_queue.contains(&cell) {
                        updated_queue.push(cell);
                    }
                } else {
                    for running_col in left_col..(right_col + 1) {
                        vertical_slice.set(current_row, running_col, Square::Running);
                    }
                    if !hit_left_wall {
                        let cell = (current_row, left_col);
                        if !updated_queue.contains(&cell) {
                            updated_queue.push(cell);
                        }
                    }
                    if !hit_right_wall {
                        let cell = (current_row, right_col);
                        if !updated_queue.contains(&cell) {
                            updated_queue.push(cell);
                        }
                    }
                }
            }
        }
        current_queue = updated_queue;
    }
}

fn get_clay_region(string: &str) -> Option<(Range<i64>, Range<i64>)> {
    lazy_static! {
        static ref FIRST_REGEX: Regex = Regex::new(r"^x=(\d+), y=(\d+)\.\.(\d+)$").unwrap();
        static ref SECOND_REGEX: Regex = Regex::new(r"y=(\d+), x=(\d+)\.\.(\d+)$").unwrap();
    }
    let first_regex_capture = FIRST_REGEX.captures(string);
    let second_regex_capture = SECOND_REGEX.captures(string);
    if let Some(capture) = first_regex_capture {
        let x = capture[1].parse::<i64>().ok()?;
        let y_first = capture[2].parse::<i64>().ok()?;
        let y_second = capture[3].parse::<i64>().ok()?;
        let row_range = y_first..(y_second + 1);
        let col_range = x..(x + 1);
        Some((row_range, col_range))
    } else if let Some(capture) = second_regex_capture {
        let y = capture[1].parse::<i64>().ok()?;
        let x_first = capture[2].parse::<i64>().ok()?;
        let x_second = capture[3].parse::<i64>().ok()?;
        let row_range = y..(y + 1);
        let col_range = x_first..(x_second + 1);
        Some((row_range, col_range))
    } else {
        None
    }
}

pub fn solve_part_one(spring_col: i64) {
    let lines = read_lines("day_seventeen.txt");
    let clay_regions: Vec<_> = lines.iter()
        .filter_map(|string| get_clay_region(string))
        .collect();
    let mut vertical_slice = VerticalSlice::from_input(&clay_regions);
    fill_vertical_slice(&mut vertical_slice, spring_col);
    let answer = get_num_water_squares(&vertical_slice) - 1;
    println!("{}", answer);
}

pub fn solve_part_two(spring_col: i64) {
    let lines = read_lines("day_seventeen.txt");
    let clay_regions: Vec<_> = lines.iter()
        .filter_map(|string| get_clay_region(string))
        .collect();
    let mut vertical_slice = VerticalSlice::from_input(&clay_regions);
    fill_vertical_slice(&mut vertical_slice, spring_col);
    let answer = get_num_still_water_squares(&vertical_slice);
    println!("{}", answer);
}