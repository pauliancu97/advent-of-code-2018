use std::fs;

fn are_units_same_type_opposites_polarities(first: char, second: char) -> bool {
    first.to_ascii_lowercase() == second.to_ascii_lowercase() && 
        (first.is_lowercase() ^ second.is_lowercase())
}

fn get_polymer_after_first_reactions(polymer: &str) -> (String, bool) {
    let mut current_polymer = String::from("");
    let mut had_reactions = false;
    let mut index: i32 = 0;
    while index < (polymer.len() as i32) - 1 {
        if are_units_same_type_opposites_polarities(polymer.as_bytes()[index as usize] as char, polymer.as_bytes()[(index + 1) as usize] as char) {
            index += 2;
            had_reactions = true;
        } else {
            current_polymer += &polymer[(index as usize)..(index as usize + 1)];
            index += 1;
        }
    }
    if index == (polymer.len() as i32) - 1 {
        current_polymer += &polymer[(index as usize)..(index as usize + 1)];
    }
    (current_polymer, had_reactions)
}

fn get_polymer_after_all_reactions(polymer: &str) -> String {
    let mut current_polymer = String::from(polymer);
    let mut is_not_finished = true;
    while is_not_finished {
        let (new_polymer, had_reactions) = get_polymer_after_first_reactions(&current_polymer);
        current_polymer = new_polymer;
        if !had_reactions {
            is_not_finished = false;
        }
    }
    current_polymer
}

pub fn solve_part_one() {
    let polymer = fs::read_to_string("day_five.txt").unwrap();
    let new_polymer = get_polymer_after_all_reactions(&polymer);
    println!("{}", new_polymer.len());
}

pub fn solve_part_two() {
    let polymer = fs::read_to_string("day_five.txt").unwrap();
    let answer = ('a'..='z').map(|chr|{
        let reduced_polymer = polymer.clone().replace(chr, "")
            .replace(chr.to_ascii_uppercase(), "");
        let new_polymer = get_polymer_after_all_reactions(&reduced_polymer);
        new_polymer.len()
    }).min().unwrap();
    println!("{}", answer);
}