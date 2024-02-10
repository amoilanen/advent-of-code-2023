use std::collections::HashMap;

pub const INPUT_PART: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

#[derive(PartialEq)]
#[derive(Debug)]
pub struct CubeSet {
    pub red: u16,
    pub green: u16,
    pub blue: u16
}

impl CubeSet {
    pub fn new(red: u16, green: u16, blue: u16) -> Self {
        CubeSet {
            red, green, blue
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Game {
    pub id: u16,
    pub drawings: Vec<CubeSet>
}

impl Game {
    pub fn new(id: u16, drawings: Vec<CubeSet>) -> Self {
        Game { id, drawings }
    }
}

fn parse_drawing(input: &str) -> CubeSet {
    fn find_color_count(cube_counts: &Vec<(u16, &str)>, color_to_find: &str) -> u16 {
        cube_counts.iter().find_map(|(count, color)|
            if *color == color_to_find { Some(*count) } else { None }
        ).unwrap_or(0)
    }
    let cube_counts: Vec<(u16, &str)> = input.split(",").map(|count_input| {
        let parts: Vec<&str> = count_input.trim().split(' ').take(2).collect::<Vec<&str>>();
        let (count, color) = (u16::from_str_radix(parts[0], 10).unwrap(), parts[1]);
        (count, color)
    }).collect();
    let red_count = find_color_count(&cube_counts, "red");
    let green_count = find_color_count(&cube_counts, "green");
    let blue_count = find_color_count(&cube_counts, "blue");
    CubeSet::new(red_count, green_count, blue_count)
}

fn parse_line(line: &str) -> Game {
    let split_line = line.split(':').take(2).collect::<Vec<&str>>();
    let (game_input, drawings_input) = (split_line[0], split_line[1]);
    let game_id = u16::from_str_radix(&game_input["Game ".len()..], 10).unwrap();
    let drawings: Vec<CubeSet> = drawings_input.split(';').map(|drawing_input| parse_drawing(drawing_input)).collect();
    Game {
        id: game_id,
        drawings: drawings
    }
}

pub fn parse(input: &str) -> Vec<Game> {
    input.split_terminator('\n')
        .map(|line| parse_line(line))
        .collect()
}

pub fn solution_part_1(parsed_input: &Vec<Game>) -> u16 {
    let total_cubes = CubeSet {
        red: 12,
        green: 13,
        blue: 14
    };
    //TODO: Implement
    return 1;
}

pub fn solution_part_2(parsed_input: &Vec<&str>) -> u32 {
    1
}