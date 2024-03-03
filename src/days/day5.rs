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

pub fn parse(input: &str) -> Almanac {
    let lines: Vec<&str> = input.split_terminator('\n')
        .map(|line| line.trim())
        .collect();
    //TODO: Implement
    Almanac {
        inputs: Vec::new(),
        maps: Vec::new()
    }
}

pub fn solution_part_1(almanac: &Almanac) -> u32 {
    1
}

pub fn solution_part_2(almanac: &Almanac) -> u32 {
    1
}