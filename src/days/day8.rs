use std::collections::HashMap;
use std::error::Error;

use crate::days::parsing;
use parsing::ParsingError;
use regex::Regex;
use core::result::Result::Err;

pub const INPUT_PART_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

pub const INPUT_PART_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

#[derive(PartialEq, Debug)]
pub enum Direction {
    L, R
}

#[derive(PartialEq, Debug)]
pub struct Node {
    pub label: String,
    pub left: String,
    pub right: String
}

impl Node {
    pub fn new(label: &str, left: &str, right: &str) -> Node {
        Node {
            label: label.to_string(),
            left: left.to_string(),
            right: right.to_string()
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Map {
    pub instructions: Vec<Direction>,
    pub nodes: Vec<Node>
}

impl Map {
    fn node_hash(&self) -> HashMap<&str, &Node> {
        let mut hash: HashMap<&str, &Node> = HashMap::new();
        for node in self.nodes.iter() {
            hash.insert(&node.label, node);
        }
        hash
    }
}

fn parse_instructions(instrucitons_input: &str) -> Result<Vec<Direction>, Box<dyn Error>> {
    instrucitons_input.chars().map(|ch| match ch {
        'L' => Ok(Direction::L),
        'R' => Ok(Direction::R),
        direction => Err(ParsingError::raise(format!("Unknown direction {}", direction)))
    }).collect()
}

fn parse_node(node_input: &str) -> Result<Node, Box<dyn Error>> {
    let node_regex = Regex::new(r"(\S+)\s+=\s+\((\S+),\s+(\S+)\)")?;
    let regex_captures = node_regex.captures(node_input).ok_or(ParsingError::raise(format!("Could not parse node_input {}", node_input)))?;
    let node = regex_captures.get(1).ok_or(ParsingError::raise(format!("Could not find label in {}", node_input)))?.as_str();
    let left = regex_captures.get(2).ok_or(ParsingError::raise(format!("Could not find left node in {}", node_input)))?.as_str();
    let right = regex_captures.get(3).ok_or(ParsingError::raise(format!("Could not find right node in {}", node_input)))?.as_str();

    Ok(Node::new(node, left, right))
}

fn parse_nodes(nodes_input: &[&str]) -> Result<Vec<Node>, Box<dyn Error>> {
    nodes_input.iter().map(|node_input| parse_node(node_input)).collect()
}

pub fn parse(input: &str) -> Result<Map, Box<dyn Error>> {
    let lines: Vec<&str> = parsing::as_lines(input).iter().map(|line| line.trim()).filter(|line| !line.is_empty()).collect();
    let instructions_input = lines[0];
    let nodes_input = &lines[1..lines.len()];

    let instructions = parse_instructions(instructions_input)?;
    let nodes =parse_nodes(nodes_input)?;

    Ok(Map {
        instructions,
        nodes
    })
}

pub fn steps_to_reach<F>(from_node_label: &str, to_node_label_predicate: F, map: &Map) -> u64
where F: Fn(&str) -> bool {
    let node_hash = map.node_hash();
    let mut steps: u64 = 0;
    let mut instruction_index = 0;
    let mut current_node_label = from_node_label;
    while !to_node_label_predicate(current_node_label) {
        let current_instruction = &map.instructions[instruction_index];
        let next_node_label = match node_hash.get(current_node_label) {
            Some(node) =>
                match current_instruction {
                    Direction::L => &node.left,
                    Direction::R => &node.right
                },
            None => return 0
        };
        current_node_label = next_node_label;
        steps = steps + 1;
        instruction_index = instruction_index + 1;
        if instruction_index >= map.instructions.len() {
            instruction_index = 0;
        }
    }
    steps
}

pub fn greatest_commond_divisor(x: u64, y: u64) -> u64 {
    let mut a = x;
    let mut b = y;
    let mut remainder = a % b;
    while remainder != 0 {
        a = b;
        b = remainder;
        remainder = a % b;
    }
    b
}

pub fn least_common_multiple(x: u64, y: u64) -> u64 {
    let gcd = greatest_commond_divisor(x, y);
    x * y / gcd
}

pub fn solution_part_1(parsed_input: &Map) -> u64 {
    steps_to_reach("AAA", |label| label =="ZZZ", parsed_input)
}

pub fn solution_part_2(parsed_input: &Map) -> u64 {
    let starting_nodes: Vec<&Node> = parsed_input.nodes.iter().filter(|node| node.label.ends_with("A")).collect();
    let step_counts: Vec<u64> = starting_nodes.iter().map(
        |node| 
        steps_to_reach(&node.label, |label| label.ends_with("Z"), parsed_input)
    ).collect();
    if let Some(&(mut minimal_number_of_steps)) = step_counts.first() {
        for step_count in step_counts {
            minimal_number_of_steps = least_common_multiple(minimal_number_of_steps, step_count)
        }
        return minimal_number_of_steps;
    }
    0
}