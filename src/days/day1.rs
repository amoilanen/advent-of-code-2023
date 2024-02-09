use std::collections::HashMap;
use std::cmp::Ordering;

pub const INPUT_PART_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

pub const INPUT_PART_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

pub fn parse(input: &str) -> Vec<&str> {
    input.split_terminator('\n').collect()
}

//Workaround for not being able to easily define a const which is initialized staticly and has a generic type
pub fn digits_to_value() -> HashMap<&'static str, u8> {
    [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9)
    ].iter().cloned().collect()
}

pub fn digit_patterns(digits_to_value: &HashMap<&'static str, u8>) -> Vec<&'static str> {
    digits_to_value.keys().cloned().collect()
}

fn find_digit(line: &str, pattern_finder: fn(&str, &str) -> Option<usize>, match_index_comparator: fn(&usize, &usize) -> Ordering, digits_to_value: &HashMap<&str, u8>, digit_patterns: &Vec<&str>) -> u8 {
    let mut indices: Vec<(usize, u8)> = Vec::new();
    for digit_pattern in digit_patterns {
        if let Some(idx) = pattern_finder(line, digit_pattern) {
            if let Some(digit_value) = digits_to_value.get(digit_pattern) {
                indices.push((idx, *digit_value))
            }
        }
    }
    let mut digit: u8 = 0;
    if let Some((_, digit_value)) = indices.iter().min_by(|x, y| match_index_comparator(&x.0, &y.0)) {
        digit = *digit_value;
    }
    digit
}

pub fn line_to_number(line: &str, digits_to_value: &HashMap<&str, u8>, digit_patterns: &Vec<&str>) -> u32 {
    let first_digit = find_digit(
        line,
        |line, digit_pattern| line.find(digit_pattern),
        |x, y| x.cmp(y),
        digits_to_value,
        digit_patterns
    );
    let last_digit = find_digit(
        line,
        |line, digit_pattern| line.rfind(digit_pattern),
        |x, y| y.cmp(x),
        digits_to_value,
        digit_patterns
    );
    (first_digit as u32) * 10 + (last_digit as u32)
}

pub fn solution(parsed_input: &Vec<&str>) -> u32 {
    let digits_to_value = digits_to_value();
    let digit_patterns = digit_patterns(&digits_to_value);
    let numbers = parsed_input.iter().map(|line|
        line_to_number(line, &digits_to_value, &digit_patterns)
    );
    numbers.sum()
}

pub fn solution_part_1(parsed_input: &Vec<&str>) -> u32 {
    solution(parsed_input)
}

pub fn solution_part_2(parsed_input: &Vec<&str>) -> u32 {
    solution(parsed_input)
}