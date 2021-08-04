use std::{collections::{HashMap, HashSet}, i64, rc::Rc};

use crate::{day_three::Matrix, utils::read_lines};



#[derive(PartialEq, Eq, Clone, Copy)]
enum Token {
    North,
    East,
    South,
    West,
    LeftParan,
    RightParan,
    Pipe
}

impl Token {
    fn to_direction(&self) -> Option<Direction> {
        match self {
            &Token::North => Some(Direction::North),
            &Token::East => Some(Direction::East),
            &Token::South => Some(Direction::South),
            &Token::West => Some(Direction::West),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South, 
    West
}
#[derive(Clone, Debug)]
pub struct Expression {
    path: Vec<Direction>,
    choices: Vec<Expression>,
    next: Option<Box<Expression>>
}

impl Expression {
    fn from_directions(directions: &Vec<Direction>) -> Expression {
        Expression {
            path: directions.clone(),
            choices: vec![],
            next: None
        }
    }
}

fn get_tokens(string: &str) -> Vec<Token> {
    string.chars()
        .filter_map(|chr| {
            match chr {
                'N' => Option::<Token>::Some(Token::North),
                'E' => Option::<Token>::Some(Token::East),
                'S' => Option::<Token>::Some(Token::South),
                'W' => Option::<Token>::Some(Token::West),
                '(' => Option::<Token>::Some(Token::LeftParan),
                ')' => Option::<Token>::Some(Token::RightParan),
                '|' => Option::<Token>::Some(Token::Pipe),
                _ => Option::<Token>::None,
            }
        })
        .collect()
}

type ParserResult<'a, Output> = Result<(Output, &'a[Token]), &'a[Token]>;


trait Parser<'a, Output> {
    fn parse(&self, tokens: &'a[Token]) -> ParserResult<'a, Output>;

    fn map<F, NewOutput>(self, f: F) -> BoxedParser<'a, NewOutput> 
        where Self: Sized + 'a,
            Output: 'a,
            NewOutput: 'a,
            F: Fn(Output) -> NewOutput + 'a
    {
        BoxedParser::new(move |input: &'a[Token]| {
            self.parse(input).map(|(output, remaining_inputs)|{
                (f(output), remaining_inputs)
            })
        })
    }
}

impl<'a, F, Output> Parser<'a, Output> for F 
where 
    F: Fn(&'a[Token]) -> ParserResult<'a, Output>
{
    fn parse(&self, tokens: &'a[Token]) -> ParserResult<'a, Output> {
        self(tokens)
    }
}

struct BoxedParser<'a, Output> {
    parser: Box<dyn Parser<'a, Output> + 'a>
}

impl<'a, Output> BoxedParser<'a, Output> {
    fn new<P>(p: P) -> Self
        where P: Parser<'a, Output> + 'a 
    {
        BoxedParser {
            parser: Box::new(p)
        }
    }
}

impl<'a, Output> Parser<'a, Output> for BoxedParser<'a, Output> {
    fn parse(&self, tokens: &'a[Token]) -> ParserResult<'a, Output> {
        self.parser.parse(tokens)
    }
}

fn match_token<'a>(token: Token) -> BoxedParser<'a, ()> {
    BoxedParser::new(move |input: &'a[Token]| {
        input.get(0).and_then(|&tk| {
            if tk == token {
                Some(
                    (
                        (), &input[1..]
                    )
                )
            } else {
                None
            }
        }).ok_or(input)
    })
}

fn match_path<'a>() -> BoxedParser<'a, Vec<Direction>> {
    BoxedParser::new(move |input: &'a[Token]| {
        let mut index: usize = 0;
        let mut path: Vec<Direction> = Vec::new();
        while let Some(direction) = input.get(index).and_then(|&token| token.to_direction()) {
            path.push(direction);
            index += 1;
        }
        if index == 0 {
            ParserResult::<'a, Vec<Direction>>::Err(input)
        } else {
            let remaining_input = &input[index..];
            Result::<(Vec<Direction>, &'a[Token]), &'a[Token]>::Ok(
                (path, remaining_input)
            )
        }
    })
}

