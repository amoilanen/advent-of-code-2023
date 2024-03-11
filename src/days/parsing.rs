use regex::Regex;
use std::fmt::Debug;
use core::str::FromStr;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub struct ParsingError {
    pub message: String,
}

impl ParsingError {
    pub fn raise(message: String) -> Box<dyn Error> {
        Box::new(ParsingError { message })
    }
}

impl Error for ParsingError {}

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter) ->  Result {
      write!(f, "{}", self.message)
    }
}


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