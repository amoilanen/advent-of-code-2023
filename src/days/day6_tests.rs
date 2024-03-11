#[cfg(test)]
use crate::days::day6;
#[cfg(test)]
use crate::days::day6::RaceRecord;

#[test]
fn test_parse_input_for_part_1() {
    let input: &str = "Time:      7  15   30
    Distance:  9  40  200";

    let parsed = day6::parse_input_for_part_1(input).unwrap();
    assert_eq!(parsed, vec![
        RaceRecord::new(7, 9),
        RaceRecord::new(15, 40),
        RaceRecord::new(30, 200)
    ])
}

#[test]
fn test_parse_input_for_part_2() {
    let input: &str = "Time:      7  15   30
    Distance:  9  40  200";

    let parsed = day6::parse_input_for_part_2(input).unwrap();
    assert_eq!(parsed, RaceRecord::new(71530, 940200))
}

#[test]
fn test_smart_number_of_ways_to_win() {
    assert_eq!(day6::smart_number_of_ways_to_win(&RaceRecord::new(7, 9)), 4);
    assert_eq!(day6::smart_number_of_ways_to_win(&RaceRecord::new(15, 40)), 8);
    assert_eq!(day6::smart_number_of_ways_to_win(&RaceRecord::new(30, 200)), 9);
    assert_eq!(day6::smart_number_of_ways_to_win(&RaceRecord::new(71530, 940200)), 71503);
}

#[test]
fn test_solution_part_1() {
    let parsed = day6::parse_input_for_part_1(day6::INPUT);
    if let Ok(input) = parsed {
        assert_eq!(day6::solution_part_1(&input), 288)
    } else {
        assert!(false)
    }
}

#[test]
fn test_solution_part_2() {
    let parsed = day6::parse_input_for_part_2(day6::INPUT);
    if let Ok(input) = parsed {
        assert_eq!(day6::solution_part_2(&input), 71503)
    } else {
        assert!(false)
    }
}