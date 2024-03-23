use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Ordering;

use crate::days::parsing;
use crate::days::compare;

pub const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

#[derive(Eq, Hash, PartialOrd, PartialEq, Debug, Clone, Copy)]
pub struct Card {
    pub value: char
}

impl Card {
    pub fn rank_part_1(&self) -> u16 {
        match self.value {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 15
        }
    }
    pub fn rank_part_2(&self) -> u16 {
        match self.value {
            'J' => 2,
            '2' => 3,
            '3' => 4,
            '4' => 5,
            '5' => 6,
            '6' => 7,
            '7' => 8,
            '8' => 9,
            '9' => 10,
            'T' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 15
        }
    }
    pub fn new(value: char) -> Card {
        Card { value }
    }
    fn cmp_part_1(first: &Self, second: &Self) -> Ordering {
        let first_rank = first.rank_part_1();
        let second_rank: u16 = second.rank_part_1();
        first_rank.cmp(&second_rank)
    }
    fn cmp_part_2(first: &Self, second: &Self) -> Ordering {
        let first_rank = first.rank_part_2();
        let second_rank: u16 = second.rank_part_2();
        first_rank.cmp(&second_rank)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
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

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        let own_rank = self.rank();
        let other_rank = other.rank();
        own_rank.cmp(&other_rank)
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Debug)]
pub struct Hand {
    pub cards: [Card; 5],
    pub hand_type_part_1: HandType,
    pub hand_type_part_2: HandType
}

impl Hand {
    pub fn new(cards: [Card; 5], hand_type: HandType) -> Hand {
        Hand { cards, hand_type_part_1: hand_type.clone(), hand_type_part_2: hand_type } //TODO: Determine the real hand type for part 2
    }

    pub fn as_string(&self) -> String {
      self.cards.map(|c| c.value.to_string()).concat()
    }

    pub fn cmp_part_1(&self, other: &Self) -> Ordering {
        let hand_type_comparison = self.hand_type_part_1.cmp(&other.hand_type_part_1);
        if hand_type_comparison != Ordering::Equal {
            hand_type_comparison
        } else {
            let card_comparison_result = compare::compare_arrays(&self.cards, &other.cards, Card::cmp_part_1);
            card_comparison_result
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Debug)]
pub struct Bid {
    pub hand: Hand,
    pub amount: u16
}

impl Bid {
    pub fn new(hand: Hand, amount: u16) -> Bid {
        Bid { hand, amount }
    }
    fn cmp_part_1(&self, other: &Self) -> Ordering {
        self.hand.cmp_part_1(&other.hand)
    }
}

fn counts<T>(elements: &Vec<T>) -> HashMap<T, u16>
where T: Eq, T: Hash, T: Copy {
    let mut counts = HashMap::new();
    for &element in elements.iter() {
        let count = counts.entry(element).or_insert(0);
        *count += 1;
    }
    counts
}

pub fn determine_hand_type_part_1(cards: &[Card; 5]) -> HandType {
    let card_counts_hash = counts(&cards.to_vec());
    let card_counts: Vec<&u16> = card_counts_hash.values().collect();
    let count_counts = counts(&card_counts);
    let five_counts = *count_counts.get(&5).unwrap_or(&0);
    let four_counts = *count_counts.get(&4).unwrap_or(&0);
    let three_counts = *count_counts.get(&3).unwrap_or(&0);
    let two_counts = *count_counts.get(&2).unwrap_or(&0);
    if five_counts == 1 {
        HandType::FiveOfAKind
    } else if four_counts == 1 {
        HandType::FourOfAKind
    } else if three_counts == 1 && two_counts == 1 {
        HandType::FullHouse
    } else if three_counts == 1 {
        HandType::ThreeOfAKind
    } else if two_counts == 2 {
        HandType::TwoPair
    } else if two_counts == 1 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

pub fn determine_hand_type_part_2(cards: &[Card; 5]) -> HandType {
    let mut card_counts_hash = counts(&cards.to_vec());
    let original_card_counts_hash = card_counts_hash.clone();
    let joker_count = original_card_counts_hash.get(&Card::new('J')).unwrap_or(&0);
    card_counts_hash.remove(&Card::new('J'));
    let max_count = card_counts_hash.iter().max_by(|x, y| x.1.cmp(y.1));
    if let Some((card, count)) = max_count {
        card_counts_hash.insert(*card, count + *joker_count);
    } else {
        card_counts_hash.insert(Card::new('J'), *joker_count);
    }
    let card_counts: Vec<&u16> = card_counts_hash.values().collect();
    let count_counts = counts(&card_counts);

    let five_counts = *count_counts.get(&5).unwrap_or(&0);
    let four_counts = *count_counts.get(&4).unwrap_or(&0);
    let three_counts = *count_counts.get(&3).unwrap_or(&0);
    let two_counts = *count_counts.get(&2).unwrap_or(&0);
    if five_counts == 1 {
        HandType::FiveOfAKind
    } else if four_counts == 1 {
        HandType::FourOfAKind
    } else if three_counts == 1 && two_counts == 1 {
        HandType::FullHouse
    } else if three_counts == 1 {
        HandType::ThreeOfAKind
    } else if two_counts == 2 {
        HandType::TwoPair
    } else if two_counts == 1 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

pub fn parse_hand_cards(cards_input: &str) -> [Card; 5] {
    let cards: Vec<Card> = cards_input.chars().map(|card| Card::new(card)).collect();
    let hand_cards: [Card; 5] = cards.try_into().unwrap();
    hand_cards
}

pub fn parse_hand(cards_input: &str) -> Hand {
    let hand_cards: [Card; 5] = parse_hand_cards(cards_input);
    let hand_type = determine_hand_type_part_1(&hand_cards);
    Hand::new(hand_cards, hand_type)
}

fn parse_line(line: &str) -> Bid {
    let (cards_input, bid_input) = line.split_once(' ').expect("Did not find a single separator :");
    let hand: Hand = parse_hand(cards_input);
    let bid_amount: u16 = bid_input.trim().parse().unwrap();
    Bid::new(hand, bid_amount)
}

pub fn parse(input: &str) -> Vec<Bid> {
    parsing::as_lines(input).iter().map(|line| parse_line(line.trim())).collect()
}

pub fn solution_part_1(input: &Vec<Bid>) -> u64 {
    let mut bids = input.clone();
    bids.sort_by(|x, y| x.cmp_part_1(y));
    (1..=bids.len()).zip(bids).map(|(rank, bid)| (rank as u64) * (bid.amount as u64)).sum()
}

pub fn solution_part_2(input: &Vec<Bid>) -> u64 {
    //TODO: Implement
    1
}