fn  one_or_more<'a, Output, P>(p: P) -> BoxedParser<'a, Vec<Output>>
    where P: Parser<'a, Output> + 'a
{
    BoxedParser::new(move |input: &'a[Token]| {
        p.parse(input).and_then(|(output, remaining_input)| {
            let mut result_outputs: Vec<Output> = Vec::new();
            result_outputs.push(output);
            let mut current_input = remaining_input;
            while let Ok((other_output, other_input)) = p.parse(current_input) {
                result_outputs.push(other_output);
                current_input = other_input;
            }
            Result::<(Vec<Output>, &'a[Token]), &'a[Token]>::Ok((result_outputs, current_input))
        })
    })
}

fn left<'a, P1, R1, P2, R2>(p1: P1, p2: P2) -> BoxedParser<'a, R1>
    where P1: Parser<'a, R1> + 'a,
        P2: Parser<'a, R2> + 'a
{
    BoxedParser::new(move |input: &'a[Token]| {
        if let Ok((first_result, remaining_input)) = p1.parse(input) {
            if let Ok((_, second_remaining_input)) = p2.parse(remaining_input) {
                ParserResult::<'a, R1>::Ok((first_result, second_remaining_input))
            } else {
                ParserResult::<'a, R1>::Err(remaining_input)
            }
        } else {
            ParserResult::<'a, R1>::Err(input)
        }
    }) 
}

fn right<'a, P1, R1, P2, R2>(p1: P1, p2: P2) -> BoxedParser<'a, R2>
    where P1: Parser<'a, R1> + 'a,
        P2: Parser<'a, R2> + 'a
{
    BoxedParser::new(move |input: &'a[Token]| {
        if let Ok((_, remaining_input)) = p1.parse(input) {
            if let Ok((second_result, second_remaining_input)) = p2.parse(remaining_input) {
                ParserResult::<'a, R2>::Ok((second_result, second_remaining_input))
            } else {
                ParserResult::<'a, R2>::Err(remaining_input)
            }
        } else {
            ParserResult::<'a, R2>::Err(input)
        }
    }) 
}

fn pair<'a, P1, R1, P2, R2>(p1: P1, p2: P2) -> BoxedParser<'a, (R1, R2)> 
    where P1: Parser<'a, R1> + 'a,
        P2: Parser<'a, R2> + 'a
{
    BoxedParser::new(move |input: &'a[Token]| {
        let (first_output, first_input) = p1.parse(input)?;
        let (second_output, second_input) = p2.parse(first_input)?;
        ParserResult::<'a, (R1, R2)>::Ok(
            ((first_output, second_output), second_input)
        )
    })
}

fn either<'a, R, P>(p1: P, p2: P) -> BoxedParser<'a, R>
    where P: Parser<'a, R> + 'a
{
    BoxedParser::new(move |input: &'a[Token]| {
        if let Ok((output, remaining_input)) = p1.parse(input) {
            ParserResult::<'a, R>::Ok((output, remaining_input))
        } else if let Ok((output, remaining_input)) = p2.parse(input) {
            ParserResult::<'a, R>::Ok((output, remaining_input))
        } else {
            ParserResult::<'a, R>::Err(input)
        }
    })
}

fn none_or_one<'a, R, P>(p: P) -> BoxedParser<'a, Option<R>>
    where P: Parser<'a, R> + 'a
{
    BoxedParser::new(move |input: &'a[Token]| {
        if let Ok((result, next_input)) = p.parse(input) {
            ParserResult::<'a, Option<R>>::Ok((Some(result), next_input))
        } else {
            ParserResult::<'a, Option<R>>::Ok((None, input))
        }
    })
}

fn match_expression_with_or<'a>() -> BoxedParser<'a, Expression> {
    left(match_expression(), match_token(Token::Pipe))
}

