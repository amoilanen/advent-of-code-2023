#[cfg(test)]
use crate::days::day7;
#[cfg(test)]
use crate::days::day7::{ Bid, Card, Hand, HandType, TaskPart };
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
        Bid::new(Hand::new([ Card::new('3'), Card::new('2'), Card::new('T'), Card::new('3'), Card::new('K') ], HandType::OnePair, HandType::OnePair), 765),
        Bid::new(Hand::new([ Card::new('T'), Card::new('5'), Card::new('5'), Card::new('J'), Card::new('5') ], HandType::ThreeOfAKind, HandType::FourOfAKind), 684),
        Bid::new(Hand::new([ Card::new('K'), Card::new('K'), Card::new('6'), Card::new('7'), Card::new('7') ], HandType::TwoPair, HandType::TwoPair), 28),
        Bid::new(Hand::new([ Card::new('K'), Card::new('T'), Card::new('J'), Card::new('J'), Card::new('T') ], HandType::TwoPair, HandType::FourOfAKind), 220),
        Bid::new(Hand::new([ Card::new('Q'), Card::new('Q'), Card::new('Q'), Card::new('J'), Card::new('A') ], HandType::ThreeOfAKind, HandType::FourOfAKind), 483)
    ])
}

#[test]
fn test_compare_hands() {
    assert_eq!(
        day7::parse_hand("T55J5").cmp(&day7::parse_hand("QQQJA"), &TaskPart::One),
        Ordering::Less
    );
    assert_eq!(
        day7::parse_hand("32T3K").cmp(&day7::parse_hand("KK677"), &TaskPart::One),
        Ordering::Less
    );
    assert_eq!(
        day7::parse_hand("KK677").cmp(&day7::parse_hand("32T3K"), &TaskPart::One),
        Ordering::Greater
    );
    assert_eq!(
        day7::parse_hand("KTJJT").cmp(&day7::parse_hand("KTJJT"), &TaskPart::One),
        Ordering::Equal
    );
    assert_eq!(
        day7::parse_hand("KK677").cmp(&day7::parse_hand("KTJJT"), &TaskPart::One),
        Ordering::Greater
    );
    assert_eq!(
        day7::parse_hand("KTJJT").cmp(&day7::parse_hand("KK677"), &TaskPart::One),
        Ordering::Less
    );
}

#[test]
fn test_determine_hand_type_for_part_1() {
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("AAAAA"), &TaskPart::One),
        HandType::FiveOfAKind
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("AA8AA"), &TaskPart::One),
        HandType::FourOfAKind
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("23332"), &TaskPart::One),
        HandType::FullHouse
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("TTT98"), &TaskPart::One),
        HandType::ThreeOfAKind
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("23432"), &TaskPart::One),
        HandType::TwoPair
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("A23A4"), &TaskPart::One),
        HandType::OnePair
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("23456"), &TaskPart::One),
        HandType::HighCard
    );
}

#[test]
fn test_determine_hand_type_for_part_2() {
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("AAAAA"), &TaskPart::Two),
        HandType::FiveOfAKind
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("AA8AA"), &TaskPart::Two),
        HandType::FourOfAKind
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("23332"), &TaskPart::Two),
        HandType::FullHouse
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("TTT98"), &TaskPart::Two),
        HandType::ThreeOfAKind
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("23432"), &TaskPart::Two),
        HandType::TwoPair
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("A23A4"), &TaskPart::Two),
        HandType::OnePair
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("23456"), &TaskPart::Two),
        HandType::HighCard
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("AJAJA"), &TaskPart::Two),
        HandType::FiveOfAKind
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("AA8JA"), &TaskPart::Two),
        HandType::FourOfAKind
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("J33J2"), &TaskPart::Two),
        HandType::FourOfAKind
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("233J2"), &TaskPart::Two),
        HandType::FullHouse
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("TJT98"), &TaskPart::Two),
        HandType::ThreeOfAKind
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("TJJ98"), &TaskPart::Two),
        HandType::ThreeOfAKind
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("234J2"), &TaskPart::Two),
        HandType::ThreeOfAKind
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("A23J4"), &TaskPart::Two),
        HandType::OnePair
    );
    assert_eq!(
        day7::determine_hand_type_for_part(&day7::parse_hand_cards("JJJJJ"), &TaskPart::Two),
        HandType::FiveOfAKind
    );
}

#[test]
fn test_sorting_of_hands_part_1() {
    let mut hands: Vec<Hand> = ["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA"]
        .iter().map(|hand_input| day7::parse_hand(hand_input)).collect();
    hands.sort_by(|x, y| x.cmp(y, &TaskPart::One));
    let result: Vec<String> = hands.iter().map(|hand| hand.as_string()).collect();
    assert_eq!(result, ["32T3K", "KTJJT", "KK677", "T55J5", "QQQJA"])
}

#[test]
fn test_sorting_of_hands_part_2() {
    let mut hands: Vec<Hand> = ["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA"]
        .iter().map(|hand_input| day7::parse_hand(hand_input)).collect();
    hands.sort_by(|x, y| x.cmp(y, &TaskPart::Two));
    let result: Vec<String> = hands.iter().map(|hand| hand.as_string()).collect();
    assert_eq!(result, ["32T3K", "KK677", "T55J5", "QQQJA", "KTJJT"])
}

#[test]
fn test_solution_part_1() {
    let parsed = day7::parse(day7::INPUT);
    assert_eq!(day7::solution_part_1(&parsed), 6440)
}

#[test]
fn test_solution_part_2() {
    let parsed = day7::parse(day7::INPUT);
    assert_eq!(day7::solution_part_2(&parsed), 5905)
}