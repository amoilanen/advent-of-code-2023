#[cfg(test)]
use crate::days::day4;
#[cfg(test)]
use crate::days::day4::Card;

#[test]
fn test_parse() {
    let input: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
    let result = day4::parse(input);
    assert_eq!(result, vec![
        Card::new(
            1,
            vec![41, 48, 83, 86, 17],
            vec![83, 86, 6, 31, 17, 9, 48, 53]
        ),
        Card::new(
            2,
            vec![13, 32, 20, 16, 61],
            vec![61, 30, 68, 82, 17, 32, 24, 19]
        ),
        Card::new(
            3,
            vec![1, 21, 53, 59, 44],
            vec![69, 82, 63, 72, 16, 21, 14, 1]
        )
    ])
}

#[test]
fn test_card_points_no_matches() {
    let card = Card::new(
        1,
        vec![87, 83, 26, 28, 32],
        vec![88, 30, 70, 12, 93, 22, 82, 36]
    );
    assert_eq![0, card.points()]
}

#[test]
fn test_card_points_one_match() {
    let card = Card::new(
        1,
        vec![41, 92, 73, 84, 69],
        vec![59, 84, 76, 51, 58, 5, 54, 83]
    );
    assert_eq![1, card.points()]
}

#[test]
fn test_card_points_three_matches() {
    let card = Card::new(
        1,
        vec![41, 48, 83, 86, 17],
        vec![83, 86, 6, 31, 17, 9, 48, 53]
    );
    assert_eq![8, card.points()]
}

#[test]
fn test_solution_part1() {
    let parsed = day4::parse(day4::INPUT);
    assert_eq!(day4::solution_part_1(&parsed), 13)
}

#[test]
fn test_solution_part2() {
    let parsed = day4::parse(day4::INPUT);
    assert_eq!(day4::solution_part_2(&parsed), 30)
}