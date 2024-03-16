use crate::days::parsing;

pub const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Card {
    pub value: char
}

impl Card {
    pub fn rank(&self) -> u16 {
        match self.value {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'A' => 10,
            'K' => 11,
            'Q' => 12,
            'J' => 13,
            'T' => 14,
            _ => 15
        }
    }
    pub fn new(value: char) -> Card {
        Card { value }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

impl HandType {
    pub fn rank(&self) -> u16 {
        match self {
            HandType::FiveOfAKind => 6,
            HandType::FourOfAKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Hand {
    pub cards: [Card; 5],
    pub hand_type: HandType
}

impl Hand {
    pub fn compare(&self, other: &Card) -> u8 {
        //TODO: Compare the hand type by rank and then invididual cards by rank if hand type ranks match
        return 1
    }

    pub fn new(cards: [Card; 5], hand_type: HandType) -> Hand {
        Hand { cards, hand_type }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Bid {
    pub hand: Hand,
    pub amount: u16
}

impl Bid {
    pub fn new(hand: Hand, amount: u16) -> Bid {
        Bid { hand, amount }
    }
}

pub fn parse(input: &str) -> Vec<Bid> {
    //TODO: Implement
    Vec::new()
}

pub fn solution_part_1(input: &Vec<Bid>) -> u64 {
    //TODO: Implement
    1
}

pub fn solution_part_2(input: &Vec<Bid>) -> u64 {
    //TODO: Implement
    1
}