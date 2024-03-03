use regex::Regex;
use std::fmt::Debug;
use core::str::FromStr;

pub fn as_lines(input: &str) -> Vec<&str> {
    input.split_terminator('\n')
        .map(|line| line.trim())
        .collect()
}

pub fn parse_numbers<T>(numbers_input: &str) -> Vec<T> where T: FromStr,<T as FromStr>::Err: Debug {
    let whitespace_regex = Regex::new(r"\s+").unwrap();
    whitespace_regex.split(numbers_input)
        .filter(|numbers_input| !numbers_input.is_empty())
        .map(|number_input|
            number_input.parse().expect("Expected a number")
        ).collect()
}