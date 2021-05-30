use std::fs;
use std::collections::HashSet;

fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("Error reading file")
}

fn get_calibrated_frequency(input: &String) -> i32 {
    input.split("\n").map( |string| string.parse::<i32>().unwrap()).sum()
}

fn get_first_repeated_frequency(input: &String) -> i32 {
    let calibrations: Vec<i32> = input.split("\n")
        .map(|string| string.parse::<i32>().unwrap())
        .collect();
    let mut found_frequencies: HashSet<i32> = HashSet::new();
    let mut current_frequency = 0;
    let mut current_index = 0;
    let mut is_not_finished = true;
    found_frequencies.insert(0);
    while is_not_finished {
        current_frequency += calibrations[current_index];
        is_not_finished = !found_frequencies.contains(&current_frequency);
        if is_not_finished {
            found_frequencies.insert(current_frequency);
            current_index = if current_index + 1 < calibrations.len() {
                current_index + 1
            } else {
                0
            }
        }
    }
    current_frequency
}

fn solve_part_one() {
    let input = read_input("day_one.txt");
    let answer = get_calibrated_frequency(&input);
    println!("{}", answer);
}

fn solve_part_two() {
    let input = read_input("day_one.txt");
    let answer = get_first_repeated_frequency(&input);
    println!("{}", answer);
}

fn main() {
    solve_part_two();
}
