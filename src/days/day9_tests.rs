#[cfg(test)]
use crate::days::day9;
#[cfg(test)]
use crate::days::day9::ValueHistory;

#[test]
fn test_parse_input() {
    let input: &str = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";

    let parsed = day9::parse(input);
    assert_eq!(parsed, vec![
        ValueHistory::new(vec![0, 3, 6, 9, 12, 15]),
        ValueHistory::new(vec![1, 3, 6, 10, 15, 21]),
        ValueHistory::new(vec![10, 13, 16, 21, 30, 45])
    ])
}