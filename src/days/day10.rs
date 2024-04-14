use std::collections::HashMap;
use crate::days::parsing;

pub const INPUT: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Coord {
    pub x: u16,
    pub y: u16
}

impl Coord {
    pub fn new(x: u16, y: u16) -> Coord {
        Coord { x, y }
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
        Connector { position, ports }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Landscape {
    starting_title: Coord,
    connectors: Vec<Connector>
}

impl Landscape {
    pub fn new(starting_title: Coord, connectors: Vec<Connector>) -> Landscape {
        Landscape { starting_title, connectors }
    }
}

pub fn parse_connector(coord: Coord, ch: char) -> Connector {
    //TODO: Implement
    Connector::new(Coord::new(0, 0), [Coord::new(0, 0), Coord::new(0, 0)])
}

pub fn parse(input: &str) -> Landscape {
    //TODO: Implement
    Landscape::new(Coord::new(0, 0), Vec::new())
}

pub fn solution_part_1(input: &Landscape) -> u64 {
    1
}

pub fn solution_part_2(input: &Landscape) -> u64 {
    1
}