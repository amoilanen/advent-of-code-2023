use std::fmt::Debug;
use std::cmp::min;
use std::cmp::max;
use crate::days::parsing;

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
    pub destination_start: u64,
    pub source_start: u64,
    pub range_length: u64
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Hash)]
#[derive(Eq)]
#[derive(Clone)]
pub struct Range {
    pub start: u64,
    pub end: u64
}

impl Range {
    pub fn new(start: u64, end: u64) -> Range {
        Range {start, end}
    }
}

impl RangeRule {
    pub fn new(destination_start: u64, source_start: u64, range_length: u64) -> RangeRule {
        RangeRule {destination_start, source_start, range_length}
    }

    // Returns a part of first converted range if any and second any remaining non-converted ranges
    pub fn convert_range(&self, range: &Range) -> (Vec<Range>, Vec<Range>) {
        let source_end = self.source_start + self.range_length - 1;
        let mut converted: Vec<Range> = Vec::new();
        let mut remaining: Vec<Range> = Vec::new();
        if range.start < self.source_start {
            remaining.push(Range::new(range.start, min(range.end, self.source_start - 1)))
        }
        if range.end > source_end {
            remaining.push(Range::new(max(range.start, source_end + 1), range.end))
        }
        let to_be_converted_start = max(range.start, self.source_start);
        let to_be_converted_end = min(range.end, source_end);
        if to_be_converted_start <= to_be_converted_end {
            let converted_range = Range::new(self.convert(to_be_converted_start), self.convert(to_be_converted_end));
            converted.push(converted_range);
        }
        (converted, remaining)
    }

    pub fn convert(&self, input: u64) -> u64 {
        if input >= self.source_start && input <= self.source_start + self.range_length - 1 {
            self.destination_start + (input - self.source_start)
        } else {
            input
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Map {
    pub rules: Vec<RangeRule>
}

impl Map {
    pub fn convert(&self, input: u64) -> u64 {
        for rule in &self.rules {
            let converted_input = rule.convert(input);
            if converted_input != input {
                return converted_input
            }
        }
        return input
    }
    pub fn convert_ranges(&self, ranges: &Vec<Range>) -> Vec<Range> {
        let mut converted: Vec<Range> = Vec::new();
        let mut remaining: Vec<Range> = ranges.clone();
        for rule in &self.rules {
            let mut updated_remaining: Vec<Range> = Vec::new();
            for remaining_range in remaining {
                let (mut next_converted, mut next_remaining) = rule.convert_range(&remaining_range);
                updated_remaining.append(&mut next_remaining);
                converted.append(&mut next_converted);
            }
            remaining = updated_remaining;
        }
        converted.append(&mut remaining);
        converted
    }
    pub fn new(rules: Vec<RangeRule>) -> Map {
        Map {rules}
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Almanac {
    pub inputs: Option<Vec<u64>>,
    pub input_ranges: Option<Vec<Range>>,
    pub maps: Vec<Map>
}

impl Almanac {
    pub fn new(inputs: Option<Vec<u64>>, input_ranges: Option<Vec<Range>>, maps: Vec<Map>) -> Almanac {
        Almanac {inputs, input_ranges, maps}
    }
}

pub fn parse_for_part_1(input: &str) -> Almanac {
    parse(input,
        |input| Some(parsing::parse_numbers(input)),
        |_| None)
}

pub fn parse_for_part_2(input: &str) -> Almanac {
    parse(input,
        |_| None,
        |input| {
            let numbers: Vec<u64> = parsing::parse_numbers(input);
            let mut input_ranges = Vec::new();
            for idx in (0..numbers.len()).step_by(2) {
                let range_start = numbers[idx];
                let range_length = numbers[idx + 1];
                input_ranges.push(Range::new(range_start, range_start + range_length - 1))
            }
            Some(input_ranges)
        })
}

pub fn parse<F1, F2>(input: &str, parse_inputs: F1, parse_input_ranges: F2) -> Almanac 
where F1: Fn(&str) -> Option<Vec<u64>>, F2: Fn(&str) -> Option<Vec<Range>> {
    let lines: Vec<&str> = parsing::as_lines(input);

    let (_, inputs_input) = lines[0].split_once(':').expect("Did not find a single separator :");

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
            let line_numbers: Vec<u64> = parsing::parse_numbers(line);
            let (destination_start, source_start, range_length) = (line_numbers[0], line_numbers[1], line_numbers[2]);
            current_rules.push(RangeRule::new(destination_start, source_start, range_length));
        }
    }
    if !current_rules.is_empty() {
        let map = Map::new(current_rules);
        maps.push(map);
    }
    Almanac {
        inputs: parse_inputs(&inputs_input),
        input_ranges: parse_input_ranges(&inputs_input),
        maps
    }
}

pub fn get_conversions(input: u64, maps: &Vec<Map>) -> Vec<u64> {
    let mut current_value: u64 = input;
    let mut conversions: Vec<u64> = vec![current_value];
    for map in maps {
        current_value = map.convert(current_value);
        conversions.push(current_value);
    }
    conversions
}

pub fn get_range_conversions(ranges: &Vec<Range>, maps: &Vec<Map>) -> Vec<Range> {
    let mut conversions: Vec<Range> = ranges.clone();
    for map in maps {
        conversions = map.convert_ranges(&conversions);
    }
    conversions
}

pub fn solution_part_1(almanac: &Almanac) -> u64 {
    let mut final_conversions: Vec<u64> = Vec::new();
    if let Some(inputs) = almanac.inputs.clone() {
        for input in inputs {
            let input_conversions = get_conversions(input, &almanac.maps);
            final_conversions.push(input_conversions.last().unwrap().clone());
        }
    }
    final_conversions.iter().min().unwrap().clone()
}

pub fn solution_part_2(almanac: &Almanac) -> u64 {
    let mut result: u64 = 0;
    if let Some(input_ranges) = almanac.input_ranges.clone() {
        let final_conversions = get_range_conversions(&input_ranges, &almanac.maps);
        result = final_conversions.iter().map(|range| range.start).min().unwrap()
    }
    result
}