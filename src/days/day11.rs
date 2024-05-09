use crate::days::parsing;
use crate::days::collections;
use std::collections::HashMap;

use super::collections::pairs_of;

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
    pub fn distance_to(self, other: &Galaxy) -> u32 {
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

pub fn compute_coefficients(galaxy_number: u32) -> Vec<u32> {
    if galaxy_number == 2 {
        return vec![1]
    } else if galaxy_number > 2 {
        let previous_coefficients = compute_coefficients(galaxy_number - 2);
        let mut coefficients: Vec<u32> = Vec::new();
        coefficients.push(galaxy_number - 1);
        for coefficient in previous_coefficients.iter() {
            coefficients.push(coefficient + galaxy_number - 1)
        }
        coefficients.push(galaxy_number - 1);
        return coefficients
    } else {
        return Vec::new()
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

pub fn solution_part_1(universe: &Universe) -> u64 {
    let expanded_universe: Universe = (*universe).clone().expand();
    let coefficients = compute_coefficients(expanded_universe.galaxies.len() as u32);
    let mut galaxy_xs: Vec<u32> = expanded_universe.galaxies.iter().map(|g| g.coord.x).collect();
    galaxy_xs.sort();
    let galaxy_x_intervals: Vec<u32> = pairs_of(&galaxy_xs).iter().map(|(current, next)| next - current).collect();
    let mut galaxy_ys: Vec<u32> = expanded_universe.galaxies.iter().map(|g| g.coord.x).collect();
    galaxy_ys.sort();
    let galaxy_y_intervals: Vec<u32> = pairs_of(&galaxy_ys).iter().map(|(current, next)| next - current).collect();
    let x_distance: u32 = galaxy_x_intervals.iter().zip(coefficients.iter()).map(|(distance, coefficient)| distance * coefficient).sum();
    let y_distance: u32 = galaxy_y_intervals.iter().zip(coefficients.iter()).map(|(distance, coefficient)| distance * coefficient).sum();
    (x_distance + y_distance) as u64
}

pub fn solution_part_2(input: Universe) -> u64 {
    1
}