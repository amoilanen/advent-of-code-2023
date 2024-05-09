use crate::day11::compute_coefficients;
#[cfg(test)]
use crate::days::day11;
#[cfg(test)]
use crate::days::day11::{ Universe, Galaxy };

#[test]
fn test_parse_input() {
    let input: &str = "...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....";

    let parsed = day11::parse(input);
    assert_eq!(parsed, Universe::new(
        vec![
            Galaxy::new(3, 0), Galaxy::new(7, 1), Galaxy::new(0, 2),
            Galaxy::new(6, 4), Galaxy::new(1, 5), Galaxy::new(9, 6),
            Galaxy::new(7, 8), Galaxy::new(0, 9), Galaxy::new(4, 9)
        ]
    ))
}

#[test]
fn test_universe_expansion() {
    let input: &str = "...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....";
    let expanded_input: &str = "....#........
    .........#...
    #............
    .............
    .............
    ........#....
    .#...........
    ............#
    .............
    .............
    .........#...
    #....#.......";

    let universe = day11::parse(input);
    let expanded_universe = day11::parse(expanded_input);
    assert_eq!(expanded_universe, universe.expand())
}

#[test]
fn test_galaxy_distance_to() {
    let first = Galaxy::new(1, 6);
    let second = Galaxy::new(5, 11);
    assert_eq!(first.distance_to(&second), 9);
}

#[test]
fn test_compute_coefficients() {
    assert_eq!(compute_coefficients(0), Vec::new());
    assert_eq!(compute_coefficients(1), Vec::new());
    assert_eq!(compute_coefficients(2), vec![1]);
    assert_eq!(compute_coefficients(3), vec![2, 2]);
    assert_eq!(compute_coefficients(4), vec![3, 4, 3]);
    assert_eq!(compute_coefficients(5), vec![4, 6, 6, 4]);
    assert_eq!(compute_coefficients(6), vec![5, 8, 9, 8, 5]);
    assert_eq!(compute_coefficients(7), vec![6, 10, 12, 12, 10, 6]);
}

#[test]
fn test_solution_1() {
    let parsed = day11::parse(day11::INPUT);
    assert_eq!(day11::solution_part_1(&parsed), 374);
}
