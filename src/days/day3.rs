pub const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Schematic {
    pub rows: Vec<Vec<char>>
}

fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

pub fn parse(input: &str) -> Schematic {
    let rows = input.split_terminator('\n')
        .map(|line| parse_line(line.trim()))
        .collect();
    Schematic { rows }
}

pub fn solution_part_1(parsed_input: &Schematic) -> u16 {
    1
}

pub fn solution_part_2(parsed_input: &Schematic) -> u32 {
    1
}