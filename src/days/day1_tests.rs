#[cfg(test)]
use crate::days::day1;

#[test]
fn test_parse_input() {
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

#[test]
fn test_line_to_number_multiple_digits() {
  assert_eq!(day1::line_to_number("a1b2c3d4e5f"), 15)
}

#[test]
fn test_line_to_number_exactly_two_digits() {
  assert_eq!(day1::line_to_number("pqr3stu8vwx"), 38)
}

#[test]
fn test_line_to_number_a_single_digit() {
  assert_eq!(day1::line_to_number("treb7uchet"), 77)
}

#[test]
fn test_line_to_number_no_digits() {
  assert_eq!(day1::line_to_number("abc"), 0)
}

#[test]
fn test_solution_part1() {
  let parsed = day1::parse(day1::INPUT);
  assert_eq!(day1::solution_part_1(&parsed), 142)
}