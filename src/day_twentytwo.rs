use priority_queue::PriorityQueue;

use crate::day_three::Matrix;
use std::{cmp::{Ordering, Reverse}, collections::HashSet};

static ROCKY_ALLOWED_EQUIPMENT_STATE: &[EquipmentState; 2] = &[
    EquipmentState::Torch,
    EquipmentState::ClimbingGear
];
static WET_ALLOWED_EQUIPMENT_STATE: &[EquipmentState; 2] = &[
    EquipmentState::ClimbingGear,
    EquipmentState::Neither
];
static NARROW_ALLOWED_EQUIPMENT_STATE: &[EquipmentState; 2] = &[
    EquipmentState::Torch,
    EquipmentState::Neither
];

const OFFSETS: &[(i64, i64); 4] = &[
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1)
];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Region {
    Rocky,
    Wet, 
    Narrow
}

impl Region {
    fn from_erosion_level(erosion_level: usize) -> Region {
        if erosion_level % 3 == 0 {
            Region::Rocky
        } else if erosion_level % 3 == 1 {
            Region::Wet
        } else {
            Region::Narrow
        }
    }

    fn get_risk(&self) -> usize {
        match self {
            &Region::Rocky => 0,
            &Region::Wet => 1,
            &Region::Narrow => 2,
        }
    }

