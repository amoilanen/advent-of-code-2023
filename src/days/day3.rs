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


fn parse_line(line: &str) -> &str {
    "todo"
}

pub fn parse(input: &str) -> Vec<&str> {
    input.split_terminator('\n')
        .map(|line| parse_line(line.trim()))
        .collect()
}

pub fn solution_part_1(parsed_input: &Vec<&str>) -> u16 {
    1
}

pub fn solution_part_2(parsed_input: &Vec<&str>) -> u32 {
    1
}