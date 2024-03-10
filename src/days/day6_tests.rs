use crate::day6::smart_number_of_ways_to_win;
#[cfg(test)]
use crate::days::day6;
#[cfg(test)]
use crate::days::day6::RaceRecord;

#[test]
fn test_parse() {
    let input: &str = "Time:      7  15   30
    Distance:  9  40  200";

    let parsed = day6::parse(input);
    assert_eq!(parsed, vec![
        RaceRecord::new(7, 9),
        RaceRecord::new(15, 40),
        RaceRecord::new(30, 200)
    ])
}

#[test]
fn test_smart_number_of_ways_to_win() {
    assert_eq!(day6::smart_number_of_ways_to_win(&RaceRecord::new(7, 9)), 4);
    assert_eq!(day6::smart_number_of_ways_to_win(&RaceRecord::new(15, 40)), 8);
    assert_eq!(day6::smart_number_of_ways_to_win(&RaceRecord::new(30, 200)), 9);
}


#[test]
fn test_solution_part1() {
    let parsed = day6::parse(day6::INPUT);
    assert_eq!(day6::solution_part_1(&parsed), 288)
}