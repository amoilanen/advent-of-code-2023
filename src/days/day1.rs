pub const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

pub fn parse(input: &str) -> Vec<&str> {
    input.split_terminator('\n').collect()
}

pub fn line_to_number(line: &str) -> u32 {
    let mut digits = line.chars().filter(|ch| ch.is_ascii_digit()).peekable();
    let first = digits.next();
    let last = digits.last().or_else(|| first);
    let number: u32 = vec![first, last].iter().flatten().fold(0, |acc, ch| acc * 10 + (*ch as u32 - '0' as u32) );
    number
}

pub fn solution_part_1(parsed_input: &Vec<&str>) -> u32 {
    let numbers = parsed_input.iter().map(|line|
        line_to_number(line)
    );
    numbers.sum()
}

pub fn solution_part_2(parsed_input: &Vec<&str>) -> u32 {
    return 1;
}