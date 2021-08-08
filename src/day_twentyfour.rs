use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

use crate::utils::read_lines;

const SEMICOLON_SPACE_SEPARATOR: &str = "; ";
const WEAKNESSES_PREFIX: &str = "weak to ";
const IMMUNITIES_PREFIX: &str = "immune to ";
const COLON_SPACE_SEPARATOR: &str = ", ";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GroupType {
    Immune, 
    Infectious
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Group {
    id: usize,
    group_type: GroupType,
    num_units: u64,
    hit_points: u64,
    attack_points: u64,
    attack_type: String,
    initiative_points: u64,
    weaknesses: Vec<String>,
    immunities: Vec<String>
}

fn has_prefix(string: &str, prefix: &str) -> bool {
    string.get(0..prefix.len())
        .map_or(false, |substring| substring == prefix)
}

fn has_weakness_prefix(string: &str) -> bool {
    has_prefix(string, WEAKNESSES_PREFIX)
}

fn has_immunities_prefix(string: &str) -> bool {
    has_prefix(string, IMMUNITIES_PREFIX)
}

fn get_weaknesses_and_immunities(string: &str) -> (Vec<String>, Vec<String>) {
    let mut weaknesses: Vec<String> = Vec::new();
    let mut immunities: Vec<String> = Vec::new();
    for sub_string in string.split("; ") {
        if has_weakness_prefix(sub_string) {
            let weaknesses_string = &sub_string[WEAKNESSES_PREFIX.len()..];
            weaknesses = weaknesses_string.split(COLON_SPACE_SEPARATOR)
                .map(|weakness_string| String::from(weakness_string))
                .collect();
        } else if has_immunities_prefix(sub_string) {
            let immunities_string = &sub_string[IMMUNITIES_PREFIX.len()..];
            immunities = immunities_string.split(COLON_SPACE_SEPARATOR)
                .map(|immunity_string| String::from(immunity_string))
                .collect();
        }
    }
    (weaknesses, immunities)
}

impl Group {
    fn from_string(id: usize, group_type: GroupType, string: &str) -> Option<Group> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^(\d+) units each with (\d+) hit points(?: \((.+)\))? with an attack that does (\d+) ([a-z]+) damage at initiative (\d+)$").unwrap();
        }
        let captures = REGEX.captures(string)?;
        if captures.get(3).is_some() {
            let num_units = captures[1].parse::<u64>().ok()?;
            let hit_points = captures[2].parse::<u64>().ok()?;
            let (weaknesses, immunities) = get_weaknesses_and_immunities(&captures[3]);
            let attack_points = captures[4].parse::<u64>().ok()?;
            let attack_type = String::from(&captures[5]);
            let initiative_points = captures[6].parse::<u64>().ok()?;
            Some(
                Group {
                    id, 
                    group_type,
                    num_units,
                    hit_points,
                    attack_points,
                    attack_type,
                    initiative_points,
                    weaknesses,
                    immunities
                }
            )
        } else {
            let num_units = captures[1].parse::<u64>().ok()?;
            let hit_points = captures[2].parse::<u64>().ok()?;
            let attack_points = captures[4].parse::<u64>().ok()?;
            let attack_type = String::from(&captures[5]);
            let initiative_points = captures[6].parse::<u64>().ok()?;
            Some(
                Group {
                    id, 
                    group_type,
                    num_units,
                    hit_points,
                    attack_points,
                    attack_type,
                    initiative_points,
                    weaknesses: vec![],
                    immunities: vec![]
                }
            )
        }
    }

    fn get_effective_power(&self) -> u64 {
        self.num_units * self.attack_points
    }

    fn get_damage_dealt_to(&self, other: &Group) -> u64 {
        if other.immunities.contains(&self.attack_type) {
            0
        } else if !other.weaknesses.contains(&self.attack_type) {
            self.get_effective_power()
        } else {
            2 * self.get_effective_power()
        }
    }

    fn deal_damage(&mut self, damage: u64) {
        let num_dead_units = damage / self.hit_points;
        if num_dead_units < self.num_units {
            self.num_units -= num_dead_units;
        } else {
            self.num_units = 0;
        }
    }

    fn is_dead(&self) -> bool {
        self.num_units == 0
    }
}

fn get_groups(strings: &[String]) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    let empty_separator_index = strings.iter()
        .enumerate()
        .find(|&(_, string)| string.is_empty())
        .map(|(index, _)| index)
        .unwrap();
    let mut current_id: usize = 0;
    for group_string in &strings[1..empty_separator_index] {
        if let Some(group) = Group::from_string(current_id, GroupType::Immune, group_string) {
            current_id += 1;
            groups.push(group);
        }
    }
    for group_string in &strings[(empty_separator_index + 2)..] {
        if let Some(group) = Group::from_string(current_id, GroupType::Infectious, group_string) {
            current_id += 1;
            groups.push(group);
        }
    }
    groups
}

