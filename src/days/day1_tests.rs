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

#[cfg(test)]
fn test_line_number(input: &str, expected: u32) {
  let digits_to_value = day1::digits_to_value();
  let digit_patterns = day1::digit_patterns(&digits_to_value);
  assert_eq!(day1::line_to_number(input, &digits_to_value, &digit_patterns), expected)
}

#[test]
fn test_line_to_number_multiple_digits() {
  test_line_number("a1b2c3d4e5f", 15)
}

#[test]
fn test_line_to_number_exactly_two_digits() {
  test_line_number("pqr3stu8vwx", 38)
}

#[test]
fn test_line_to_number_a_single_digit() {
  test_line_number("treb7uchet", 77)
}

#[test]
fn test_line_to_number_no_digits() {
  test_line_number("abc", 0)
}

#[test]
fn test_line_to_number_two_digits_as_words() {
  test_line_number("two1nine", 29)
}

#[test]
fn test_line_to_number_two_digits_as_words_some_words_intersect() {
  test_line_number("eightwothree", 83)
}

#[test]
fn test_solution_part1() {
  let parsed = day1::parse(day1::INPUT_PART_1);
  assert_eq!(day1::solution_part_1(&parsed), 142)
}

#[test]
fn test_solution_part2() {
  let parsed = day1::parse(day1::INPUT_PART_2);
  assert_eq!(day1::solution_part_2(&parsed), 281)
}