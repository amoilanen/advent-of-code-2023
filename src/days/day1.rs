use std::collections::HashMap;

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

pub fn line_to_number(line: &str) -> u32 {

    let DIGIT_TO_VALUE: HashMap<&str, u8> = {
        let mut digitToValue = HashMap::new();
        digitToValue.insert("one", 1);
        digitToValue.insert("two", 2);
        digitToValue.insert("three", 3);
        digitToValue.insert("four", 4);
        digitToValue.insert("five", 5);
        digitToValue.insert("six", 6);
        digitToValue.insert("seven", 7);
        digitToValue.insert("eight", 8);
        digitToValue.insert("nine", 9);
        digitToValue.insert("1", 1);
        digitToValue.insert("2", 2);
        digitToValue.insert("3", 3);
        digitToValue.insert("4", 4);
        digitToValue.insert("5", 5);
        digitToValue.insert("6", 6);
        digitToValue.insert("7", 7);
        digitToValue.insert("8", 8);
        digitToValue.insert("9", 9);
        digitToValue
    };

    let DIGIT_PATTERNS: Vec<&str> = DIGIT_TO_VALUE.keys().cloned().collect();

    let mut first_indices: Vec<(usize, u8)> = Vec::new();
    for digit_pattern in &DIGIT_PATTERNS {
        if let Some(idx) = line.find(digit_pattern) {
            if let Some(digit_value) = DIGIT_TO_VALUE.get(digit_pattern) {
                first_indices.push((idx, *digit_value))
            }
        }
    }
    let mut first_digit: u8 = 0;
    if let Some((_, digit_value)) = first_indices.iter().min_by(|x, y| x.0.cmp(&y.0)) {
        first_digit = *digit_value;
    }

    let mut last_indices: Vec<(usize, u8)> = Vec::new();
    for digit_pattern in &DIGIT_PATTERNS {
        if let Some(idx) = line.rfind(digit_pattern) {
            if let Some(digit_value) = DIGIT_TO_VALUE.get(digit_pattern) {
                last_indices.push((idx, *digit_value))
            }
        }
    }
    let mut last_digit: u8 = 0;
    if let Some((_, digit_value)) = last_indices.iter().max_by(|x, y| x.0.cmp(&y.0)) {
        last_digit = *digit_value;
    }
    (first_digit as u32) * 10 + (last_digit as u32)
}

pub fn solution(parsed_input: &Vec<&str>) -> u32 {
    let numbers = parsed_input.iter().map(|line|
        line_to_number(line)
    );
    numbers.sum()
}

pub fn solution_part_1(parsed_input: &Vec<&str>) -> u32 {
    solution(parsed_input)
}

pub fn solution_part_2(parsed_input: &Vec<&str>) -> u32 {
    solution(parsed_input)
}