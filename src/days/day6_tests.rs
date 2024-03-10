#[cfg(test)]
use crate::days::day6;
#[cfg(test)]
use crate::days::day6::RaceRecord;

#[test]
fn test_parse() {
    let input: &str = "Time:      7  15   30
    Distance:  9  40  200";

    let parsed = day6::parse(input);
    assert_eq!(parsed, vec![
        RaceRecord::new(7, 9),
        RaceRecord::new(15, 40),
        RaceRecord::new(30, 200)
    ])
}