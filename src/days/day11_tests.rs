#[cfg(test)]
use crate::days::day11;
#[cfg(test)]
use crate::days::day11::{ Universe, Galaxy, Coord };

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