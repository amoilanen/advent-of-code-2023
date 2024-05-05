use crate::days::parsing;
use crate::days::collections;
use std::collections::HashMap;

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

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Coord {
    x: u32,
    y: u32
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Galaxy {
    pub coord: Coord
}

impl Galaxy {
    pub fn new(x: u32, y: u32) -> Galaxy {
        Galaxy { coord: Coord { x, y } }
    }
}

#[derive(Clone, Debug, Hash, PartialEq)]
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
        let mut x_increments: HashMap<&Galaxy, u32> = HashMap::new();
        let mut y_increments: HashMap<&Galaxy, u32> = HashMap::new();
        let distinct_xs: Vec<u32> = collections::unique(&self.galaxies.clone().iter().map(|g| g.coord.x).collect());
        let distinct_ys: Vec<u32> = collections::unique(&self.galaxies.clone().iter().map(|g| g.coord.y).collect());
        for galaxy in self.galaxies.iter() {
            let mut empty_columns_before = galaxy.coord.x;
            let mut empty_rows_before = galaxy.coord.y;
            for other_galaxy_x in distinct_xs.iter() {
                if other_galaxy_x < &galaxy.coord.x {
                    empty_columns_before = empty_columns_before - 1;
                }
            }
            for other_galaxy_y in distinct_ys.iter() {
                if other_galaxy_y < &galaxy.coord.y {
                    empty_rows_before = empty_rows_before - 1;
                }
            }
            x_increments.insert(galaxy, empty_columns_before);
            y_increments.insert(galaxy, empty_rows_before);
        }
        let mut updated_galaxies: Vec<Galaxy> = Vec::new();
        for galaxy in self.galaxies.iter() {
            let updated_x = galaxy.coord.x + x_increments.get(&galaxy).unwrap_or(&0);
            let updated_y = galaxy.coord.y + y_increments.get(&galaxy).unwrap_or(&0);
            let updated_galaxy: Galaxy = Galaxy::new(updated_x, updated_y);
            updated_galaxies.push(updated_galaxy);
        }
        Universe::new(updated_galaxies)
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