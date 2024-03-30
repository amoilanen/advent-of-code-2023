use crate::days::parsing;
use parsing::ParsingError;
use regex::Regex;

use std::error::Error;
use core::result::Result::Err;


pub const INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

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

pub fn solution_part_1(parsed_input: &Map) -> u32 {
    1
}

pub fn solution_part_2(parsed_input: &Map) -> u32 {
    1
}