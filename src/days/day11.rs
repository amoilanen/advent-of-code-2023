use crate::days::parsing;

pub const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Coord {
    x: u32,
    y: u32
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Galaxy {
    pub coord: Coord
}

impl Galaxy {
    pub fn new(x: u32, y: u32) -> Galaxy {
        Galaxy { coord: Coord { x, y } }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Universe {
    pub galaxies: Vec<Galaxy>
}

impl Universe {
    pub fn new(galaxies: Vec<Galaxy>) -> Universe {
        Universe { galaxies }
    }
}

impl Universe {
    pub fn expand(self) -> Universe {
        //TODO: Implement
        self
    }
}

pub fn parse(input: &str) -> Universe  {
    let mut galaxies: Vec<Galaxy> = Vec::new();
    for (y, line) in parsing::as_lines(input).iter().enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            match symbol {
                '#' => galaxies.push(Galaxy::new(x as u32, y as u32)),
                _ => continue
            }
        }
    }
    Universe::new(galaxies)
}

pub fn solution_part_1(input: &Universe) -> u64 {
    1
}

pub fn solution_part_2(input: &Universe) -> u64 {
    1
}