fn match_inside_paranthesis<'a>() -> BoxedParser<'a, Vec<Expression>> {
    pair(
        one_or_more(match_expression_with_or()), 
        none_or_one(match_expression())
    ).map(|(vec_expressions, opt_expression)| {
        let mut res_vec_expressions = vec_expressions.clone();
        let end_expression = if let Some(expr) = opt_expression {
            expr
        } else {
            Expression::from_directions(&vec![])
        };
        res_vec_expressions.push(end_expression);
        res_vec_expressions
    })
}

fn match_paranthesis_expression<'a>() -> BoxedParser<'a, Expression> {
    right(
        match_token(Token::LeftParan),
        left(
            match_inside_paranthesis(),
            match_token(Token::RightParan)
        )
    ).map(|choices| {
        let mut expression = Expression::from_directions(&vec![]);
        expression.choices = choices;
        expression
    })
}

fn match_single_path_expression<'a>() -> BoxedParser<'a, Expression> {
    match_path().map(|directions| Expression::from_directions(&directions))
}

fn get_matched_expression<'a>(tokens: &'a[Token]) -> (Option<Expression>, &'a[Token]) {
    if tokens.is_empty() {
        (None, tokens)
    } else {
        let expression_parser = either(match_paranthesis_expression(), match_single_path_expression());
        match expression_parser.parse(tokens) {
            Ok((mut expression, next_tokens)) => {
                let (opt_next_expression, next_next_tokens) = get_matched_expression(next_tokens);
                if let Some(next_expression) = opt_next_expression {
                    expression.next = Some(Box::new(next_expression));
                }
                (Some(expression), next_next_tokens)
            },
            Err(next_tokens) => (None, next_tokens),
        }
    }
}

fn match_expression<'a>() -> BoxedParser<'a, Expression> {
    BoxedParser::new(move |input: &'a[Token]| {
        let (expression_opt, next_input) = get_matched_expression(input);
        if let Some(expression) = expression_opt {
            ParserResult::<'a, Expression>::Ok((expression, next_input))
        } else {
            ParserResult::<'a, Expression>::Err(input)
        }
    })
}

pub fn get_expression(string: &str) -> Expression {
    let tokens = get_tokens(string);
    if let Ok((expression, _)) = match_expression().parse(&tokens) {
        return expression;
    }
    panic!("Error");
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Room,
    DoorVertical,
    DoorHorizontal,
    Wall
}

impl Tile {
    fn get_char(&self) -> char {
        match self {
            &Tile::Room => '.',
            &Tile::Wall => '#',
            &Tile::DoorHorizontal => '-',
            &Tile::DoorVertical => '|',
        }
    }

    fn is_door(&self) -> bool {
        match self {
            &Tile::DoorHorizontal => true,
            &Tile::DoorVertical => true,
            _ => false,
        }
    }

    fn is_room(&self) -> bool {
        match self {
            &Tile::Room => true,
            _ => false,
        }
    }
}

struct Map {
    tiles: HashMap<(i64, i64), Tile>,
    min_row: i64,
    max_row: i64,
    min_col: i64,
    max_col: i64
}


impl Map {
    fn new() -> Map {
        Map {
            tiles: HashMap::new(),
            min_row: i64::MAX,
            max_row: i64::MIN,
            min_col: i64::MAX,
            max_col: i64::MIN,
        }
    }

    fn set(&mut self, row: i64, col: i64, tile: Tile) {
        self.tiles.insert((row, col), tile);
        if row < self.min_row {
            self.min_row = row;
        }
        if row > self.max_row {
            self.max_row = row;
        }
        if col < self.min_col {
            self.min_col = col;
        }
        if col > self.max_col {
            self.max_col = col;
        }
    }

    fn get(&self, row: i64, col: i64) -> Tile {
        self.tiles.get(&(row, col)).map(|&tile| tile).unwrap()
    }

