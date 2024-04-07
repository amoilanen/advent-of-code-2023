#[cfg(test)]
use crate::days::day9;
#[cfg(test)]
use crate::days::day9::ValueHistory;

#[test]
fn test_parse_input() {
    let input: &str = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";

    let parsed = day9::parse(input);
    assert_eq!(parsed, vec![
        ValueHistory::new(vec![0, 3, 6, 9, 12, 15]),
        ValueHistory::new(vec![1, 3, 6, 10, 15, 21]),
        ValueHistory::new(vec![10, 13, 16, 21, 30, 45])
    ])
}

#[test]
fn test_pairs_of() {
    assert_eq!(day9::pairs_of(&vec![1, 2, 3, 4, 5]), vec![(1, 2), (2, 3), (3, 4), (4, 5)]);
    assert_eq!(day9::pairs_of(&vec![1]), Vec::new());
    assert_eq!(day9::pairs_of(&Vec::<i64>::new()), Vec::new());
}

#[test]
fn test_differences_of() {
    assert_eq!(day9::differences_of(&vec![1, 2, 4, 7, 11]), vec![1, 2, 3, 4]);
    assert_eq!(day9::differences_of(&vec![1]), Vec::new());
    assert_eq!(day9::differences_of(&Vec::<i64>::new()), Vec::new());
}

#[test]
fn test_is_all_zeros() {
    assert!(!day9::is_all_zeros(&vec![1, 2, 3]));
    assert!(!day9::is_all_zeros(&vec![0, 0, 0, 1, 0]));
    assert!(day9::is_all_zeros(&vec![0, 0, 0, 0, 0]));
}

#[test]
fn test_extrapolate_next() {
    assert_eq!(day9::extrapolate_next(&vec![0, 3, 6, 9, 12, 15]), 18);
    assert_eq!(day9::extrapolate_next(&vec![1, 3, 6, 10, 15, 21]), 28);
    assert_eq!(day9::extrapolate_next(&vec![10, 13, 16, 21, 30, 45]), 68);
    assert_eq!(day9::extrapolate_next(&vec![0, 1, 2, 3, 4, 5]), 6);
    assert_eq!(day9::extrapolate_next(&vec![0, 0, 0, 0]), 0);
}

#[test]
fn test_extrapolate_previous() {
    assert_eq!(day9::extrapolate_previous(&vec![0, 3, 6, 9, 12, 15]), -3);
    assert_eq!(day9::extrapolate_previous(&vec![1, 3, 6, 10, 15, 21]), 0);
    assert_eq!(day9::extrapolate_previous(&vec![10, 13, 16, 21, 30, 45]), 5);
    assert_eq!(day9::extrapolate_previous(&vec![1, 2, 3, 4, 5]), 0);
    assert_eq!(day9::extrapolate_previous(&vec![0, 0, 0, 0]), 0);
}

#[test]
fn test_solution_1() {
    let parsed = day9::parse(day9::INPUT);
    assert_eq!(day9::solution_part_1(&parsed), 114)
}

#[test]
fn test_solution_2() {
    let parsed = day9::parse(day9::INPUT);
    assert_eq!(day9::solution_part_2(&parsed), 2)
}