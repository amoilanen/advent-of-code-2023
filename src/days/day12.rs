use crate::days::parsing;
use std::error::Error;
use parsing::ParsingError;

pub const INPUT: &str = "#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1";

#[derive(Debug)]
#[derive(PartialEq)]
pub struct ConditionRecord {
    pub springs: Vec<char>,
    pub group_sizes: Vec<u32>
}

impl ConditionRecord {
    pub fn new(springs: Vec<char>, group_sizes: Vec<u32>) -> ConditionRecord {
        ConditionRecord { springs, group_sizes }
    }
}

fn parse_condition_record(condition_record_input: &str) -> Result<ConditionRecord, Box<dyn Error>> {
    let (springs_input, group_sizes_input) =  condition_record_input.split_once(" ")
        .ok_or(ParsingError::raise(format!("Could not find blank separator in {}", condition_record_input)))?;
    let mut group_sizes: Vec<u32> = Vec::new();
    for group_size in group_sizes_input.split(",") {
        group_sizes.push(group_size.parse()?)
    }
    Ok(ConditionRecord {
        springs: springs_input.chars().collect(),
        group_sizes: group_sizes
    })
}

pub fn parse(input: &str) -> Result<Vec<ConditionRecord>, Box<dyn Error>> {
    let lines: Vec<&str> = parsing::as_lines(input).iter().map(|line| line.trim()).filter(|line| !line.is_empty()).collect();
    let mut condition_records: Vec<ConditionRecord> = Vec::new();
    for condition_record_input in lines.iter() {
        condition_records.push(parse_condition_record(condition_record_input)?)
    }
    Ok(condition_records)
}

pub fn solution_part_1(input: &Vec<ConditionRecord>) -> u64 {
    1
}

pub fn solution_part_2(input: &Vec<ConditionRecord>) -> u64 {
    1
}