    fn to_matrix(&self) -> Matrix<Tile> {
        let rows = (self.max_row - self.min_row + 1) as usize;
        let cols = (self.max_col - self.min_col + 1) as usize;
        let mut matrix: Matrix<Tile> = Matrix::new(rows, cols, Tile::Wall);
        for row in self.min_row..=self.max_row {
            for col in self.min_col..=self.max_col {
                let matrix_row = (row - self.min_row) as usize;
                let matrix_col = (col - self.min_col) as usize;
                let tile = match self.tiles.get(&(row, col)) {
                    Some(&tile) => tile,
                    None => Tile::Wall,
                };
                matrix.set(matrix_row, matrix_col, tile);
            }
        }
        matrix
    }
}

fn fill_map_by_expression(expression: &Expression, map: &mut Map, row: i64, col: i64) -> (i64, i64) {
    let mut current_row = row;
    let mut current_col = col;
    for &direction in expression.path.iter() {
        match direction {
            Direction::North => {
                map.set(current_row - 1, current_col, Tile::DoorHorizontal);
                map.set(current_row - 2, current_col, Tile::Room);
                map.set(current_row - 1, current_col - 1, Tile::Wall);
                map.set(current_row - 1, current_col + 1, Tile::Wall);
                current_row -= 2;
            },
            Direction::East => {
                map.set(current_row, current_col + 1, Tile::DoorVertical);
                map.set(current_row, current_col +2, Tile::Room);
                map.set(current_row - 1, current_col + 1, Tile::Wall);
                map.set(current_row + 1, current_col + 1, Tile::Wall);
                current_col += 2;
            },
            Direction::South => {
                map.set(current_row + 1, current_col, Tile::DoorHorizontal);
                map.set(current_row + 2, current_col, Tile::Room);
                map.set(current_row + 1, current_col - 1, Tile::Wall);
                map.set(current_row + 1, current_col + 1, Tile::Wall);
                current_row += 2;
            },
            Direction::West => {
                map.set(current_row, current_col - 1, Tile::DoorVertical);
                map.set(current_row, current_col - 2, Tile::Room);
                map.set(current_row - 1, current_col - 1, Tile::Wall);
                map.set(current_row + 1, current_col - 1, Tile::Wall);
                current_col -= 2;
            },
        }
    }
    if !expression.choices.is_empty() {
        let original_row = current_row;
        let original_col = current_col;
        for branch_expression in expression.choices.iter() {
            let (branch_row, branch_col) = fill_map_by_expression(branch_expression, map, original_row, original_col);
            if let Some(next_expression) = &expression.next {
                let (next_row, next_col) = fill_map_by_expression(next_expression.as_ref(), map, branch_row, branch_col);
                current_row = next_row;
                current_col = next_col;
            }
        }
    } else {
        if let Some(next_expression) = &expression.next {
            let (next_row, next_col) = fill_map_by_expression(next_expression.as_ref(), map, current_row, current_col);
            current_row = next_row;
            current_col = next_col;
        }
    }
    (current_row, current_col)
}

fn get_matrix_repr(matrix: &Matrix<Tile>) -> String {
    let mut string: String = String::new();
    for row in 0..matrix.rows {
        for col in 0..matrix.cols {
            string.push(matrix.get(row, col).get_char());
        }
        string.push('\n');
    }
    string
}

fn get_map_after_expression(expression: &Expression) -> (Map, Matrix<Tile>) {
    let mut map = Map::new();
    map.set(0, 0, Tile::Room);
    fill_map_by_expression(expression, &mut map, 0, 0);
    let matrix = map.to_matrix();
    (map, matrix)
}

const OFFSETS: &[(i64, i64); 4] = &[
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1)
];

