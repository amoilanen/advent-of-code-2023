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
#[derive(Hash)]
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
    ports: [Coord; 2],
    symbol: char
}

const LOOP_HORIZONTAL_BOUNDARIES: [char; 3] = ['|', 'L', 'J'];

impl Connector {
    pub fn new(position: Coord, ports: [Coord; 2], symbol: char) -> Connector {
        let mut sorted_ports = ports.clone();
        sorted_ports.sort_unstable();
        Connector { position, ports: sorted_ports, symbol }
    }
    pub fn is_connected_with(&self, coord: &Coord) -> bool {
        self.ports.contains(coord)
    }
    pub fn get_other_end_than(&self, other_than: &Coord) -> &Coord {
        if &self.ports[0] == other_than {
            &self.ports[1]
        } else {
            &self.ports[0]
        }
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

    fn build_loop_from(&self, from_tile: &Coord, first_edge: &Connector, coordinate_connectors: &HashMap<&Coord, &Connector>) -> Vec<Coord> {
        let mut current_from_coord = from_tile;
        let mut current_edge_coord = &first_edge.position;
        let mut no_loop_found = false;
        let mut found_loop = false;
        let mut partially_built_loop: Vec<Coord> = vec![from_tile.clone()];

        while !no_loop_found && !found_loop {
            if let Some(edge) = coordinate_connectors.get(current_edge_coord) {
                partially_built_loop.push(current_edge_coord.clone());
                if edge.is_connected_with(current_from_coord) {
                    current_edge_coord = edge.get_other_end_than(current_from_coord);
                    current_from_coord = &edge.position;
                    if current_edge_coord == from_tile {
                        found_loop = true;
                    }
                } else {
                    no_loop_found = true;
                }
            } else {
                no_loop_found = true;
            }
        }
        if found_loop {
            partially_built_loop
        } else {
            Vec::new()
        }
    }

    fn cooordinate_to_connector(&self) -> HashMap<&Coord, &Connector> {
        let mut coordinate_connectors: HashMap<&Coord, &Connector> = HashMap::new();
        for connector in self.connectors.iter() {
            coordinate_connectors.insert(&connector.position, &connector);
        }
        coordinate_connectors
    }

    pub fn find_loops(&self, from_tile: &Coord) -> Vec<Vec<Coord>> {
        fn exists_in_a_loop(coord: &Coord, loops: &Vec<Vec<Coord>>) -> bool {
            loops.iter().any(|l| l.iter().any(|tile| tile == coord))
        }
        let coordinate_connectors = self.cooordinate_to_connector();
        let mut loops : Vec<Vec<Coord>> = Vec::new();
        let possible_loop_origins = vec![
            Coord::new(from_tile.x + 1, from_tile.y),
            Coord::new(from_tile.x, from_tile.y + 1),
            Coord::new(from_tile.x - 1, from_tile.y),
            Coord::new(from_tile.x, from_tile.y - 1)
        ];
        for possible_loop_origin in possible_loop_origins.iter() {
            if !exists_in_a_loop(possible_loop_origin, &loops) {
                if let Some(starting_connector) = coordinate_connectors.get(&possible_loop_origin) {
                    let found_loop: Vec<Coord> = self.build_loop_from(from_tile, *starting_connector, &coordinate_connectors);
                    if found_loop.len() > 0 {
                        loops.push(found_loop);
                    }
                }
            }
        }
        loops
    }
}

pub fn count_enclosed_tiles(tile_loop: &Vec<Coord>, landscape: &Landscape) -> u64 {
    let coordinate_to_connector = landscape.cooordinate_to_connector();
    let xs: Vec<i32> = tile_loop.iter().map(|coord| coord.x).collect();
    let ys: Vec<i32> = tile_loop.iter().map(|coord| coord.y).collect();
    let min_x = *xs.iter().min().unwrap_or(&0);
    let max_x = *xs.iter().max().unwrap_or(&0);
    let min_y = *ys.iter().min().unwrap_or(&0);
    let max_y = *ys.iter().max().unwrap_or(&0);
    let mut tile_count: u64 = 0;
    for y in min_y..=max_y {
        let mut is_inside_loop = false;
        for x in min_x..=max_x {
            let tile_symbol = match coordinate_to_connector.get(&Coord::new(x, y)) {
                Some(connector) => connector.symbol,
                None => '.'
            };
            let is_loop_tile = tile_loop.contains(&Coord::new(x, y));
            if is_loop_tile {
                if LOOP_HORIZONTAL_BOUNDARIES.contains(&tile_symbol) {
                    is_inside_loop = !is_inside_loop;
                }
            } else {
                if is_inside_loop {
                    tile_count = tile_count + 1;
                }
            }
        }
    };
    tile_count
}

pub fn parse_connector(x: i32, y: i32, ch: char) -> Result<Connector,  Box<dyn Error>> {
    match ch {
        '|' => Ok(Connector::new(Coord::new(x, y), [Coord::new(x, y - 1), Coord::new(x, y + 1)], ch)),
        '-' => Ok(Connector::new(Coord::new(x, y), [Coord::new(x - 1, y), Coord::new(x + 1, y)], ch)),
        'L' => Ok(Connector::new(Coord::new(x, y), [Coord::new(x, y - 1), Coord::new(x + 1, y)], ch)),
        'J' => Ok(Connector::new(Coord::new(x, y), [Coord::new(x, y - 1), Coord::new(x - 1, y)], ch)),
        '7' => Ok(Connector::new(Coord::new(x, y), [Coord::new(x, y + 1), Coord::new(x - 1, y)], ch)),
        'F' => Ok(Connector::new(Coord::new(x, y), [Coord::new(x, y + 1), Coord::new(x + 1, y)], ch)),
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
    let loops = input.find_loops(&input.starting_title);
    let farthest_distances: Vec<u64> = loops.iter().map(|l| ((l.len() + 1) / 2) as u64).collect();
    *farthest_distances.first().unwrap()
}

pub fn solution_part_2(input: &Landscape) -> u64 {
    1
}