use regex::Regex;
use core::str::FromStr;
use std::fmt::Debug;

pub const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[derive(PartialEq)]
#[derive(Debug)]
pub struct RangeRule {
    pub destination_start: u32,
    pub source_start: u32,
    pub range_length: u32
}

impl RangeRule {
    pub fn new(destination_start: u32, source_start: u32, range_length: u32) -> RangeRule {
        RangeRule {destination_start, source_start, range_length}
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Map {
    pub rules: Vec<RangeRule>
}

impl Map {
    pub fn new(rules: Vec<RangeRule>) -> Map {
        Map {rules}
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Almanac {
    pub inputs: Vec<u32>,
    pub maps: Vec<Map>
}

impl Almanac {
    pub fn new(inputs: Vec<u32>, maps: Vec<Map>) -> Almanac {
        Almanac {inputs, maps}
    }
}

fn parse_numbers<T>(numbers_input: &str) -> Vec<T> where T: FromStr,<T as FromStr>::Err: Debug {
    let whitespace_regex = Regex::new(r"\s+").unwrap();
    whitespace_regex.split(numbers_input)
        .filter(|numbers_input| !numbers_input.is_empty())
        .map(|number_input|
            number_input.parse().expect("Expected a number")
        ).collect()
}

pub fn parse(input: &str) -> Almanac {
    let lines: Vec<&str> = input.split_terminator('\n')
        .map(|line| line.trim())
        .collect();
    
    let (_, inputs_input) = lines[0].split_once(':').expect("Did not find a single separator :");
    let inputs: Vec<u32> = parse_numbers(inputs_input);

    let mut maps: Vec<Map> = Vec::new();
    
    let mut rest_of_lines = lines.iter().skip(2);
    let mut current_rules: Vec<RangeRule> = Vec::new();
    while let Some(line) = rest_of_lines.next() {
        if line.trim().is_empty() {
            let map = Map::new(current_rules);
            maps.push(map);
            current_rules = Vec::new();
        } else if line.find(':').is_some() {
            // Skip the mapping header
        } else {
            let line_numbers: Vec<u32> = parse_numbers(line);
            let (destination_start, source_start, range_length) = (line_numbers[0], line_numbers[1], line_numbers[2]);
            current_rules.push(RangeRule::new(destination_start, source_start, range_length));
        }
    }
    if !current_rules.is_empty() {
        let map = Map::new(current_rules);
        maps.push(map);
        current_rules = Vec::new();
    }
    Almanac {
        inputs,
        maps
    }
}

pub fn solution_part_1(almanac: &Almanac) -> u32 {
    1
}

pub fn solution_part_2(almanac: &Almanac) -> u32 {
    1
}