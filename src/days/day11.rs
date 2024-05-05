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
        self
          .expand_dimension(
            |g| g.coord.x,
            |g, updated_x| Galaxy::new(updated_x, g.coord.y)
          )
          .expand_dimension(
            |g| g.coord.y,
            |g, updated_y| Galaxy::new(g.coord.x, updated_y)
          )
    }

    fn expand_dimension<F1, F2>(self, get_coord: F1, update_coord: F2) -> Universe
    where
      F1: Fn(&Galaxy) -> u32,
      F2: Fn(&Galaxy, u32) -> Galaxy
    {
        let mut coord_increments: HashMap<&Galaxy, u32> = HashMap::new();
        let distinct_coords: Vec<u32> = collections::unique(&self.galaxies.clone().iter().map(|g| get_coord(&g)).collect());
        for galaxy in self.galaxies.iter() {
            let mut empty_places_before = get_coord(&galaxy);
            for other_galaxy_coord in distinct_coords.iter() {
                if other_galaxy_coord < &get_coord(&galaxy) {
                    empty_places_before = empty_places_before - 1;
                }
            }
            coord_increments.insert(galaxy, empty_places_before);
        }
        let mut updated_galaxies: Vec<Galaxy> = Vec::new();
        for galaxy in self.galaxies.iter() {
            let updated_coord = get_coord(&galaxy) + coord_increments.get(&galaxy).unwrap_or(&0);
            let updated_galaxy: Galaxy = update_coord(&galaxy, updated_coord);
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