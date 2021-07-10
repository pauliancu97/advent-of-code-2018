use crate::utils::read_lines;



struct Rule {
    pattern: Vec<bool>,
    result: bool
}

impl Rule {
    fn from_string(string: &str) -> Rule {
        let mut pattern: Vec<bool> = Vec::new();
        for chr in (&string[0..5]).chars() {
            pattern.push(chr == '#');
        }
        let result = string.chars().last().unwrap() == '#';
        Rule { pattern, result }
    }
}

fn get_pot_pattern(string: &str) -> Vec<bool> {
    let mut pattern: Vec<bool> = Vec::new();
    for chr in string.chars() {
        pattern.push(chr == '#');
    }
    pattern
}

fn is_pattern_matched(pattern: &[bool], rule: &Rule) -> bool {
    pattern.iter().zip(rule.pattern.iter())
        .all(|(&first, &second)| first == second)
}


fn get_next_pattern(current_pattern: &Vec<bool>, rules: &Vec<Rule>) -> Vec<bool> {
    let mut next_pattern = current_pattern.clone();
    for index in 0..=(current_pattern.len() - 5) {
        let pattern_slice = &current_pattern[index..(index + 5)];
        let matching_rule = rules.iter()
            .find(|&rule| is_pattern_matched(pattern_slice, rule));
        if let Some(rule) = matching_rule {
            next_pattern[index + 2] = rule.result;
        } else {
            next_pattern[index + 2] = false;
        }
    }
    next_pattern
}

fn get_pattern_after_iterations(pattern: &Vec<bool>, rules: &Vec<Rule>, num_iterations: usize) -> Vec<bool> {
    let mut current_pattern = pattern.clone();
    for index in 0..num_iterations {
        current_pattern.insert(0, false);
        current_pattern.insert(0, false);
        current_pattern.insert(0, false);
        current_pattern.push(false);
        current_pattern.push(false);
        current_pattern.push(false);
        let next_pattern = get_next_pattern(&current_pattern, rules);
        current_pattern = next_pattern;
        let score = get_pattern_score(&current_pattern, index + 1);
        println!("generation={}, score={}", index + 1, score);
    }
    current_pattern
}

fn get_pattern_score(pattern: &[bool], num_iterations: usize) -> i64 {
    pattern.iter()
        .enumerate()
        .filter(|&(_, &is_alive)| is_alive)
        .map(|(index, _)| (index as i64) - 3 * (num_iterations as i64))
        .sum()
}

pub fn solve_part_one(num_iterations: usize) {
    let strings = read_lines("day_twelve.txt");
    let first_pattern = get_pot_pattern(&strings[0]);
    let rules: Vec<_> = strings.iter()
        .skip(1)
        .map(|string| Rule::from_string(string))
        .collect();
    let last_pattern = get_pattern_after_iterations(&first_pattern, &rules, num_iterations);
    let answer: i64 = last_pattern.iter()
        .enumerate()
        .filter(|&(_, &is_alive)| is_alive)
        .map(|(index, _)| (index as i64) - 3 * (num_iterations as i64))
        .sum();
    println!("{}", answer);
}