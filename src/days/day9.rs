use crate::days::parsing;

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

fn parse_value_history(value_history_input: &str) -> ValueHistory {
    let values: Vec<u32> = value_history_input.split(' ').map(|x| x.parse().unwrap()).collect();
    ValueHistory::new(values)
}

pub fn parse(input: &str) -> Vec<ValueHistory> {
    let lines: Vec<&str> = parsing::as_lines(input).iter().map(|line| line.trim()).filter(|line| !line.is_empty()).collect();
    let histories: Vec<ValueHistory> = lines.iter().map(|value_history_input| parse_value_history(value_history_input)).collect();
    histories
}

pub fn solution_part_1(input: &Vec<ValueHistory>) -> u32 {
    1
}

pub fn solution_part_2(input: &Vec<ValueHistory>) -> u32 {
    1
}