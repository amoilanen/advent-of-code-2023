use regex::Regex;

pub const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83;
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Card {
    pub id: u16,
    pub winning_numbers: Vec<u16>,
    pub present_numbers: Vec<u16>
}

impl Card {
    pub fn new(id: u16, winning_numbers: Vec<u16>, present_numbers: Vec<u16>) -> Card {
        Card {id, winning_numbers, present_numbers}
    }
}

fn parse_numbers(numbers_input: &str) -> Vec<u16> {
    let whitespace_regex = Regex::new(r"\s+").unwrap();
    whitespace_regex.split(numbers_input)
        .filter(|numbers_input| !numbers_input.is_empty())
        .map(|number_input|
            number_input.parse().expect("Expected a number")
        ).collect()
}


fn parse_line(line: &str) -> Card {
    let (card_info_input, numbers_input) = line.split_once(':').expect("Did not find a single separator :");
    let (winning_input, present_input) = numbers_input.split_once('|').expect("Did not find a single separator |");
    let (_, card_id_input) = card_info_input.split_once(' ').expect("Did not find the card id input");
    Card {
        id: card_id_input.parse().expect("Card id should be a number"),
        winning_numbers: parse_numbers(winning_input),
        present_numbers: parse_numbers(present_input)
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