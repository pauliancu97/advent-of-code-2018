use core::num;
use std::{cmp::min, collections::{HashMap, HashSet}, usize};
use regex::Regex;
use crate::utils::read_lines;
struct Dependency {
    task: char,
    dependency: char
}

struct Task {
    id: char,
    remaining_time: usize
}

impl Task {
    fn new(id: char, additional_time: usize) -> Task {
        let remaining_time = (id as usize) - ('A' as usize) + 1 + additional_time;
        Task { id, remaining_time }
    }
}

impl Dependency {
    fn from_string(string: &str) -> Option<Dependency> {
        let regex = Regex::new(r"Step (.) must be finished before step (.) can begin\.").ok()?;
        let captures = regex.captures(string)?;
        let task = captures[2].as_bytes().first().map(|byte| *byte as char)?;
        let dependency = captures[1].as_bytes().first().map(|byte| *byte as char)?;
        Some(Dependency { task, dependency })
    }
}

fn get_dependencies(strings: &Vec<String>) -> Vec<Dependency> {
    strings.iter().filter_map(|string| Dependency::from_string(string)).collect()
}

fn get_tasks_graph(dependencies: &Vec<Dependency>) -> (HashSet<char>, HashMap<char, Vec<char>>) {
    let tasks_set: HashSet<_> = dependencies.iter()
        .flat_map(|dependency| vec![dependency.task, dependency.dependency].into_iter())
        .collect();
    let mut tasks_dependencies: HashMap<char, Vec<char>> = HashMap::new();
    for task in &tasks_set {
        tasks_dependencies.insert(*task, Vec::new());
    }
    for dependency in dependencies {
        let task_dependencies = tasks_dependencies.entry(dependency.task).or_insert(Vec::new());
        task_dependencies.push(dependency.dependency);
    }
    (tasks_set, tasks_dependencies)
}

fn get_tasks_in_order(tasks_set: &HashSet<char>, tasks_dependencies: &mut HashMap<char, Vec<char>>) -> String {
    let mut result: String = String::from("");
    let mut is_not_done = true;
    let mut visited: HashSet<char> = HashSet::new();
    while is_not_done {
        let current_task = tasks_set.iter()
            .filter(|task| {
                tasks_dependencies.get(*task).unwrap().is_empty() && !visited.contains(*task)
            })
            .map(|chr| *chr)
            .min()
            .unwrap();
        visited.insert(current_task);
        result += &(current_task.to_string());
        for task in tasks_set {
            if let Some(dependencies) = tasks_dependencies.get_mut(task) {
                let updated_dependencies: Vec<_> = dependencies.iter()
                    .filter(|&&chr| chr != current_task)
                    .map(|&chr| chr)
                    .collect(); 
                *dependencies = updated_dependencies;
            }
        }
        is_not_done = visited.len() != tasks_set.len();
    }
    result
}

fn get_task_completion_time(
    tasks_set: &HashSet<char>,
    tasks_dependencies: &mut HashMap<char, Vec<char>>,
    num_workers: usize,
    additional_time: usize
) -> usize {
    let mut current_second: usize = 0;
    let mut is_not_done = true;
    let mut completed: HashSet<char> = HashSet::new();
    let mut unstarted: HashSet<char> = tasks_set.clone();
    let mut workers_tasks: Vec<Option<Task>> = Vec::new();
    for _ in 0..num_workers {
        workers_tasks.push(None);
    }
    while is_not_done {
        for worker_task in &mut workers_tasks {
            if let Some(task) = worker_task {
                if task.remaining_time == 0 {
                    completed.insert(task.id);
                    for task_id in tasks_set {
                        if let Some(dependencies) = tasks_dependencies.get_mut(task_id) {
                            let updated_dependencies: Vec<_> = dependencies.iter()
                                .filter(|&&chr| chr != task.id)
                                .map(|&chr| chr)
                                .collect(); 
                            *dependencies = updated_dependencies;
                        }
                    }
                    *worker_task = None;
                }
            }
        }
        let mut doable_tasks: Vec<_> = unstarted.iter()
            .filter(|&id| tasks_dependencies.get(id).unwrap().is_empty())
            .map(|id| *id)
            .collect();
        doable_tasks.sort();
        let mut free_workers: Vec<_> = workers_tasks.iter_mut()
            .filter(|option_task| (**option_task).is_none())
            .collect();
        let min_len = min(doable_tasks.len(), free_workers.len());
        for index in 0..min_len {
            *free_workers[index] = Some(Task::new(doable_tasks[index], additional_time));
            unstarted.remove(&doable_tasks[index]);
        }
        for worker_task in &mut workers_tasks {
            if let Some(task) = worker_task {
                task.remaining_time -= 1;
            }
        }
        is_not_done = completed.len() != tasks_set.len();
        if is_not_done {
            current_second += 1;
        }
    }
    current_second
}

pub fn solve_part_one() {
    let strings = read_lines("day_seven.txt");
    let dependencies = get_dependencies(&strings);
    let (tasks_set, mut tasks_dependencies) = get_tasks_graph(&dependencies);
    let answer = get_tasks_in_order(&tasks_set, &mut tasks_dependencies);
    println!("{}", answer);
}

pub fn solve_part_two() {
    let strings = read_lines("day_seven.txt");
    let dependencies = get_dependencies(&strings);
    let (tasks_set, mut tasks_dependencies) = get_tasks_graph(&dependencies);
    let answer = get_task_completion_time(&tasks_set, &mut tasks_dependencies, 5, 60);
    println!("{}", answer);
}
