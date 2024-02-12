pub mod days;

pub use crate::days::day1;
pub use crate::days::day2;
pub use crate::days::day3;

fn main() {
    println!("Day 1:");
    let day_1_part_1_input_parsed = day1::parse(day1::INPUT_PART_1);
    println!("{:?}", day1::solution_part_1(&day_1_part_1_input_parsed));
    let day_1_part_2_input_parsed = day1::parse(day1::INPUT_PART_2);
    println!("{:?}", day1::solution_part_2(&day_1_part_2_input_parsed));
    println!("Day 2:");
    let day_2_input_parsed = day2::parse(day2::INPUT);
    println!("{:?}", day2::solution_part_1(&day_2_input_parsed));
    println!("{:?}", day2::solution_part_2(&day_2_input_parsed));
    println!("Day 3:");
    let day_3_input_parsed = day3::parse(day3::INPUT);
    println!("{:?}", day3::solution_part_1(&day_3_input_parsed));
    println!("{:?}", day3::solution_part_2(&day_3_input_parsed));
}