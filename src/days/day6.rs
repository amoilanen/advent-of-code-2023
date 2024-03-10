use crate::days::parsing;

pub const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

#[derive(Debug)]
#[derive(PartialEq)]
pub struct RaceRecord {
    pub total_time: u64,
    pub best_distance: u64
}

impl RaceRecord {
    pub fn new(total_time: u64, best_distance: u64) -> RaceRecord {
        RaceRecord { total_time, best_distance }
    }
}

pub fn parse(input: &str) -> Vec<RaceRecord> {
    let lines = parsing::as_lines(input);
    if lines.len() < 2 {
        println!("Expected to parse two lines of input, but found {:?}", lines);
        return Vec::new()
    }
    let (_, times_input) = lines[0].split_once(':').expect("Did not find a single separator :");
    let times: Vec<u64> = parsing::parse_numbers(times_input);
    let (_, distances_input) = lines[1].split_once(':').expect("Did not find a single separator :");
    let distances: Vec<u64> = parsing::parse_numbers(distances_input);
    times.iter().zip(distances.iter()).map(|(time, distance)| RaceRecord::new(*time, *distance)).collect()
}

pub fn number_of_ways_to_win(record: &RaceRecord) -> u64 {
    let mut ways_to_win_count: u64 = 0;
    for wait_time in 1..record.total_time {
        let speed = wait_time;
        let moving_time = record.total_time - wait_time;
        if speed * moving_time > record.best_distance {
            ways_to_win_count = ways_to_win_count + 1;
        }
    }
    ways_to_win_count
}

fn floor(x: f64) -> f64 {
  x.trunc()
}

fn ceil(num: f64) -> f64 {
  let floored = floor(num);
  if num > floored {
    floored + 1.0
  } else {
    floored
  }
}

pub fn smart_number_of_ways_to_win(record: &RaceRecord) -> u64 {
    let t = record.total_time as f64 / 2.0;
    let d = record.best_distance as f64;
    let discriminant_squared = t * t - d;
    if discriminant_squared <= 0.0 {
        0
    } else {
        let discriminant: f64 = (discriminant_squared as f64).sqrt();
        let left_root = t as f64 - discriminant;
        let mut smallest_time = ceil(left_root);
        if smallest_time == left_root {
            smallest_time = smallest_time + 1.0;
        }
        if smallest_time < 0.0 {
            smallest_time = 0.0;
        }
        let right_root = t as f64 + discriminant;
        let mut largest_time = floor(right_root);
        if largest_time == right_root {
            largest_time = largest_time - 1.0;
        }
        (largest_time - smallest_time + 1.0) as u64
    }
}

pub fn solution_part_1(input: &Vec<RaceRecord>) -> u64 {
    let number_of_ways_to_win: Vec<u64> = input.iter().map(|race_record| smart_number_of_ways_to_win(&race_record) as u64).collect();
    println!("Number of ways to win = {:?}", number_of_ways_to_win);
    number_of_ways_to_win.iter().fold(1, |acc, element| acc * element)
}

pub fn solution_part_2(input: &Vec<RaceRecord>) -> u64 {
    1
}