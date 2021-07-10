use std::{u8, usize};

struct RecipesState {
    recipes_score: Vec<u8>,
    first_index: usize,
    second_index: usize
}

impl RecipesState {
    fn new() -> RecipesState {
        RecipesState {
            recipes_score: vec![3, 7],
            first_index: 0,
            second_index: 1
        }
    }
    
    fn update(&mut self) {
        let added_scores = self.recipes_score[self.first_index] + self.recipes_score[self.second_index];
        if added_scores < 10 {
            self.recipes_score.push(added_scores);
        } else {
            self.recipes_score.push(added_scores / 10);
            self.recipes_score.push(added_scores % 10);
        }
        let current_len = self.recipes_score.len();
        let previous_first_index = self.first_index;
        let previous_second_index = self.second_index;
        self.first_index = (previous_first_index + (1 + self.recipes_score[previous_first_index] as usize) % current_len) % current_len;
        self.second_index = (previous_second_index + (1 + self.recipes_score[previous_second_index] as usize) % current_len) % current_len;
    }

    fn update_num_steps(&mut self, steps: usize) -> String {
        while self.recipes_score.len() < steps + 10 {
            self.update();
        }
        (&self.recipes_score[steps..steps + 10]).iter()
            .map(|score| score.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    fn get_num_recipes_until_pattenr(&mut self, pattern: &[u8]) -> usize {
        let mut is_done = false;
        let mut result: usize = 0;
        while !is_done {
            if self.recipes_score.len() >= pattern.len() {
                let last_recipes = &self.recipes_score[(self.recipes_score.len() - pattern.len())..];
                let condition = last_recipes.iter().zip(pattern.iter())
                    .all(|(&first, &second)| first == second);
                is_done = is_done || condition;
                if is_done {
                    result = self.recipes_score.len() - pattern.len();
                }
                if self.recipes_score.len() > pattern.len() && !is_done {
                    let last_recipes = &self.recipes_score[(self.recipes_score.len() - pattern.len() - 1)..];
                    let condition = last_recipes.iter().zip(pattern.iter())
                    .all(|(&first, &second)| first == second);
                    is_done = is_done || condition;
                    if is_done {
                        result = self.recipes_score.len() - pattern.len() - 1;
                    }
                }
            }
            if !is_done {
                self.update();
            }
        }
        result
    }
}

pub fn solve_part_one(steps: usize) {
    let mut recipes_state = RecipesState::new();
    let answer = recipes_state.update_num_steps(steps);
    println!("{}", answer);
}

pub fn solve_part_two(pattern: &[u8]) {
    let mut recipes_state = RecipesState::new();
    let answer = recipes_state.get_num_recipes_until_pattenr(pattern);
    println!("{}", answer);
}