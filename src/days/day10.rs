use std::collections::HashMap;
use crate::days::parsing;
use std::error::Error;
use std::cmp::Ordering;
use parsing::ParsingError;

pub const INPUT: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(PartialOrd)]
pub struct Coord {
    pub x: i32,
    pub y: i32
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
      if self.x != other.x {
        self.x.cmp(&other.x)
      } else {
        self.y.cmp(&other.y)
      }
    }
  }

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Connector {
    position: Coord,
    ports: [Coord; 2]
}

impl Connector {
    pub fn new(position: Coord, ports: [Coord; 2]) -> Connector {
        let mut sorted_ports = ports.clone();
        sorted_ports.sort_unstable();
        Connector { position, ports: sorted_ports }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Landscape {
    pub starting_title: Coord,
    pub connectors: Vec<Connector>
}

impl Landscape {
    pub fn new(starting_title: Coord, connectors: Vec<Connector>) -> Landscape {
        Landscape { starting_title, connectors }
    }
}

pub fn parse_connector(x: i32, y: i32, ch: char) -> Result<Connector,  Box<dyn Error>> {
    match ch {
        '|' => Ok(Connector::new(Coord::new(x, y), [Coord::new(x, y - 1), Coord::new(x, y + 1)])),
        '-' => Ok(Connector::new(Coord::new(x, y), [Coord::new(x - 1, y), Coord::new(x + 1, y)])),
        'L' => Ok(Connector::new(Coord::new(x, y), [Coord::new(x, y - 1), Coord::new(x + 1, y)])),
        'J' => Ok(Connector::new(Coord::new(x, y), [Coord::new(x, y - 1), Coord::new(x - 1, y)])),
        '7' => Ok(Connector::new(Coord::new(x, y), [Coord::new(x, y + 1), Coord::new(x - 1, y)])),
        'F' => Ok(Connector::new(Coord::new(x, y), [Coord::new(x, y + 1), Coord::new(x + 1, y)])),
        _ => Err(ParsingError::raise(format!("Unknown connector in {}", ch))),

    }
}

pub fn parse(input: &str) -> Result<Landscape, Box<dyn Error>>  {
    let mut connectors: Vec<Connector> = Vec::new();
    let mut found_starting_tile: Option<Coord> = None;
    for (y, line) in parsing::as_lines(input).iter().enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            match symbol {
                '.' => continue,
                'S' => found_starting_tile = Some(Coord::new(x as i32, y as i32)),
                _ => connectors.push(parse_connector(x as i32, y as i32, symbol)?)
            }
        }
    }
    let starting_tile = found_starting_tile.ok_or(ParsingError::raise(format!("Did not find a starting tile in {}", input)))?;
    Ok(Landscape::new(starting_tile, connectors))
}

pub fn solution_part_1(input: &Landscape) -> u64 {
    1
}

pub fn solution_part_2(input: &Landscape) -> u64 {
    1
}