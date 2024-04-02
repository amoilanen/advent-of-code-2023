pub const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

#[derive(Debug)]
#[derive(PartialEq)]
pub struct ValueHistory {
    pub values: Vec<u32>
}

impl ValueHistory {
    pub fn new(values: Vec<u32>) -> ValueHistory {
        ValueHistory { values }
    }
}

pub fn parse(input: &str) -> Vec<ValueHistory> {
    Vec::new()
}

pub fn solution_part_1(input: &Vec<ValueHistory>) -> u32 {
    1
}

pub fn solution_part_2(input: &Vec<ValueHistory>) -> u32 {
    1
}