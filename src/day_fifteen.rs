use core::num;
use std::collections::{HashSet, VecDeque};

use crate::{day_three::Matrix, utils::read_matrix};

const WALL_CELL_CHAR: char = '#';
const EMPTY_CELL_CHAR: char = '.';
const ELF_CELL_CHAR: char = 'E';
const GOBLIN_CELL_CHAR: char = 'G';
const OFFSETS: &[(isize, isize); 4] = &[(-1, 0), (0, -1), (0, 1), (1, 0)];

#[derive(PartialEq, Eq, Clone, Copy)]
enum UnitType {
    Elf,
    Goblin
}

impl UnitType {
    fn is_enemy(&self, other: &UnitType) -> bool {
        *self != *other
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Unit {
    id: usize,
    unit_type: UnitType,
    hit_points: usize,
    attack_points: usize,
    row: usize,
    col: usize
}

impl Unit {
    fn new(id: usize, unit_type: UnitType, row: usize, col: usize) -> Unit {
        Unit {
            id,
            unit_type,
            hit_points: 200,
            attack_points: 3,
            row, 
            col
        }
    }

    fn attack(&self, other: &mut Unit) {
        if other.hit_points >= self.attack_points {
            other.hit_points -= self.attack_points;
        } else {
            other.hit_points = 0;
        }
    }

    fn is_dead(&self) -> bool {
        self.hit_points == 0
    }

    fn is_enemy(&self, other: &Unit) -> bool {
        self.unit_type != other.unit_type
    }
}

#[derive(PartialEq, Eq, Clone)]
enum CaveCell {
    EmptyCell,
    WallCell,
    UnitCell { unit: Unit }
}

fn get_cave(char_matrix: &Matrix<char>) -> Matrix<CaveCell> {
    let mut cave: Matrix<CaveCell> = Matrix::new(char_matrix.rows, char_matrix.cols, CaveCell::EmptyCell);
    let mut num_units: usize = 0;
    for row in 0..char_matrix.rows {
        for col in 0..char_matrix.cols {
            let chr = char_matrix.get(row, col);
            let cave_cell = match chr {
                WALL_CELL_CHAR => CaveCell::WallCell,
                EMPTY_CELL_CHAR => CaveCell::EmptyCell,
                ELF_CELL_CHAR => {
                    num_units += 1;
                    CaveCell::UnitCell { unit: Unit::new(num_units, UnitType::Elf, row, col) }
                },
                GOBLIN_CELL_CHAR => {
                    num_units += 1;
                    CaveCell::UnitCell { unit: Unit::new(num_units, UnitType::Goblin, row, col) }
                },
                _ => CaveCell::EmptyCell,
            };
            cave.set(row, col, cave_cell);
        }
    }
    cave
}

fn get_attack(unit: &Unit, cave: &mut Matrix<CaveCell>) -> Option<Unit> {
    let rows = cave.rows as isize;
    let cols = cave.cols as isize;
    let mut min_health: usize = 210;
    let mut result: Option<Unit> = None;
    for &(row_offset, col_offset) in OFFSETS {
        let offseted_row = (unit.row as isize) + row_offset;
        let offseted_col = (unit.col as isize) + col_offset;
        if offseted_row >= 0 && offseted_row < rows &&
            offseted_col >= 0 && offseted_col < cols {
                let cave_cell = cave.get(offseted_row as usize, offseted_col as usize);
                if let CaveCell::UnitCell { unit: posible_enemy_unit } = cave_cell {
                    if posible_enemy_unit.is_enemy(unit) {
                        if posible_enemy_unit.hit_points < min_health {
                            min_health = posible_enemy_unit.hit_points;
                            result = Some(posible_enemy_unit);
                        }
                    }
                }
            }
    }
    result
}

fn get_bfs_data(unit: &Unit, cave: &Matrix<CaveCell>) -> (Matrix<isize>, Matrix<(usize, usize)>) {
    let mut visited: Matrix<bool> = Matrix::new(cave.rows, cave.cols, false);
    let mut distances: Matrix<isize> = Matrix::new(cave.rows, cave.cols, -1);
    let mut parents: Matrix<(usize, usize)> = Matrix::new(cave.rows, cave.cols, (0, 0));
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    visited.set(unit.row, unit.col, true);
    distances.set(unit.row, unit.col, 0);
    queue.push_back((unit.row, unit.col));
    while !queue.is_empty() {
        let (row, col) = queue.pop_front().unwrap();
        for &(row_offset, col_offset) in OFFSETS {
            let offseted_row = (row as isize) + row_offset;
            let offseted_col = (col as isize) + col_offset;
            if offseted_row >= 0 && offseted_row < (cave.rows as isize) &&
                offseted_col >= 0 && offseted_col < (cave.cols as isize) {
                    let cell = cave.get(offseted_row as usize, offseted_col as usize);
                    if cell == CaveCell::EmptyCell && !visited.get(offseted_row as usize, offseted_col as usize) {
                        visited.set(offseted_row as usize, offseted_col as usize, true);
                        distances.set(offseted_row as usize, offseted_col as usize, distances.get(row, col) + 1);
                        parents.set(offseted_row as usize, offseted_col as usize, (row, col));
                        queue.push_back((offseted_row as usize, offseted_col as usize));
                    }
                }
        }
    }
    (distances, parents)
}

fn is_enemy_type_adjacent(row: usize, col: usize, unit_type: &UnitType, cave: &Matrix<CaveCell>) -> bool {
    let mut result = false;
    for &(row_offset, col_offset) in OFFSETS {
        let offseted_row = (row as isize) + row_offset;
        let offseted_col = (col as isize) + col_offset;
        if offseted_row >= 0 && offseted_row < (cave.rows as isize) &&
            offseted_col >= 0 && offseted_col < (cave.cols as isize) {
                if let CaveCell::UnitCell { unit } = cave.get(offseted_row as usize, offseted_col as usize) {
                    if unit.unit_type.is_enemy(&unit_type) {
                        result = true;
                    }
                }
            }
    }
    result
}  

fn get_destination(unit_type: &UnitType, distances: &Matrix<isize>, cave: &Matrix<CaveCell>) -> Option<(usize, usize)> {
    let mut min_distance: isize = -1;
    let mut result: Option<(usize, usize)> = None;
    for row in 0..cave.rows {
        for col in 0..cave.cols {
            if cave.get(row, col) == CaveCell::EmptyCell {
                if is_enemy_type_adjacent(row, col, unit_type, cave) {
                    let dist = distances.get(row, col);
                    if dist != -1 && (dist < min_distance || min_distance == -1) {
                        min_distance = dist;
                        result = Some((row, col));
                    }
                }
            }
        }
    }
    result
}

fn get_first_step(dest_row: usize, dest_col: usize, src_row: usize, src_col: usize, parents: &Matrix<(usize, usize)>) -> (usize, usize) {
    let mut row = dest_row;
    let mut col = dest_col;
    while parents.get(row, col) != (src_row, src_col) {
        let (updated_row, updated_col) = parents.get(row, col);
        row = updated_row;
        col = updated_col;
    }
    (row, col)
}

fn  update_for_unit(unit: &mut Unit, cave: &mut Matrix<CaveCell>) -> bool {
    let mut found_target = false;
    if let Some(mut enemy_unit) = get_attack(unit, cave) {
        found_target = true;
        unit.attack(&mut enemy_unit);
        cave.set(enemy_unit.row, enemy_unit.col, CaveCell::UnitCell { unit : enemy_unit.clone() });
        if enemy_unit.is_dead() {
            cave.set(enemy_unit.row, enemy_unit.col, CaveCell::EmptyCell);
        }
    } else {
        let (distances, parents) = get_bfs_data(unit, cave);
        if let Some((dest_row, dest_col)) = get_destination(&unit.unit_type, &distances, cave) {
            found_target = true;
            let (step_row, step_col) = get_first_step(dest_row, dest_col, unit.row, unit.col, &parents);
            cave.set(unit.row, unit.col, CaveCell::EmptyCell);
            unit.row = step_row;
            unit.col = step_col;
            cave.set(unit.row, unit.col, CaveCell::UnitCell{ unit: unit.clone() });
            if let Some(mut enemy_unit) = get_attack(unit, cave) {
                unit.attack(&mut enemy_unit);
                cave.set(enemy_unit.row, enemy_unit.col, CaveCell::UnitCell { unit: enemy_unit.clone() });
                if enemy_unit.is_dead() {
                    cave.set(enemy_unit.row, enemy_unit.col, CaveCell::EmptyCell);
                }
            }
        }
    }
    found_target
}

fn is_only_one_type_of_unit_left(cave: &Matrix<CaveCell>) -> bool {
    let mut first_unit_type: UnitType = UnitType::Elf;
    let mut is_first_unit_type_set = false;
    for row in 0..cave.rows {
        for col in 0..cave.cols {
            if let CaveCell::UnitCell { unit } = cave.get_ref(row, col) {
                if !is_first_unit_type_set {
                    is_first_unit_type_set = true;
                    first_unit_type = unit.unit_type.clone();
                } else {
                    if first_unit_type != unit.unit_type {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn update_cave(cave: &mut Matrix<CaveCell>) -> bool {
    let mut updated_units_ids: HashSet<usize> = HashSet::new();
    let mut is_full_round = true;
    for row in 0..cave.rows {
        for col in 0..cave.cols {
            if let CaveCell::UnitCell { mut unit } = cave.get(row, col) {
                if !updated_units_ids.contains(&unit.id) {
                    updated_units_ids.insert(unit.id);
                    is_full_round = is_full_round && !is_only_one_type_of_unit_left(cave);
                    update_for_unit(&mut unit, cave);
                }
            }
        }
    }
    is_full_round
}

fn update_cave_until_end(cave: &mut Matrix<CaveCell>) -> usize {
    let mut num_turns = 0;
    let mut updated_units_ids: HashSet<usize> = HashSet::new();
    let mut is_done = false;
    while !is_done {
        for row in 0..cave.rows {
            for col in 0..cave.cols {
                if let CaveCell::UnitCell { mut unit } = cave.get(row, col) {
                    if !updated_units_ids.contains(&unit.id) {
                        updated_units_ids.insert(unit.id);
                        update_for_unit(&mut unit, cave);
                    }
                }
            }
        }
        is_done = is_only_one_type_of_unit_left(cave);
        if !is_done {
            updated_units_ids.clear();
            num_turns += 1;
        }
    }
    num_turns
}

fn print_cave(cave: &Matrix<CaveCell>) {
    for row in 0..cave.rows {
        for col in 0..cave.cols {
            let chr = match cave.get(row, col) {
                CaveCell::EmptyCell => EMPTY_CELL_CHAR,
                CaveCell::WallCell => WALL_CELL_CHAR,
                CaveCell::UnitCell { unit } => {
                    match unit.unit_type {
                        UnitType::Elf => ELF_CELL_CHAR,
                        UnitType::Goblin => GOBLIN_CELL_CHAR,
                    }
                },
            };
            print!("{}", chr);
        }
        println!("");
    }
}

fn get_num_unit_type(cave: &Matrix<CaveCell>, unit_type: UnitType) -> usize {
    cave.count_predicate( |cave_cell| {
        if let CaveCell::UnitCell { unit } = cave_cell {
            unit.unit_type == unit_type
        } else {
            false
        }
    })
}

fn get_num_elves(cave: &Matrix<CaveCell>) -> usize {
    get_num_unit_type(cave, UnitType::Elf)
}

fn get_num_goblins(cave: &Matrix<CaveCell>) -> usize {
    get_num_unit_type(cave, UnitType::Goblin)
}

fn update_until_first_elf_dies(cave: &mut Matrix<CaveCell>) -> (bool, usize) {
    let original_num_elves = get_num_elves(cave);
    let mut num_turns: usize = 0;
    while original_num_elves == get_num_elves(cave) && get_num_goblins(cave) != 0 {
        let is_full_round = update_cave(cave);
        if is_full_round {
            num_turns += 1;
        }
    }
    (original_num_elves != get_num_elves(cave), num_turns)
}

fn get_cave_when_elves_win(original_cave: &Matrix<CaveCell>) -> (usize, usize) {
    let mut result_num_turns: usize = 0;
    let mut result_sum_hit_points: usize = 0;
    let mut current_elf_attack_points: usize = 4;
    let mut is_not_done = true;
    while is_not_done {
        let mut current_cave = original_cave.map(CaveCell::EmptyCell, |cave_cell|{
            if let CaveCell::UnitCell { unit } = cave_cell {
                if unit.unit_type == UnitType::Elf {
                    CaveCell::UnitCell {
                        unit: Unit {
                            attack_points: current_elf_attack_points,
                            ..unit.clone()
                        }
                    }
                } else {
                    cave_cell.clone()
                }
            } else {
                cave_cell.clone()
            }
        });
        let (has_elf_died , num_turns) = update_until_first_elf_dies(&mut current_cave);
        if !has_elf_died {
            is_not_done = false;
            result_num_turns = num_turns;
            result_sum_hit_points = get_sum_remaining_units(&current_cave);
        } else {
            current_elf_attack_points += 1;
        }
    }
    (result_num_turns, result_sum_hit_points)
}

fn get_sum_remaining_units(cave: &Matrix<CaveCell>) -> usize {
    let mut sum: usize = 0;
    for row in 0..cave.rows {
        for col in 0..cave.cols {
            if let CaveCell::UnitCell { unit } = cave.get_ref(row, col) {
                sum += unit.hit_points;
            }
        }
    }
    sum
}

pub fn solve_part_one() {
    let char_matrix = read_matrix("day_fifteen.txt");
    let mut cave = get_cave(&char_matrix);
    let num_turns = update_cave_until_end(&mut cave);
    let sum_hit_points = get_sum_remaining_units(&cave);
    println!("num_turns = {}, sum_hit_points= {}", num_turns, sum_hit_points);
    println!("{}", num_turns * sum_hit_points);
}

pub fn solve_part_two() {
    let char_matrix = read_matrix("day_fifteen.txt");
    let cave = get_cave(&char_matrix);
    let (num_turns, sum_hit_points) = get_cave_when_elves_win(&cave);
    println!("num_turns = {}, sum_hit_points= {}", num_turns, sum_hit_points);
    println!("{}", num_turns * sum_hit_points);
}

