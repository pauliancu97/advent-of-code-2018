use std::{cell::RefCell, clone, rc::Rc, thread::current, u64, usize, vec};

#[derive(Debug)]
struct Node {
    data: u64,
    next: usize,
    prev: usize
}

impl Node {
    fn new(data: u64) -> Node {
        Node {
            data,
            next: 0,
            prev: 0
        }
    }
}

#[derive(Debug)]
struct CircularLinkedList {
    nodes: Vec<Node>
}

impl CircularLinkedList {

    fn from_vec(vector: &Vec<u64>) -> CircularLinkedList {
        let mut circular_linked_list = CircularLinkedList { nodes: Vec::new() };
        for it in vector {
            circular_linked_list.add(*it);
        }
        circular_linked_list
    }

    fn add(&mut self, data: u64) -> usize {
        if self.nodes.is_empty() {
            self.nodes.push(Node::new(data));
            0
        } else {
            self.insert(self.nodes.len() - 1, data)
        }
    }

    fn insert(&mut self, node_index: usize, data: u64) -> usize {
        let mut new_node  = Node::new(data);
        let new_node_index = self.nodes.len();
        let current_node_next = self.nodes[node_index].next;
        {
            let current_node = &mut self.nodes[node_index];
            current_node.next = new_node_index;
        };
        {
            let next_node = &mut self.nodes[current_node_next];
            next_node.prev = new_node_index;
        };
        new_node.prev = node_index;
        new_node.next = current_node_next;
        self.nodes.push(new_node);
        new_node_index
    }

    fn remove(&mut self, node_index: usize) -> usize {
        let current_node_prev = self.nodes[node_index].prev;
        let current_node_next = self.nodes[node_index].next;
        {
            let next_node = &mut self.nodes[current_node_next];
            next_node.prev = current_node_prev;
        };
        {
            let prev_node = &mut self.nodes[current_node_prev];
            prev_node.next = current_node_next;
        };
        current_node_next
    }

    fn get_node_after(&self, index: usize, steps: usize) -> usize {
        let mut current_index = index;
        for _ in 0..steps {
            current_index = self.nodes[current_index].next;
        }
        current_index
    }

    fn get_node_before(&self, index: usize, steps: usize) -> usize {
        let mut current_index = index;
        for _ in 0..steps {
            current_index = self.nodes[current_index].prev;
        }
        current_index
    }

    fn get_value(&self, index:usize) -> u64 {
        self.nodes[index].data
    }
}

pub fn test_circular_list() {
    let mut circular_linked_list = CircularLinkedList::from_vec(&vec![0, 2, 1]);
    circular_linked_list.remove(0);
    println!("{:#?}", circular_linked_list);
}
struct CircularList {
    current_index: i64,
    list: Vec<u64>
}

impl CircularList {
    fn new() -> CircularList {
        CircularList {
            current_index: 1,
            list: vec![0, 2, 1]
        }
    }

    fn add_element(&mut self, element: u64) -> u64 {
        if element % 23 != 0 {
            let next_index = if self.current_index + 2 > (self.list.len() as i64) {
                self.current_index - (self.list.len() as i64) + 2
            } else {
                self.current_index + 2
            };
            self.list.insert(next_index as usize, element);
            self.current_index = next_index;
            0
        } else {
            let remove_index = if (self.current_index as i64) - 7 < 0 {
                (self.list.len() as i64) + self.current_index - 7
            } else {
                self.current_index - 7
            };
            let score = self.list.remove(remove_index as usize) + element;
            self.current_index = if remove_index == (self.list.len() as i64) {
                0
            } else {
                remove_index
            };
            score
        }
    }
}

struct Game {
    list: CircularLinkedList,
    current_index: i64
}

impl Game {
    fn new() -> Game {
        Game {
            list: CircularLinkedList::from_vec(&vec![0, 2, 1]),
            current_index: 1
        }
    }

    fn add_element(&mut self, element: u64) -> u64 {
        if element % 23 != 0 {
            let insert_index = self.list.get_node_after(self.current_index as usize, 1);
            self.current_index = self.list.insert(insert_index, element) as i64;
            0
        } else {
            let remove_index = self.list.get_node_before(self.current_index as usize, 7);
            let score = self.list.get_value(remove_index) + element;
            self.current_index = self.list.remove(remove_index) as i64;
            score
        }
    }
}

fn get_highest_player_score(num_players: usize, num_turns: u64) -> u64 {
    let mut players_score: Vec<u64> = vec![0; num_players];
    let mut circular_list = CircularList::new();
    let mut current_player_index: usize = 2;
    for turn in 3..=num_turns {
        players_score[current_player_index] += circular_list.add_element(turn);
        current_player_index = if current_player_index + 1 >= num_players {
            0
        } else {
            current_player_index + 1
        };
    }
    players_score.iter().max().map(|it| *it).unwrap()
}

fn get_highest_player_score_with_linked_list(num_players: usize, num_turns: u64) -> u64 {
    let mut players_score: Vec<u64> = vec![0; num_players];
    let mut circular_list = Game::new();
    let mut current_player_index: usize = 2;
    for turn in 3..=num_turns {
        players_score[current_player_index] += circular_list.add_element(turn);
        current_player_index = if current_player_index + 1 >= num_players {
            0
        } else {
            current_player_index + 1
        };
    }
    players_score.iter().max().map(|it| *it).unwrap()
}

pub fn solve_part_one(num_players: usize, num_turns: u64) {
    println!("{}", get_highest_player_score(num_players, num_turns));
}

pub fn solve_part_two(num_players: usize, num_turns: u64) {
    println!("{}", get_highest_player_score_with_linked_list(num_players, num_turns));
}