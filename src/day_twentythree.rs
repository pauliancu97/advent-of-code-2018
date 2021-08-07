use core::f64;
use std::{cmp::{Ordering, max}, collections::{BinaryHeap, HashMap}, u64};

use lazy_static::lazy_static;
use regex::Regex;

use crate::utils::read_lines;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Vector {
    x: i64,
    y: i64,
    z: i64
}

impl Vector {
    fn get_manhattan_distance(&self, other: &Vector) -> u64 {
        ((self.x - other.x).abs() as u64) +
            ((self.y - other.y).abs() as u64) +
            ((self.z - other.z).abs() as u64)
    }

    fn get_distance_to_orign(&self) -> u64 {
        self.x.abs() as u64 + self.y.abs() as u64 + self.z.abs() as u64
    }
} 

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Nanobot {
    position: Vector,
    radius: u64
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Space {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    z_min: i64,
    z_max: i64
}

impl Space {
    fn new() -> Space {
        Space {
            x_min: i64::MAX,
            x_max: i64::MIN,
            y_min: i64::MAX,
            y_max: i64::MIN,
            z_min: i64::MAX,
            z_max: i64::MIN
        }
    }

    fn update_for_nanobots(&mut self, nanobots: &[Nanobot]) {
        for nanobot in nanobots {
            if self.x_min > nanobot.position.x {
                self.x_min = nanobot.position.x;
            }
            if self.x_max < nanobot.position.x {
                self.x_max = nanobot.position.x;
            }
            if self.y_min > nanobot.position.y {
                self.y_min = nanobot.position.y;
            }
            if self.y_max < nanobot.position.y {
                self.y_max = nanobot.position.y;
            }
            if self.z_min > nanobot.position.z {
                self.z_min = nanobot.position.z;
            }
            if self.z_max < nanobot.position.z {
                self.z_max = nanobot.position.z;
            }
        }
        let x_dimension = self.x_max - self.x_min;
        let y_dimension = self.y_max - self.y_min;
        let z_dimension = self.z_max - self.z_min;
        let max_dimension = max(x_dimension, max(y_dimension, z_dimension));
        let power = (max_dimension as f64).log2();
        let dimension = 2.0f64.powf(power.ceil()) as i64;
        self.x_min = -dimension;
        self.x_max = dimension;
        self.y_min = -dimension;
        self.y_max = dimension;
        self.z_min = -dimension;
        self.z_max = dimension;
    }

    fn get_distance_from_point(&self, position: &Vector) -> u64 {
        let mut distance: u64 = 0;
        if position.x < self.x_min {
            distance += (self.x_min - position.x) as u64;
        }
        if position.x > self.x_max {
            distance += (position.x - self.x_max) as u64;
        }
        if position.y < self.y_min {
            distance += (self.y_min - position.y) as u64;
        }
        if position.y > self.y_max {
            distance += (position.y - self.y_max) as u64;
        }
        if position.z < self.z_min {
            distance += (self.z_min - position.z) as u64;
        }
        if position.z > self.z_max {
            distance += (position.z - self.z_max) as u64;
        }
        distance
    }

    fn get_distance_to_origin(&self) -> u64 {
        self.get_distance_from_point(&Vector { x: 0, y: 0, z: 0})
    }

    fn is_in_range(&self, nanobot: &Nanobot) -> bool {
        let mut distance: u64 = 0;
        if nanobot.position.x < self.x_min {
            distance += (self.x_min - nanobot.position.x) as u64;
        }
        if nanobot.position.x > self.x_max {
            distance += (nanobot.position.x - self.x_max) as u64;
        }
        if nanobot.position.y < self.y_min {
            distance += (self.y_min - nanobot.position.y) as u64;
        }
        if nanobot.position.y > self.y_max {
            distance += (nanobot.position.y - self.y_max) as u64;
        }
        if nanobot.position.z < self.z_min {
            distance += (self.z_min - nanobot.position.z) as u64;
        }
        if nanobot.position.z > self.z_max {
            distance += (nanobot.position.z - self.z_max) as u64;
        }
        distance <= nanobot.radius
    }

    fn get_num_nanobots_in_range(&self, nanobots: &[Nanobot]) -> usize {
        nanobots.iter()
            .filter(|&nanobot| self.is_in_range(nanobot))
            .count()
    }

    fn center(&self) -> Vector {
        let x = (self.x_max + self.x_min) / 2;
        let y = (self.y_max + self.y_min) / 2;
        let z = (self.z_max + self.z_min) / 2;
        Vector { x, y, z }
    }

    fn size(&self) -> u64 {
        let x_dimension = (self.x_max - self.x_min) as u64;
        let y_dimension = (self.y_max - self.y_min) as u64;
        let z_dimension = (self.z_max - self.z_min) as u64;
        max(x_dimension, max(y_dimension, z_dimension))
    }

    fn split(&self) -> Vec<Space> {
        let x_mid = (self.x_max + self.x_min) / 2;
        let y_mid = (self.y_max + self.y_min) / 2;
        let z_mid = (self.z_max + self.z_min) / 2;
        vec![
            Space {
                x_min: self.x_min,
                x_max: x_mid,
                y_min: y_mid,
                y_max: self.y_max,
                z_min: self.z_min,
                z_max: z_mid
            }, 
            Space {
                x_min: x_mid,
                x_max: self.x_max,
                y_min: y_mid,
                y_max: self.y_max,
                z_min: self.z_min,
                z_max: z_mid
            },
            Space {
                x_min: self.x_min,
                x_max: x_mid,
                y_min: self.y_min,
                y_max: y_mid,
                z_min: self.z_min,
                z_max: z_mid
            },
            Space {
                x_min: x_mid,
                x_max: self.x_max,
                y_min: self.y_min,
                y_max: y_mid,
                z_min: self.z_min,
                z_max: z_mid
            }, 
            Space {
                x_min: self.x_min,
                x_max: x_mid,
                y_min: y_mid,
                y_max: self.y_max,
                z_min: z_mid,
                z_max: self.z_max
            }, 
            Space {
                x_min: x_mid,
                x_max: self.x_max,
                y_min: y_mid,
                y_max: self.y_max,
                z_min: z_mid,
                z_max: self.z_max
            },
            Space {
                x_min: self.x_min,
                x_max: x_mid,
                y_min: self.y_min,
                y_max: y_mid,
                z_min: z_mid,
                z_max: self.z_max
            },
            Space {
                x_min: x_mid,
                x_max: self.x_max,
                y_min: self.y_min,
                y_max: y_mid,
                z_min: z_mid,
                z_max: self.z_max
            }
        ]
    }
}

impl Nanobot {
    fn from_string(string: &str) -> Option<Nanobot> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^pos=<(\-?\d+),(\-?\d+),(\-?\d+)>, r=(\d+)$").unwrap();
        }
        let captures = REGEX.captures(string)?;
        let x = captures.get(1).and_then(|regex_match| regex_match.as_str().parse::<i64>().ok())?;
        let y = captures.get(2).and_then(|regex_match| regex_match.as_str().parse::<i64>().ok())?;
        let z = captures.get(3).and_then(|regex_match| regex_match.as_str().parse::<i64>().ok())?;
        let radius = captures.get(4).and_then(|regex_match| regex_match.as_str().parse::<u64>().ok())?;
        Some(
            Nanobot {
                position: Vector { x, y, z},
                radius
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PriorityQueueSpace {
    num_nanobots: usize,
    space: Space
}

impl Ord for PriorityQueueSpace {
    fn cmp(&self, other: &Self) -> Ordering {
        self.num_nanobots.cmp(&other.num_nanobots)
            .then(other.space.get_distance_to_origin().cmp(&self.space.get_distance_to_origin()))
    }
}

impl PartialOrd for PriorityQueueSpace {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_num_of_nanobots_in_range_of(position: &Vector, nanobots: &[Nanobot]) -> usize {
    nanobots.iter()
        .filter_map(|nanobot| {
            let distance = position.get_manhattan_distance(&nanobot.position);
            if distance <= nanobot.radius {
                Some(distance)
            } else {
                None
            }
        })
        .count()
}

fn get_num_nanobots_in_range(nanobots: &Vec<Nanobot>, main_nanobot: &Nanobot) -> usize {
    nanobots.iter()
        .map(|nanobot| nanobot.position.get_manhattan_distance(&main_nanobot.position))
        .filter(|&distance| distance <= main_nanobot.radius)
        .count()
}

fn get_num_nanobots_in_range_of_strongest_nanobot(nanobots: &Vec<Nanobot>) -> Option<usize> {
    let strongest_nanobot = nanobots.iter()
        .max_by(|first, second| first.radius.cmp(&second.radius))?;
    Some(
        get_num_nanobots_in_range(nanobots, strongest_nanobot)
    )
}

fn get_most_populated_coordinate(nanobots: &[Nanobot]) -> Vector {
    let mut result = Vector { x: 0, y: 0, z: 0};
    let mut max_num_nanobots: usize = 0;
    let mut priority_queue: BinaryHeap<PriorityQueueSpace> = BinaryHeap::new();
    let mut original_space = Space::new();
    original_space.update_for_nanobots(nanobots);
    priority_queue.push(PriorityQueueSpace { space: original_space, num_nanobots: nanobots.len() });
    while let Some(priority_queue_space) = priority_queue.pop() {
        if priority_queue_space.space.size() == 1 {
            let space = &priority_queue_space.space;
            for z in space.z_min..=space.z_max {
                for y in space.y_min..=space.y_max {
                    for x in space.x_min..=space.x_max {
                        let vector = Vector { x, y, z};
                        let num_nanobots = get_num_of_nanobots_in_range_of(&vector, nanobots);
                        if num_nanobots > max_num_nanobots {
                            max_num_nanobots = num_nanobots;
                            result = vector;
                        }
                    }
                }
            }
            break;
        } else if priority_queue_space.space.size() > 1 {
            for space in priority_queue_space.space.split() {
                let num_nanobots = space.get_num_nanobots_in_range(nanobots);
                priority_queue.push(PriorityQueueSpace { space, num_nanobots });
            }
        }
    }
    result
}

pub fn solve_part_one() {
    let strings = read_lines("day_twentythree.txt");
    let nanobots: Vec<_> = strings.iter()
        .filter_map(|string| Nanobot::from_string(string))
        .collect();
    let answer = get_num_nanobots_in_range_of_strongest_nanobot(&nanobots).expect("Empty list of nanobots");
    println!("{}", answer);
}

pub fn solve_part_two() {
    let strings = read_lines("day_twentythree.txt");
    let nanobots: Vec<_> = strings.iter()
        .filter_map(|string| Nanobot::from_string(string))
        .collect();
    let coordinate = get_most_populated_coordinate(&nanobots);
    let answer = coordinate.get_distance_to_orign();
    println!("{}", answer);
}