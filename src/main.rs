pub mod days;

pub use crate::days::day1;

fn main() {
    println!("Day 1:");
    let day_1_input_parsed = day1::parse(day1::INPUT);
    println!("{:?}", day1::solution_part_1(day_1_input_parsed));
    println!("{:?}", day1::solution_part_2(day_1_input_parsed));
}