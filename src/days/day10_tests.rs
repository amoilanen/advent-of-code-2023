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

#[test]
fn test_find_single_loop() {
    let input: &str = ".....
    .S-7.
    .|.|.
    .L-J.
    .....";

    let landscape = day10::parse(input).unwrap();
    let found_loop = landscape.find_loops(&landscape.starting_title);
    // Counter-clockwise one by one
    assert_eq!(found_loop, vec![vec![
        Coord::new(1, 1), Coord::new(2, 1), Coord::new(3, 1),
        Coord::new(3, 2), Coord::new(3, 3),
        Coord::new(2, 3), Coord::new(1, 3),
        Coord::new(1, 2)
    ]])
}

#[test]
fn test_find_loop_two_loops_starting_at_same_point() {
    let input: &str = ".....
    .F7..
    .LS7.
    ..LJ.
    .....";

    let landscape = day10::parse(input).unwrap();
    let found_loop = landscape.find_loops(&landscape.starting_title);
    // Counter-clockwise one by one
    assert_eq!(found_loop, vec![vec![
            Coord::new(2, 2), Coord::new(3, 2), Coord::new(3, 3), Coord::new(2, 3)
        ],
        vec![
            Coord::new(2, 2), Coord::new(1, 2), Coord::new(1, 1), Coord::new(2, 1)
        ]
    ])
}

#[test]
fn test_no_loop_can_be_found() {
    let input: &str = ".....
    ..|.|
    .-S-J
    ..|..
    .....";

    let landscape = day10::parse(input).unwrap();
    let found_loop = landscape.find_loops(&landscape.starting_title);
    // Counter-clockwise one by one
    assert_eq!(found_loop, Vec::<Vec<Coord>>::new())
}

#[test]
fn test_there_is_a_loop_but_a_line_of_edges_which_forms_no_loop() {
    let input: &str = ".....
    .F7..
    .LS-7
    ..|.|
    ..L.J";

    let landscape = day10::parse(input).unwrap();
    let found_loop = landscape.find_loops(&landscape.starting_title);
    // Counter-clockwise one by one
    assert_eq!(found_loop, vec![
        vec![
            Coord::new(2, 2), Coord::new(1, 2), Coord::new(1, 1), Coord::new(2, 1)
        ]
    ])
}

#[test]
fn test_solution_1() {
    let parsed = day10::parse(day10::INPUT).unwrap();
    assert_eq!(day10::solution_part_1(&parsed), 8)
}