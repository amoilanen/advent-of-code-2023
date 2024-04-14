#[cfg(test)]
use crate::days::day10;
#[cfg(test)]
use crate::days::day10::{ Coord, Connector, Landscape };

#[test]
fn test_parse_input() {
    let input: &str = ".....
    .S-7.
    .|.|.
    .L-J.
    .....";

    let parsed = day10::parse(input).unwrap();
    assert_eq!(parsed, Landscape::new(
        Coord::new(1, 1),
        vec![
            Connector::new(Coord::new(2, 1), [Coord::new(1, 1), Coord::new(3, 1)]), // -
            Connector::new(Coord::new(3, 1), [Coord::new(2, 1), Coord::new(3, 2)]), // 7
            Connector::new(Coord::new(1, 2), [Coord::new(1, 1), Coord::new(1, 3)]), // |
            Connector::new(Coord::new(3, 2), [Coord::new(3, 1), Coord::new(3, 3)]), // |
            Connector::new(Coord::new(1, 3), [Coord::new(1, 2), Coord::new(2, 3)]), // L
            Connector::new(Coord::new(2, 3), [Coord::new(1, 3), Coord::new(3, 3)]), // -
            Connector::new(Coord::new(3, 3), [Coord::new(2, 3), Coord::new(3, 2)]) // J
        ]
    ))
}