fn get_target_selection(groups: &[Group]) -> HashMap<usize, usize> {
    let mut selection: HashMap<usize, usize> = HashMap::new();
    let mut selected: HashSet<usize> = HashSet::new();
    let mut sorted_groups = groups.to_vec();
    sorted_groups.sort_by(|first, second| {
        second.get_effective_power().cmp(&first.get_effective_power())
            .then(second.initiative_points.cmp(&first.initiative_points))
    });
    for group in &sorted_groups {
        let mut selectable_groups: Vec<_> = sorted_groups.iter()
            .filter(|&other_group| {
                other_group.group_type != group.group_type && !selected.contains(&other_group.id)
                    && group.get_damage_dealt_to(other_group) != 0
            })
            .collect();
        selectable_groups.sort_by(|&first, &second| {
            let damage_to_first_group = group.get_damage_dealt_to(first);
            let damage_to_second_group = group.get_damage_dealt_to(second);
            let effective_power_first_group = first.get_effective_power();
            let effective_power_second_group = second.get_effective_power();
            let initiative_first_group = first.initiative_points;
            let initiative_second_group = second.initiative_points;
            damage_to_second_group.cmp(&damage_to_first_group)
                .then(effective_power_second_group.cmp(&effective_power_first_group))
                .then(initiative_second_group.cmp(&initiative_first_group))
        });
        if let Some(selected_group) = selectable_groups.get(0) {
            selection.insert(group.id, selected_group.id);
            selected.insert(selected_group.id);
        }
    }
    selection
}

fn execute_step(groups: &mut Vec<Group>) -> bool {
    let selection = get_target_selection(groups);
    if selection.is_empty() {
        return true;
    }
    groups.sort_by(|first, second| second.initiative_points.cmp(&first.initiative_points));
    for group_index in 0..groups.len() {
        if !groups[group_index].is_dead() {
            if let Some(&selected_target_id) = selection.get(&groups[group_index].id) {
                let selected_group_index_option = groups.iter()
                    .enumerate()
                    .find(|&(_, other_group)| other_group.id == selected_target_id)
                    .map(|(index, _)| index);
                if let Some(selected_group_index) = selected_group_index_option {
                    let is_alive = !groups[selected_group_index].is_dead();
                    let damage_dealt = groups[group_index].get_damage_dealt_to(&groups[selected_group_index]);
                    if is_alive {
                        groups[selected_group_index].deal_damage(damage_dealt);
                    }
                }
            }
        }
    }
    groups.retain(|group| !group.is_dead());
    is_finished(groups)
}

fn is_finished(groups: &[Group]) -> bool {
    let num_immune_groups = groups.iter()
        .filter(|group| group.group_type == GroupType::Immune)
        .count();
    let num_infectious_groups = groups.iter()
        .filter(|group| group.group_type == GroupType::Infectious)
        .count();
    num_immune_groups == 0 || num_infectious_groups == 0
}

fn execute_until_finished(original_groups: &mut Vec<Group>) {
    let mut current_groups = original_groups.clone();
    let mut previous_groups = original_groups.clone();
    let mut is_finished = execute_step(&mut current_groups);
    while !is_finished && previous_groups != current_groups {
        previous_groups = current_groups.clone();
        is_finished = execute_step(&mut current_groups);
    }
    *original_groups = current_groups;
}

fn has_immune_system_won(groups: &[Group]) -> bool {
    groups.iter().filter(|group| group.group_type == GroupType::Infectious).count() == 0
}

fn get_total_num_of_units(groups: &[Group]) -> u64 {
    groups.iter()
        .map(|group| group.num_units)
        .sum()
}

fn get_num_immune_groups_after_victory(groups: &[Group]) -> u64 {
    let mut current_attack_points_boost: u64 = 1;
    let mut is_not_done = true;
    let mut result: u64 = 0;
    while is_not_done {
        let mut current_groups: Vec<_> = groups.iter()
            .map(|group| {
                match group.group_type {
                    GroupType::Immune => Group {
                        attack_points: group.attack_points + current_attack_points_boost,
                        ..group.clone()
                    },
                    GroupType::Infectious => group.clone(),
                }
            })
            .collect();
        execute_until_finished(&mut current_groups);
        if has_immune_system_won(&current_groups) {
            is_not_done = false;
            result = get_total_num_of_units(&&current_groups);
        } else {
            current_attack_points_boost += 1;
        }
    }
    result
}

pub fn solve_part_one() {
    let strings = read_lines("day_twentyfour.txt");
    let mut groups = get_groups(&strings);
    execute_until_finished(&mut groups);
    let answer = get_total_num_of_units(&groups);
    println!("{}", answer);
}

pub fn solve_part_two() {
    let strings = read_lines("day_twentyfour.txt");
    let groups = get_groups(&strings);
    let answer = get_num_immune_groups_after_victory(&groups);
    println!("{}", answer);
}