fn get_furthest_room(map: &Map, matrix: &Matrix<Tile>) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: Vec<((usize, usize), usize)> = Vec::new();
    let mut max_dist: usize = 0;
    let first_row = (-map.min_row) as usize;
    let first_col = (-map.min_col) as usize;
    visited.insert((first_row, first_col));
    queue.push(((first_row, first_col), 0));
    while !queue.is_empty() {
        let ((row, col), dist) = queue.remove(0);
        for &(offset_row, offset_col) in OFFSETS {
            let door_row_offseted = row as i64 + offset_row;
            let door_col_offseted = col as i64 + offset_col;
            let room_row_offseted = row as i64 + offset_row * 2;
            let room_col_offseted = col as i64 + offset_col * 2;
            if door_row_offseted >= 0 && door_row_offseted < matrix.rows as i64 &&
                door_col_offseted >= 0 && door_col_offseted < matrix.cols as i64 && 
                room_row_offseted >= 0 && room_row_offseted < matrix.rows as i64 &&
                room_col_offseted >= 0 && room_col_offseted < matrix.rows as i64 {
                    let door_offseted_row = door_row_offseted as usize;
                    let door_offseted_col = door_col_offseted as usize;
                    let room_offseted_row = room_row_offseted as usize;
                    let room_offseted_col = room_col_offseted as usize;
                    if matrix.get(door_offseted_row, door_offseted_col).is_door() && matrix.get(room_offseted_row, room_offseted_col).is_room() &&
                     !visited.contains(&(room_offseted_row, room_offseted_col)) {
                        visited.insert((room_offseted_row, room_offseted_col));
                        queue.push(((room_offseted_row, room_offseted_col), dist + 1));
                        if dist + 1 > max_dist {
                            max_dist = dist + 1;
                        }
                    }
                }
        }
    }
    max_dist
}

fn get_rooms_with_distance(map: &Map, matrix: &Matrix<Tile>, distance: usize) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: Vec<((usize, usize), usize)> = Vec::new();
    let mut num_rooms: usize = 0;
    let first_row = (-map.min_row) as usize;
    let first_col = (-map.min_col) as usize;
    visited.insert((first_row, first_col));
    queue.push(((first_row, first_col), 0));
    while !queue.is_empty() {
        let ((row, col), dist) = queue.remove(0);
        for &(offset_row, offset_col) in OFFSETS {
            let door_row_offseted = row as i64 + offset_row;
            let door_col_offseted = col as i64 + offset_col;
            let room_row_offseted = row as i64 + offset_row * 2;
            let room_col_offseted = col as i64 + offset_col * 2;
            if door_row_offseted >= 0 && door_row_offseted < matrix.rows as i64 &&
                door_col_offseted >= 0 && door_col_offseted < matrix.cols as i64 && 
                room_row_offseted >= 0 && room_row_offseted < matrix.rows as i64 &&
                room_col_offseted >= 0 && room_col_offseted < matrix.rows as i64 {
                    let door_offseted_row = door_row_offseted as usize;
                    let door_offseted_col = door_col_offseted as usize;
                    let room_offseted_row = room_row_offseted as usize;
                    let room_offseted_col = room_col_offseted as usize;
                    if matrix.get(door_offseted_row, door_offseted_col).is_door() && matrix.get(room_offseted_row, room_offseted_col).is_room() &&
                     !visited.contains(&(room_offseted_row, room_offseted_col)) {
                        visited.insert((room_offseted_row, room_offseted_col));
                        queue.push(((room_offseted_row, room_offseted_col), dist + 1));
                        if dist + 1 >= distance {
                            num_rooms += 1;
                        }
                    }
                }
        }
    }
    num_rooms
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenParser {
    Path {
        directions: Vec<Direction>
    },
    LeftParan,
    RightParan,
    Pipe
}

fn is_direction(string: &str) -> bool {
    string == "N" || string == "W" || string == "E" || string == "S"
}

fn get_direction(string: &str) -> Direction {
    match string {
        "N" => Direction::North,
        "E" => Direction::East,
        "S" => Direction::South,
        _ => Direction::West,
    }
}

