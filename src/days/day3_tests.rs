#[cfg(test)]
use crate::days::day3;

#[test]
fn test_parse() {
    let input: &str = "12..34
....*.";
    let result = day3::parse(input);
    assert_eq!(result, vec!["abc"])
}