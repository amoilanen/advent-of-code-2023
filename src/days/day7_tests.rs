#[cfg(test)]
use crate::days::day7;
#[cfg(test)]
use crate::days::day7::{ Bid, Card, Hand, HandType };
#[cfg(test)]
use std::cmp::Ordering;

#[test]
fn test_parse_input() {
    let input: &str = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";

    let parsed = day7::parse(input);
    assert_eq!(parsed, vec![
        Bid::new(Hand::new([ Card::new('3'), Card::new('2'), Card::new('T'), Card::new('3'), Card::new('K') ], HandType::OnePair), 765),
        Bid::new(Hand::new([ Card::new('T'), Card::new('5'), Card::new('5'), Card::new('J'), Card::new('5') ], HandType::ThreeOfAKind), 684),
        Bid::new(Hand::new([ Card::new('K'), Card::new('K'), Card::new('6'), Card::new('7'), Card::new('7') ], HandType::TwoPair), 28),
        Bid::new(Hand::new([ Card::new('K'), Card::new('T'), Card::new('J'), Card::new('J'), Card::new('T') ], HandType::TwoPair), 220),
        Bid::new(Hand::new([ Card::new('Q'), Card::new('Q'), Card::new('Q'), Card::new('J'), Card::new('A') ], HandType::ThreeOfAKind), 483)
    ])
}

#[test]
fn test_compare_hands() {
    assert_eq!(
        day7::parse_hand("T55J5").cmp(&day7::parse_hand("QQQJA")),
        Ordering::Less
    );
    assert_eq!(
        day7::parse_hand("32T3K").cmp(&day7::parse_hand("KK677")),
        Ordering::Less
    );
    assert_eq!(
        day7::parse_hand("KK677").cmp(&day7::parse_hand("32T3K")),
        Ordering::Greater
    );
    assert_eq!(
        day7::parse_hand("KTJJT").cmp(&day7::parse_hand("KTJJT")),
        Ordering::Equal
    );
    assert_eq!(
        day7::parse_hand("KK677").cmp(&day7::parse_hand("KTJJT")),
        Ordering::Greater
    );
    assert_eq!(
        day7::parse_hand("KTJJT").cmp(&day7::parse_hand("KK677")),
        Ordering::Less
    );
}

#[test]
fn test_sorting_of_hands() {
    let mut hands: Vec<Hand> = ["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA"]
        .iter().map(|hand_input| day7::parse_hand(hand_input)).collect();
    hands.sort_by(|x, y| x.cmp(y));
    let result: Vec<String> = hands.iter().map(|hand| hand.as_string()).collect();
    assert_eq!(result, ["32T3K", "KTJJT", "KK677", "T55J5", "QQQJA"])
}

#[test]
fn test_solution_part_1() {
    let parsed = day7::parse(day7::INPUT);
    assert_eq!(day7::solution_part_1(&parsed), 6440)
}