pub fn get_parser_tokens(string: &str) -> Vec<TokenParser> {
    let mut current_string = string;
    let mut tokens: Vec<TokenParser> = Vec::new();
    while !current_string.is_empty() {
        let first = &current_string[0..1];
        if first == "(" {
            current_string = &current_string[1..];
            tokens.push(TokenParser::LeftParan);
        } else if first == ")" {
            current_string = &current_string[1..];
            tokens.push(TokenParser::RightParan);
        } else if first == "|" {
            current_string = &current_string[1..];
            tokens.push(TokenParser::Pipe);
        } else {
            let mut directions: Vec<Direction> = Vec::new();
            let mut index: usize = 0;
            while index < current_string.len() && is_direction(&current_string[index..(index + 1)]) {
                let direction = get_direction(&current_string[index..(index + 1)]);
                directions.push(direction);
                index += 1;
            }
            current_string = &current_string[index..];
            tokens.push(TokenParser::Path { directions });
        }
    }
    tokens
}

fn get_expression_from_parser_tokens<'a>(tokens: &'a[TokenParser]) -> (Option<Expression>, &'a[TokenParser]) {
    if tokens.is_empty() {
        (None, tokens)
    } else {
        if let TokenParser::Path { directions } = &tokens[0] {
            let mut expression = Expression::from_directions(directions);
            let mut remaining_tokens = &tokens[1..];
            if let (Some(next_expression), next_tokens) = get_expression_from_parser_tokens(&tokens[1..]) {
                expression.next = Some(Box::new(next_expression));
                remaining_tokens = next_tokens;
            }
            (Some(expression), remaining_tokens)
        } else {
            if tokens[0] == TokenParser::LeftParan {
                let mut expression = Expression {
                    path: vec![],
                    choices: vec![],
                    next: None
                };
                let mut current_tokens = &tokens[1..];
                let mut is_not_done = true;
                while is_not_done {
                    let (opt_expression, next_tokens) = get_expression_from_parser_tokens(current_tokens);
                    let branch_expression = opt_expression.unwrap();
                    current_tokens = next_tokens;
                    expression.choices.push(branch_expression);
                    if current_tokens[0] == TokenParser::RightParan {
                        is_not_done = false;
                    } else {
                        current_tokens = &current_tokens[1..];
                        if current_tokens[0] == TokenParser::RightParan {
                            is_not_done = false;
                            expression.choices.push(Expression::from_directions(&vec![]));
                        }
                    }
                }
                current_tokens = &current_tokens[1..];
                if let (Some(next_expression), next_tokens) = get_expression_from_parser_tokens(current_tokens) {
                    expression.next = Some(Box::new(next_expression));
                    current_tokens = next_tokens;
                }
                (Some(expression), current_tokens)
            } else {
                (None, tokens)
            }
        }   
    }
}

pub fn get_expression_from_string(string: &str) -> Expression {
    let tokens = get_parser_tokens(string);
    let (opt_expression, _) = get_expression_from_parser_tokens(&tokens);
    opt_expression.unwrap()
}

fn get_map_after_path(directions: &[Direction], map: &mut Map, row: i64, col: i64) -> (i64, i64) {
    let mut current_row = row;
    let mut current_col = col;
    for &direction in directions {
        match direction {
            Direction::North => {
                map.set(current_row - 1, current_col, Tile::DoorHorizontal);
                map.set(current_row - 2, current_col, Tile::Room);
                map.set(current_row - 1, current_col - 1, Tile::Wall);
                map.set(current_row - 1, current_col + 1, Tile::Wall);
                current_row -= 2;
            },
            Direction::East => {
                map.set(current_row, current_col + 1, Tile::DoorVertical);
                map.set(current_row, current_col +2, Tile::Room);
                map.set(current_row - 1, current_col + 1, Tile::Wall);
                map.set(current_row + 1, current_col + 1, Tile::Wall);
                current_col += 2;
            },
            Direction::South => {
                map.set(current_row + 1, current_col, Tile::DoorHorizontal);
                map.set(current_row + 2, current_col, Tile::Room);
                map.set(current_row + 1, current_col - 1, Tile::Wall);
                map.set(current_row + 1, current_col + 1, Tile::Wall);
                current_row += 2;
            },
            Direction::West => {
                map.set(current_row, current_col - 1, Tile::DoorVertical);
                map.set(current_row, current_col - 2, Tile::Room);
                map.set(current_row - 1, current_col - 1, Tile::Wall);
                map.set(current_row + 1, current_col - 1, Tile::Wall);
                current_col -= 2;
            },
        }
    }
    (current_row, current_col)
}

