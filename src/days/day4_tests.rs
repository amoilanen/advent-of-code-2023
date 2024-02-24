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
            vec![41, 48, 83, 86, 17],
            vec![83, 86, 6, 31, 17, 9, 48, 53]
        ),
        Card::new(
            vec![13, 32, 20, 16, 61],
            vec![61, 30, 68, 82, 17, 32, 24, 19]
        ),
        Card::new(
            vec![1, 21, 53, 59, 44],
            vec![69, 82, 63, 72, 16, 21, 14, 1]
        )
    ])
}