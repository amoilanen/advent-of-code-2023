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

pub fn pairs_of<T: Copy>(values: &Vec<T>) -> Vec<(T, T)> {
    let mut result: Vec<(T, T)> = Vec::new();
    let mut idx = 1;
    while idx < values.len() {
        result.push((values[idx -1], values[idx]));
        idx = idx + 1;
    }
    result
}

pub fn differences_of(values: &Vec<u32>) -> Vec<u32> {
    let pairs = pairs_of(values);
    pairs.iter().map(|pair| {
        pair.1 - pair.0
    }).collect()
}

pub fn is_all_zeros(values: &Vec<u32>) -> bool {
    values.iter().all(|x| *x == 0)
}

pub fn extrapolate_next(values: &Vec<u32>) -> u32 {
    let mut current_differences = values.clone();
    let mut all_differences: Vec<Vec<u32>> = Vec::new();
    while !is_all_zeros(&current_differences) {
        let next_differences = differences_of(&current_differences);
        all_differences.push(current_differences);
        current_differences = next_differences;
    }
    all_differences.iter().map(|differences| differences.last().unwrap()).sum()
}

pub fn solution_part_1(input: &Vec<ValueHistory>) -> u32 {
    input.iter().map(|history| extrapolate_next(&history.values)).sum()
}

pub fn solution_part_2(input: &Vec<ValueHistory>) -> u32 {
    1
}