    fn get_allowed_equipment_state(&self) -> &'static [EquipmentState] {
        match self {
            &Region::Rocky => ROCKY_ALLOWED_EQUIPMENT_STATE,
            &Region::Wet => WET_ALLOWED_EQUIPMENT_STATE,
            &Region::Narrow => NARROW_ALLOWED_EQUIPMENT_STATE,
        }
    }

    fn get_char(&self) -> char {
        match self {
            &Region::Rocky => '.',
            &Region::Wet => '=',
            &Region::Narrow => '|',
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
enum EquipmentState {
    Torch,
    ClimbingGear,
    Neither
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct ClimberState {
    row: usize,
    col: usize,
    equipment_state: EquipmentState
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct PriorityQueueState {
    cost: usize,
    climber_state: ClimberState
}

impl Ord for PriorityQueueState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then(self.climber_state.cmp(&other.climber_state))
    }
}

impl PartialOrd for PriorityQueueState {
    fn partial_cmp(&self, other: &PriorityQueueState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_cave(rows: usize, cols: usize, depth: usize) -> Matrix<Region> {
    let mut erosion_levels: Matrix<usize> = Matrix::new(rows, cols, 0);
    erosion_levels.set(0, 0, depth % 20183);
    for col in 1..cols {
        erosion_levels.set(0, col, ((col * 16807) + depth) % 20183);
    }
    for row in 1..rows {
        erosion_levels.set(row, 0, ((row * 48271) + depth) % 20183);
    }
    for row in 1..rows {
        for col in 1..cols {
            let geo_index = erosion_levels.get(row - 1, col) * erosion_levels.get(row, col - 1);
            erosion_levels.set(row, col, (geo_index + depth) % 20183);
        }
    }
    erosion_levels.map(Region::Rocky, |&erosion_level| {
        Region::from_erosion_level(erosion_level)
    })
}

fn get_erosions_and_cave(rows: usize, cols: usize, depth: usize) -> (Matrix<usize>, Matrix<Region>) {
    let mut erosion_levels: Matrix<usize> = Matrix::new(rows, cols, 0);
    erosion_levels.set(0, 0, depth % 20183);
    for col in 1..cols {
        erosion_levels.set(0, col, ((col * 16807) + depth) % 20183);
    }
    for row in 1..rows {
        erosion_levels.set(row, 0, ((row * 48271) + depth) % 20183);
    }
    for row in 1..rows {
        for col in 1..cols {
            let geo_index = erosion_levels.get(row - 1, col) * erosion_levels.get(row, col - 1);
            erosion_levels.set(row, col, (geo_index + depth) % 20183);
        }
    }
    erosion_levels.set(rows - 1, cols - 1, depth % 20183);
    let region_matrix = erosion_levels.map(Region::Rocky, |&erosion_level| {
        Region::from_erosion_level(erosion_level)
    });
    (erosion_levels, region_matrix)
}

fn get_risk_level(cave: &Matrix<Region>) -> usize {
    let mut risk_level: usize = 0;
    for row in 0..cave.rows {
        for col in 0..cave.cols {
            risk_level += cave.get(row, col).get_risk();
        }
    }
    risk_level - cave.get(cave.rows - 1, cave.cols -1).get_risk()
}

fn expand_region_matrix(region_matrix: &mut Matrix<Region>, erosion_level_matrix: &mut Matrix<usize>, depth: usize) {
    region_matrix.expand(Region::Rocky);
    erosion_level_matrix.expand(0);
    erosion_level_matrix.set(0, erosion_level_matrix.cols - 1, ((erosion_level_matrix.cols - 1) * 16807 + depth) % 20183);
    erosion_level_matrix.set(erosion_level_matrix.rows - 1, 0, ((erosion_level_matrix.rows - 1) * 48271 + depth) % 20183);
    for row in 1..erosion_level_matrix.rows {
        let col = erosion_level_matrix.cols - 1;
        let geo_index = erosion_level_matrix.get(row - 1, col) * erosion_level_matrix.get(row, col - 1);
        erosion_level_matrix.set(row, col, (geo_index + depth) % 20183);
    }
    for col in 1..(erosion_level_matrix.cols - 1) {
        let row = erosion_level_matrix.rows - 1;
        let geo_index = erosion_level_matrix.get(row - 1, col) * erosion_level_matrix.get(row, col - 1);
        erosion_level_matrix.set(row, col, (geo_index + depth) % 20183);
    }
    for col in 0..erosion_level_matrix.cols {
        let row = region_matrix.rows - 1;
        let region = Region::from_erosion_level(erosion_level_matrix.get(row, col));
        region_matrix.set(row, col, region);
    }
    for row in 0..(erosion_level_matrix.rows - 1) {
        let col = region_matrix.cols - 1;
        let region = Region::from_erosion_level(erosion_level_matrix.get(row, col));
        region_matrix.set(row, col, region);
    }
}

fn get_next_states(climber_state: &ClimberState, cost: usize, regions: &mut Matrix<Region>, erosion_levels: &mut Matrix<usize>, depth: usize) -> Vec<PriorityQueueState> {
    let mut next_states: Vec<PriorityQueueState> = Vec::new();
    let current_row = climber_state.row;
    let current_col = climber_state.col;
    let current_region = regions.get(current_row, current_col);
    for &equipment_state in current_region.get_allowed_equipment_state() {
        if equipment_state != climber_state.equipment_state {
            next_states.push(PriorityQueueState {
                cost: cost + 7,
                climber_state: ClimberState {
                    row: climber_state.row,
                    col: climber_state.col,
                    equipment_state
                }
            });
        }
    }
    for &(offset_row, offset_col) in OFFSETS {
        let offseted_row = (current_row as i64) + offset_row;
        let offseted_col = (current_col as i64) + offset_col;
        if offseted_row >= 0 && offseted_col >= 0 {
            let row_offseted = offseted_row as usize;
            let col_offseted = offseted_col as usize;
            if row_offseted >= regions.rows || col_offseted >= regions.cols {
                expand_region_matrix(regions, erosion_levels, depth);
            }
            let next_region = regions.get(row_offseted, col_offseted);
            if next_region.get_allowed_equipment_state().contains(&climber_state.equipment_state) {
                next_states.push(PriorityQueueState {
                    cost: cost + 1,
                    climber_state: ClimberState {
                        row: row_offseted,
                        col: col_offseted,
                        equipment_state: climber_state.equipment_state
                    }
                });
            }
        }
    }
    next_states 
}

fn is_goal(climber_state: &ClimberState, target_row: usize, target_col: usize) -> bool {
    climber_state.equipment_state == EquipmentState::Torch && climber_state.row == target_row && 
        climber_state.col == target_col
}

fn get_min_time(regions: &mut Matrix<Region>, erosion_levels: &mut Matrix<usize>, depth: usize, target_row: usize, target_col: usize) -> usize {
    let mut visited: HashSet<ClimberState> = HashSet::new();
    let mut frontier: PriorityQueue<ClimberState, Reverse<usize>> = PriorityQueue::new();
    let mut result: Option<usize> = None;
    frontier.push(ClimberState {
        row: 0,
        col: 0,
        equipment_state: EquipmentState::Torch
    }, Reverse(0));
    while result.is_none() {
        let (current_state, cost) = frontier.pop().unwrap();
        if is_goal(&current_state, target_row, target_col) {
            result = Some(cost.0);
        } else {
            visited.insert(current_state.clone());
            for next_queue_states in get_next_states(&current_state, cost.0, regions, erosion_levels, depth) {
                if !visited.contains(&next_queue_states.climber_state) {
                    frontier.push_increase(next_queue_states.climber_state.clone(), Reverse(next_queue_states.cost));
                }
            }
        }
    }
    result.unwrap()
}

pub fn solve_part_one(target_row: usize, target_col: usize, depth: usize) {
    let cave = get_cave(target_row + 1, target_col + 1, depth);
    let answer = get_risk_level(&cave);
    println!("{}", answer);
}

fn get_cave_repr(regions: &Matrix<Region>, rows: usize, cols: usize) -> String {
    let mut string = String::new();
    for row in 0..rows {
        for col in 0..cols {
            string.push(regions.get(row, col).get_char());
        }
        string.push('\n');
    }
    string
}

pub fn solve_part_two(target_row: usize, target_col: usize, depth: usize) {
    let (mut erosion_levels, mut regions) = get_erosions_and_cave(target_row + 1, target_col + 1, depth);
    let answer = get_min_time(&mut regions, &mut erosion_levels, depth, target_row, target_col);
    println!("{}", answer);
}