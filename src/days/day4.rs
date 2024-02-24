pub const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Card {
    pub winning_numbers: Vec<i16>,
    pub present_numbers: Vec<i16>
}

impl Card {
    pub fn new(winning_numbers: Vec<i16>, present_numbers: Vec<i16>) -> Card {
        Card {winning_numbers, present_numbers}
    }
}

fn parse_line(line: &str) -> Card {
    //TODO: Implement
    Card {
        winning_numbers: Vec::new(),
        present_numbers: Vec::new()
    }
}

pub fn parse(input: &str) -> Vec<Card> {
    input.split_terminator('\n')
        .map(|line| parse_line(line.trim()))
        .collect()
}

pub fn solution_part_1(input: &Vec<Card>) -> u16 {
    1
}

pub fn solution_part_2(input: &Vec<Card>) -> u16 {
    2
}