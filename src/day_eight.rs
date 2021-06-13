use std::u64;
use crate::utils::read_lines;

struct TreeNode {
    children: Vec<TreeNode>,
    meta_data: Vec<u64>
}

fn get_tree_aux<'a>(description: &'a[u64]) -> (TreeNode, &'a[u64]) {
    let num_children = description[0];
    let num_meta_data = description[1];
    let mut children: Vec<TreeNode> = Vec::new();
    let mut current_remaining_description = &description[2..];
    for _ in 0..num_children {
        let (child_node, remaining_description) = get_tree_aux(current_remaining_description);
        children.push(child_node);
        current_remaining_description = remaining_description;
    }
    let mut meta_data: Vec<u64> = Vec::new();
    for index in 0..num_meta_data {
        meta_data.push(current_remaining_description[index as usize]);
    }
    current_remaining_description = &current_remaining_description[(num_meta_data as usize)..];
    let tree_node = TreeNode { children, meta_data };
    (tree_node, current_remaining_description)
}

fn get_tree(description: &[u64]) -> TreeNode {
    get_tree_aux(description).0
}

fn get_meta_data_sum(tree: &TreeNode) -> u64 {
    let mut current_sum: u64 = tree.meta_data.iter().sum();
    for child in &tree.children {
        current_sum += get_meta_data_sum(child);
    }
    current_sum
}

fn get_node_value(tree: &TreeNode) -> u64 {
    if tree.children.len() == 0 {
        tree.meta_data.iter().sum()
    } else {
        tree.meta_data.iter()
            .filter(|&&child_index| child_index > 0 && child_index <= (tree.children.len() as u64))
            .map(|&child_index| get_node_value(&tree.children[child_index as usize - 1]))
            .sum()
    }
}

pub fn solve_part_one() {
    let line = &read_lines("day_eight.txt")[0];
    let description: Vec<_> = line.split(' ').filter_map(|string| string.parse::<u64>().ok()).collect();
    let tree = get_tree(&description);
    let answer = get_meta_data_sum(&tree);
    println!("{}", answer);
}

pub fn solve_part_two() {
    let line = &read_lines("day_eight.txt")[0];
    let description: Vec<_> = line.split(' ').filter_map(|string| string.parse::<u64>().ok()).collect();
    let tree = get_tree(&description);
    let answer = get_node_value(&tree);
    println!("{}", answer);
}