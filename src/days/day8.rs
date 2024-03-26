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

pub fn parse(input: &str) -> Map {
    Map {
        instructions: Vec::new(),
        nodes: Vec::new()
    }
}

pub fn solution_part_1(parsed_input: &Map) -> u32 {
    1
}

pub fn solution_part_2(parsed_input: &Map) -> u32 {
    1
}