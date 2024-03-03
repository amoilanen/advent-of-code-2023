use crate::days::parsing;
use std::collections::HashMap;

pub const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
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

    pub fn matches_number(&self) -> u16 {
        self.present_numbers.iter().filter(|present_number| self.winning_numbers.contains(present_number)).count() as u16
    }

    pub fn points(&self) -> u16 {
        let matches_number = self.matches_number();
        if matches_number > 0 {
            2u16.pow(matches_number as u32 - 1)
        } else {
            0
        }
    }
}

fn parse_line(line: &str) -> Card {
    let (card_info_input, numbers_input) = line.split_once(':').expect("Did not find a single separator :");
    let (winning_input, present_input) = numbers_input.split_once('|').expect("Did not find a single separator |");
    let card_id_input = card_info_input.split(' ').last().expect("Did not find the card id input");
    Card {
        id: card_id_input.parse().expect("Card id should be a number"),
        winning_numbers: parsing::parse_numbers(winning_input),
        present_numbers: parsing::parse_numbers(present_input)
    }
}

pub fn parse(input: &str) -> Vec<Card> {
    parsing::as_lines(input).iter().map(|line| parse_line(line.trim())).collect()
}

pub fn solution_part_1(cards: &Vec<Card>) -> u16 {
    cards.iter().map(|card| card.points()).sum()
}

fn get_full_card_counts(cards: &Vec<Card>) -> HashMap<u16, u32> {
    let mut card_id_to_card: HashMap<u16, &Card> = HashMap::new();
    for card in cards {
        card_id_to_card.insert(card.id, &card);
    }
    let mut card_ids: Vec<u16> = card_id_to_card.keys().cloned().collect();
    card_ids.sort();

    let mut card_counts: HashMap<u16, u32> = HashMap::new();
    for card in cards {
        card_counts.insert(card.id, 1);
    }
    for card_id in card_ids {
        let card = &card_id_to_card.get(&card_id).unwrap();
        let current_card_matches = card.matches_number();
        for subsequent_card_id in (card.id + 1)..=(card.id + current_card_matches) {
            let current_card_count = (&card_counts).get(&card.id).unwrap_or(&0);
            let updated_count = (&card_counts).get(&subsequent_card_id).unwrap_or(&0) + current_card_count;
            card_counts.insert(subsequent_card_id, updated_count);
        }
    }
    card_counts
}

pub fn solution_part_2(cards: &Vec<Card>) -> u32 {
    get_full_card_counts(&cards).values().sum()
}