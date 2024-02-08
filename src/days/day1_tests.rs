#[cfg(test)]
use crate::days::day1;

#[test]
fn parse_input() {
    let input: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let result = day1::parse(input);
    assert_eq!(&result, &[
      "1abc2",
      "pqr3stu8vwx",
      "a1b2c3d4e5f",
      "treb7uchet"
    ].to_vec())
}