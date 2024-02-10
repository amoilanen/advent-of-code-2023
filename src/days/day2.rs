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

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Game {
    pub id: u16,
    pub drawings: Vec<CubeSet>
}

pub fn parse(input: &str) -> Vec<Game> {
    vec![Game {
        id: 1,
        drawings: Vec::new()
    }]
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