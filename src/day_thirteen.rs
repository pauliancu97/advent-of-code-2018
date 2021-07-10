use std::{collections::{HashMap, HashSet}, iter::FromIterator};

use crate::{day_three::Matrix, utils::read_matrix};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    row: usize,
    col: usize
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn from_char(chr: char) -> Option<Direction> {
        match chr {
            CART_NORTH_CHAR => Some(Direction::North),
            CART_SOUTH_CHAR => Some(Direction::South),
            CART_EAST_CHAR => Some(Direction::East),
            CART_WEST_CHAR => Some(Direction::West),
            _ => None,
        }
    }

    fn get_straigth(&self) -> Direction {
        match self {
            &Direction::North => Direction::North,
            &Direction::South => Direction::South,
            &Direction::East => Direction::East,
            &Direction::West => Direction::West,
        }
    }

    fn get_right(&self) -> Direction {
        match self {
            &Direction::North => Direction::East,
            &Direction::South => Direction::West,
            &Direction::East => Direction::South,
            &Direction::West => Direction::North,
        }
    }

    fn get_left(&self) -> Direction {
        match self {
            &Direction::North => Direction::West,
            &Direction::South => Direction::East,
            &Direction::East => Direction::North,
            &Direction::West => Direction::South,
        }
    }

    fn get_turn_direction(&self, turn_direction: &TurnDirection) -> Direction {
        match *turn_direction {
            TurnDirection::Left => self.get_left(),
            TurnDirection::Straight => self.get_straigth(),
            TurnDirection::Right => self.get_right(),
        }
    }
}

trait Turn {
    fn get_turned_direction(&self, direction: &Direction) -> Option<Direction>;
}

struct SouthEastOrWestNorth;

impl Turn for SouthEastOrWestNorth {
    fn get_turned_direction(&self, direction: &Direction) -> Option<Direction> {
        if *direction == Direction::South {
            Some(Direction::East)
        } else if *direction == Direction::West {
            Some(Direction::North)
        } else {
            None
        }
    }
}

struct EastSouthOrNorthWest;

impl Turn for EastSouthOrNorthWest {
    fn get_turned_direction(&self, direction: &Direction) -> Option<Direction> {
        if *direction == Direction::East {
            Some(Direction::South)
        } else if *direction == Direction::North {
            Some(Direction::West) 
        } else {
            None
        }
    }
}

struct EastNorthOrSouthWest;

impl Turn for EastNorthOrSouthWest {
    fn get_turned_direction(&self, direction: &Direction) -> Option<Direction> {
        if *direction == Direction::East {
            Some(Direction::North)
        } else if *direction == Direction::South {
            Some(Direction::West)
        } else {
            None
        }
    }
}

struct WestSouthOrNorthEast;

