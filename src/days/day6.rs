use crate::days::parsing;

pub const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

#[derive(Debug)]
#[derive(PartialEq)]
pub struct RaceRecord {
    pub total_time: u16,
    pub best_distance: u16
}

impl RaceRecord {
    pub fn new(total_time: u16, best_distance: u16) -> RaceRecord {
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
    let times: Vec<u16> = parsing::parse_numbers(times_input);
    let (_, distances_input) = lines[1].split_once(':').expect("Did not find a single separator :");
    let distances: Vec<u16> = parsing::parse_numbers(distances_input);
    times.iter().zip(distances.iter()).map(|(time, distance)| RaceRecord::new(*time, *distance)).collect()
}

fn number_of_ways_to_win(record: &RaceRecord) -> u16 {
    let mut ways_to_win_count: u16 = 0;
    for wait_time in 1..record.total_time {
        let speed = wait_time;
        let moving_time = record.total_time - wait_time;
        if speed * moving_time > record.best_distance {
            ways_to_win_count = ways_to_win_count + 1;
        }
    }
    ways_to_win_count
}

pub fn solution_part_1(input: &Vec<RaceRecord>) -> u64 {
    let number_of_ways_to_win = input.iter().map(|race_record| number_of_ways_to_win(&race_record) as u64);
    number_of_ways_to_win.fold(1, |acc, element| acc * element)
}

pub fn solution_part_2(input: &Vec<RaceRecord>) -> u64 {
    1
}