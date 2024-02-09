pub mod days;

pub use crate::days::day1;

fn main() {
    println!("Day 1:");
    let day_1_part_1_input_parsed = day1::parse(day1::INPUT_PART_1);
    println!("{:?}", day1::solution_part_1(&day_1_part_1_input_parsed));
    let day_1_part_2_input_parsed = day1::parse(day1::INPUT_PART_2);
    println!("{:?}", day1::solution_part_2(&day_1_part_2_input_parsed));
}