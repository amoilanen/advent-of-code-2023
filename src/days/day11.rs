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
    x: u64,
    y: u64
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Galaxy {
    pub coord: Coord
}

impl Galaxy {
    pub fn new(x: u64, y: u64) -> Galaxy {
        Galaxy { coord: Coord { x, y } }
    }
    pub fn distance_to(&self, other: &Galaxy) -> u64 {
        self.coord.x.abs_diff(other.coord.x) + self.coord.y.abs_diff(other.coord.y)
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

    pub fn expand(self, expansion_step: u64) -> Universe {
        self
          .expand_dimension(
            |g| g.coord.x,
            |g, updated_x| Galaxy::new(updated_x, g.coord.y),
            expansion_step
          )
          .expand_dimension(
            |g| g.coord.y,
            |g, updated_y| Galaxy::new(g.coord.x, updated_y),
            expansion_step
          )
    }

    fn expand_dimension<F1, F2>(self, get_coord: F1, update_coord: F2, expansion_step: u64) -> Universe
    where
      F1: Fn(&Galaxy) -> u64,
      F2: Fn(&Galaxy, u64) -> Galaxy
    {
        let mut coord_increments: HashMap<&Galaxy, u64> = HashMap::new();
        let distinct_coords: Vec<u64> = collections::unique(&self.galaxies.clone().iter().map(|g| get_coord(&g)).collect());
        for galaxy in self.galaxies.iter() {
            let mut empty_places_before = get_coord(&galaxy);
            for other_galaxy_coord in distinct_coords.iter() {
                if other_galaxy_coord < &get_coord(&galaxy) {
                    empty_places_before = empty_places_before - 1;
                }
            }
            coord_increments.insert(galaxy, empty_places_before * (expansion_step - 1));
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
                '#' => galaxies.push(Galaxy::new(x as u64, y as u64)),
                _ => continue
            }
        }
    }
    Universe::new(galaxies)
}

pub fn sum_of_all_distances_in_expanded_universe(universe: &Universe, expansion_step: u64) -> u64 {
    let expanded_universe: Universe = (*universe).clone().expand(expansion_step);
    let mut full_distance: u64 = 0;
    for first in expanded_universe.galaxies.iter() {
        for second in expanded_universe.galaxies.iter() {
            full_distance = full_distance + first.distance_to(second) as u64
        }
    }
    full_distance / 2
}

pub fn solution_part_1(universe: &Universe) -> u64 {
    sum_of_all_distances_in_expanded_universe(universe, 2)
}

pub fn solution_part_2(universe: &Universe) -> u64 {
    sum_of_all_distances_in_expanded_universe(universe, 1000000)
}