impl Turn for WestSouthOrNorthEast {
    fn get_turned_direction(&self, direction: &Direction) -> Option<Direction> {
        if *direction == Direction::West {
            Some(Direction::South)
        } else if *direction == Direction::North {
            Some(Direction::East)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum TurnDirection {
    Left,
    Straight,
    Right,
}

impl TurnDirection {
    fn get_next_turn_direction(&self) -> TurnDirection {
        match *self {
            TurnDirection::Left => TurnDirection::Straight,
            TurnDirection::Straight => TurnDirection::Right,
            TurnDirection::Right => TurnDirection::Left,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Cart {
    row: usize,
    col: usize,
    direction: Direction,
    turn_direction: TurnDirection,
    id: usize
}

impl Cart {
    fn new(row: usize, col: usize, direction: &Direction, id: usize) -> Cart {
        Cart {
            row, col, direction: direction.clone(), turn_direction: TurnDirection::Left, id
        }
    }

    fn update_position(&mut self) {
        match self.direction {
            Direction::North => self.row -= 1,
            Direction::South => self.row += 1,
            Direction::East => self.col += 1,
            Direction::West => self.col -= 1,
        }
    }

    fn update_on_turn(&mut self, turns: &Vec<Box<dyn Turn>>) {
        self.direction = turns.iter()
            .filter_map(|turn| turn.get_turned_direction(&self.direction))
            .nth(0)
            .unwrap();
    }

    fn update_on_turn_type(&mut self, track_element: &TrackElement) {
        if *track_element == TrackElement::FirstTurn {
            let updated_direction = match self.direction {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            };
            self.direction = updated_direction;
        } else if *track_element == TrackElement::SecondTurn {
            let updated_direction = match self.direction {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::North,
                Direction::West => Direction::South,
            };
            self.direction = updated_direction;
        }
    }

    fn update_on_intersection(&mut self) {
        self.direction = self.direction.get_turn_direction(&self.turn_direction);
        self.turn_direction = self.turn_direction.get_next_turn_direction();
    }

    fn get_position(&self) -> Position {
        Position{
            row: self.row,
            col: self.col
        }
    }
}
#[derive(PartialEq, Eq, Clone, Copy)]
enum TrackElement {
    Nothing,
    Straight,
    FirstTurn,
    SecondTurn, 
    Intersection,
}

struct Tracks {
    matrix: Matrix<TrackElement>,
    turns: HashMap<(usize, usize), Vec<Box<dyn Turn>>>
}

struct State {
    tracks: Tracks,
    carts: Vec<Cart>
}

const FIRST_STRAIGHT_CHAR: char = '-';
const SECOND_STRAIGHT_CHAR: char = '|';
const FIRST_TURN_CHAR: char = '\\';
const SECOND_TURN_CHAR: char = '/';
const INTERSECTION_CHAR: char = '+';
const NOTHING_CHAR: char = ' ';
const CART_NORTH_CHAR: char = '^';
const CART_SOUTH_CHAR: char = 'v';
const CART_EAST_CHAR: char = '>';
const CART_WEST_CHAR: char = '<';

impl Tracks {
    fn new(char_matrix: &Matrix<char>) -> Tracks {
        let mut matrix: Matrix<TrackElement> = Matrix::new(char_matrix.rows, char_matrix.cols, TrackElement::Nothing);
        let mut turns: HashMap<(usize, usize), Vec<Box<dyn Turn>>> = HashMap::new();
        for row in 0..char_matrix.rows {
            for col in 0..char_matrix.cols {
                let chr = char_matrix.get(row, col);
                match chr {
                    FIRST_STRAIGHT_CHAR | SECOND_STRAIGHT_CHAR | CART_EAST_CHAR | CART_NORTH_CHAR | CART_SOUTH_CHAR | CART_WEST_CHAR => {
                        matrix.set(row, col, TrackElement::Straight);
                    },
                    INTERSECTION_CHAR => {
                        matrix.set(row, col, TrackElement::Intersection);
                    },
                    FIRST_TURN_CHAR => {
                        let mut posible_turns: Vec<Box<dyn Turn>> = Vec::new();
                        matrix.set(row, col, TrackElement::FirstTurn);
                        if row != 0 && col < char_matrix.cols - 1 && char_matrix.get(row - 1, col) != NOTHING_CHAR && char_matrix.get(row, col + 1) != NOTHING_CHAR {
                            posible_turns.push(Box::new(SouthEastOrWestNorth));
                        }
                        if row < char_matrix.rows - 1 && col != 0 && char_matrix.get(row, col - 1) != NOTHING_CHAR && char_matrix.get(row + 1, col) != NOTHING_CHAR {
                            posible_turns.push(Box::new(EastSouthOrNorthWest));
                        }
                        turns.insert((row, col), posible_turns);
                    }
                    SECOND_TURN_CHAR => {
                        let mut posible_turns: Vec<Box<dyn Turn>> = Vec::new();
                        matrix.set(row, col, TrackElement::SecondTurn);
                        if row != 0 && col != 0 && char_matrix.get(row, col - 1) != NOTHING_CHAR && char_matrix.get(row - 1, col) != NOTHING_CHAR{
                            posible_turns.push(Box::new(EastNorthOrSouthWest));
                        }
                        if row < char_matrix.rows - 1 && col < char_matrix.cols - 1 && char_matrix.get(row + 1, col) != NOTHING_CHAR && char_matrix.get(row, col + 1) != NOTHING_CHAR {
                            posible_turns.push(Box::new(WestSouthOrNorthEast));
                        }
                        turns.insert((row, col), posible_turns);
                    },
                    _ => {},
                }
            }
        }
        Tracks {
            matrix, turns
        }
    }
}

impl State {
    fn new(char_matrix: &Matrix<char>) -> State {
        let tracks = Tracks::new(char_matrix);
        let mut carts: Vec<Cart> = Vec::new();
        for row in 0..char_matrix.rows {
            for col in 0..char_matrix.cols {
                let chr = char_matrix.get(row, col);
                if let Some(direction) = Direction::from_char(chr) {
                    let cart = Cart::new(row, col, &direction, carts.len());
                    carts.push(cart);
                }
            }
        }
        State {
            tracks, 
            carts
        }
    }

    fn update(&mut self) {
        for cart in &mut self.carts {
            cart.update_position();
            if self.tracks.matrix.get(cart.row, cart.col) == TrackElement::FirstTurn || self.tracks.matrix.get(cart.row, cart.col) == TrackElement::SecondTurn {
                cart.update_on_turn_type(&self.tracks.matrix.get(cart.row, cart.col));
            } else if self.tracks.matrix.get(cart.row, cart.col) == TrackElement::Intersection {
                cart.update_on_intersection();
            }
        }
    }

    fn update_correct(&mut self) {
        self.carts.sort_by(|first, second| first.get_position().cmp(&second.get_position()));
        let mut carts_to_be_deleted_ids: HashSet<usize> = HashSet::new();
        for cart_index in 0..self.carts.len() {
            {
                let cart = &mut self.carts[cart_index];
                if !carts_to_be_deleted_ids.contains(&cart.id) {
                    cart.update_position();
                    if self.tracks.matrix.get(cart.row, cart.col) == TrackElement::FirstTurn || self.tracks.matrix.get(cart.row, cart.col) == TrackElement::SecondTurn {
                        cart.update_on_turn_type(&self.tracks.matrix.get(cart.row, cart.col));
                    } else if self.tracks.matrix.get(cart.row, cart.col) == TrackElement::Intersection {
                        cart.update_on_intersection();
                    }
                }
            }
            for other_cart_index in 0..self.carts.len() {
                let cart = &self.carts[cart_index];
                let other_cart = &self.carts[other_cart_index];
                if cart.id != other_cart.id && cart.get_position() == other_cart.get_position() {
                    carts_to_be_deleted_ids.insert(cart.id);
                    carts_to_be_deleted_ids.insert(other_cart.id);
                }
            }
        }
        self.carts = self.carts.iter()
            .filter(|cart| !carts_to_be_deleted_ids.contains(&cart.id))
            .map(|cart| cart.clone())
            .collect();
    }

    fn get_crash_position(&self) -> Option<(usize, usize)> {
        let mut result: Option<(usize, usize)> = None;
        for first_index in 0..(self.carts.len() - 1) {
            for second_index in (first_index + 1)..self.carts.len() {
                let first_cart = &self.carts[first_index];
                let second_cart = &self.carts[second_index];
                if let None = result {
                    if first_cart.row == second_cart.row && first_cart.col == second_cart.col {
                        result = Some((first_cart.row, first_cart.col));
                    }
                }
            }
        }
        result
    }

    fn remove_crashing_carts(&mut self) {
        let mut crashing_carts_indicies: Vec<usize> = Vec::new();
        for first_index in 0..(self.carts.len() - 1) {
            for second_index in (first_index + 1)..self.carts.len() {
                let first_cart = &self.carts[first_index];
                let second_cart = &self.carts[second_index];
                if first_cart.row == second_cart.row && first_cart.col == second_cart.col {
                    crashing_carts_indicies.push(first_index);
                    crashing_carts_indicies.push(second_index);
                }
            }
        }
        let updated_carts: Vec<_> = self.carts.iter()
            .enumerate()
            .filter(|&(index, _)| !crashing_carts_indicies.contains(&index))
            .map(|(_, cart)| cart.clone())
            .collect();
        if self.carts != updated_carts {
            let difference: Vec<_> = self.carts.iter()
                .filter(|&cart| !updated_carts.contains(cart))
                .collect();
            println!("{:?}", difference);
        }
        self.carts = updated_carts;
    } 

    fn get_first_crash_position(&mut self) -> (usize, usize) {
        let mut result: (usize, usize) = (0, 0);
        let mut is_not_done = true;
        while is_not_done {
            let crash_position = self.get_crash_position();
            if let Some(position) = crash_position {
                result = position;
                is_not_done = false;
            } else {
                self.update();
            }
        }
        result
    }

    fn get_last_remaining_cart_position(&mut self) -> (usize, usize) {
        while self.carts.len() != 1 {
            self.update_correct()
        }
        let last_cart = self.carts.first().unwrap();
        (last_cart.row, last_cart.col)
    }
}

pub fn solve_first_part() {
    let char_matrix = read_matrix("day_thirteen.txt");
    let mut state = State::new(&char_matrix);
    let (row, col) = state.get_first_crash_position();
    println!("{},{}", col, row);
}

pub fn solve_second_part() {
    let char_matrix = read_matrix("day_thirteen.txt");
    let mut state = State::new(&char_matrix);
    let (row, col) = state.get_last_remaining_cart_position();
    println!("{},{}", col, row);
}