fn get_right_paran_index(tokens: &[TokenParser]) -> usize {
    let mut index = 0;
    let mut nested_count: usize = 1;
    while index < tokens.len() && nested_count != 0 {
        if tokens[index] == TokenParser::LeftParan {
            nested_count += 1;
        } else if tokens[index] == TokenParser::RightParan {
            nested_count -= 1;
        }
        if nested_count != 0 {
            index += 1;
        }
    }
    index + 1
}

fn get_separated_pipe_tokens<'a>(tokens: &'a[TokenParser]) -> Vec<&'a[TokenParser]> {
    let mut pipe_indices: Vec<usize> = Vec::new();
    let mut expressions: Vec<&'a[TokenParser]> = Vec::new();
    let mut level: usize = 0;
    for (token_index, token) in tokens.iter().enumerate() {
        match token {
            &TokenParser::LeftParan => {
                level += 1;
            },
            &TokenParser::RightParan => {
                level -= 1;
            },
            &TokenParser::Pipe => {
                if level == 0 {
                    pipe_indices.push(token_index);
                }
            }
            _ => {},
        }
    }
    expressions.push(&tokens[0..pipe_indices[0]]);
    for index in 0..(pipe_indices.len() - 1) {
        let first_pipe_index = pipe_indices[index] + 1;
        let second_pipe_index = pipe_indices[index + 1];
        expressions.push(&tokens[first_pipe_index..second_pipe_index]);
    }
    expressions.push(&tokens[(pipe_indices[pipe_indices.len() - 1] + 1)..]);
    expressions
}

fn get_directions_str(directions: &[Direction]) -> String {
    let mut string = String::new();
    for &direction in directions {
        let chr = match direction {
            Direction::North => 'N',
            Direction::East => 'E',
            Direction::South => 'S',
            Direction::West => 'W',
        };
        string.push(chr);
    }
    string
}

fn get_map_after_token_expression<'a>(tokens: &'a[TokenParser], map: &mut Map, row: i64, col: i64) {
    let mut current_row = row;
    let mut current_col = col;
    let mut coordinates: Vec<(i64, i64)> = Vec::new();
    for token in tokens {
        match token {
            TokenParser::Path { directions } => {
                let (next_row, next_col) = get_map_after_path(directions, map, current_row, current_col);
                current_row = next_row;
                current_col = next_col;
            },
            &TokenParser::LeftParan => {
                coordinates.push((current_row, current_col));
            },
            &TokenParser::RightParan => {
                let (next_row, next_col) = coordinates.pop().unwrap();
                current_row = next_row;
                current_col = next_col;
            },
            &TokenParser::Pipe => {
                let &(next_row, next_col) = coordinates.last().unwrap();
                current_row = next_row;
                current_col = next_col;
            }
        }
    }
}

fn get_map(tokens: &[TokenParser]) -> (Map, Matrix<Tile>) {
    let mut map = Map::new();
    map.set(0, 0, Tile::Room);
    get_map_after_token_expression(tokens, &mut map, 0, 0);
    let matrix = map.to_matrix();
    (map, matrix)
}

pub fn solve_part_one() {
    let strings = read_lines("day_twenty.txt");
    let tokens = get_parser_tokens(&strings[0]);
    let (map, matrix) = get_map(&tokens);
    let answer = get_furthest_room(&map, &matrix);
    println!("{:?}", answer);
}

pub fn solve_part_two() {
    let strings = read_lines("day_twenty.txt");
    let tokens = get_parser_tokens(&strings[0]);
    let (map, matrix) = get_map(&tokens);
    let answer = get_rooms_with_distance(&map, &matrix, 1000);
    println!("{:?}", answer);
}