use std::{collections::HashMap, usize};
use crate::utils::read_lines;


fn is_twice_thrice_id(id: &str) -> (bool, bool) {
    let mut chars_frequencies: HashMap<char, i32> = HashMap::new();
    for chr in id.chars() {
        let option_frequency = chars_frequencies.get(&chr);
        match option_frequency {
            Some(&frequency) => chars_frequencies.insert(chr, frequency + 1),
            None => chars_frequencies.insert(chr, 1)
        };
    }
    let is_twice = chars_frequencies.iter().any(|(_, &frequency)| frequency == 2);
    let is_thrice = chars_frequencies.iter().any(|(_, &frequency)| frequency == 3);
    (is_twice, is_thrice)
}

fn get_checksum_twice_thrice(input: &Vec<String>) -> i32 {
    let mut num_of_twices = 0;
    let mut num_of_thrices = 0;
    for id in input {
        let (is_twice, is_thrice) = is_twice_thrice_id(id);
        if is_twice {
            num_of_twices += 1;
        }
        if is_thrice {
            num_of_thrices += 1;
        }
    }
    num_of_twices * num_of_thrices
}

fn get_index_difference(first: &str, second: &str) -> Option<usize> {
    if first.len() != second.len() {
        None
    } else {
        let different_indices: Vec<_> = first.chars().zip(second.chars())
            .enumerate()
            .filter(|&(_, (first_char, second_char))| first_char != second_char)
            .map(|(index, _)| index)
            .collect();
        if different_indices.len() == 1 {
            Some(different_indices[0])
        } else {
            None
        }
    }
}

fn get_answer_string(input: &Vec<String>) -> String {
    let mut answer: Option<String> = None;
    for first_index in 0..(input.len() - 1) {
        for second_index in (first_index + 1)..input.len() {
            if let Some(difference_index) = get_index_difference(&input[first_index], &input[second_index]) {
                if let None = answer {
                    let first_str = input[first_index].as_str();
                    let second_str = input[second_index].as_str();
                    let answer_str = (&first_str[0..difference_index]).to_string() + &second_str[(difference_index + 1)..];
                    answer = Some(answer_str);
                }
            }
        }
    }
    answer.unwrap()
}

pub fn solve_part_one() {
    let input = read_lines("day_two.txt");
    let answer = get_checksum_twice_thrice(&input);
    println!("{}", answer);
}

pub fn solve_part_two() {
    let input = read_lines("day_two.txt");
    let answer = get_answer_string(&input);
    println!("{}", answer);
}