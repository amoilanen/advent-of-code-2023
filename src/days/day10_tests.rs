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
            Connector::new(Coord::new(2, 1), [Coord::new(1, 1), Coord::new(3, 1)], '-'),
            Connector::new(Coord::new(3, 1), [Coord::new(2, 1), Coord::new(3, 2)], '7'),
            Connector::new(Coord::new(1, 2), [Coord::new(1, 1), Coord::new(1, 3)], '|'),
            Connector::new(Coord::new(3, 2), [Coord::new(3, 1), Coord::new(3, 3)], '|'),
            Connector::new(Coord::new(1, 3), [Coord::new(1, 2), Coord::new(2, 3)], 'L'),
            Connector::new(Coord::new(2, 3), [Coord::new(1, 3), Coord::new(3, 3)], '-'),
            Connector::new(Coord::new(3, 3), [Coord::new(2, 3), Coord::new(3, 2)], 'J')
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
fn test_internal_tiles_first_example_loop() {
    let input: &str = "...........
    .S-------7.
    .|F-----7|.
    .||.....||.
    .||.....||.
    .|L-7.F-J|.
    .|..|.|..|.
    .L--J.L--J.
    ...........";

    let landscape = day10::parse(input).unwrap();
    let found_loops = landscape.find_loops(&landscape.starting_title);
    let found_loop = found_loops.first().unwrap();
    let enclosed_tiles_count = day10::count_enclosed_tiles(found_loop, &landscape);
    assert_eq!(enclosed_tiles_count, 4)
}

#[test]
fn test_internal_tiles_second_example_loop() {
    let input: &str = ".F----7F7F7F7F-7....
    .|F--7||||||||FJ....
    .||.FJ||||||||L7....
    FJL7L7LJLJ||LJ.L-7..
    L--J.L7...LJS7F-7L7.
    ....F-J..F7FJ|L7L7L7
    ....L7.F7||L7|.L7L7|
    .....|FJLJ|FJ|F7|.LJ
    ....FJL-7.||.||||...
    ....L---J.LJ.LJLJ...";

    let landscape = day10::parse(input).unwrap();
    let found_loops = landscape.find_loops(&landscape.starting_title);
    let found_loop = found_loops.first().unwrap();
    let enclosed_tiles_count = day10::count_enclosed_tiles(found_loop, &landscape);
    assert_eq!(enclosed_tiles_count, 8)
}

#[test]
fn test_internal_tiles_third_example_loop() {
    let input: &str = "FF7FSF7F7F7F7F7F---7
    L|LJ||||||||||||F--J
    FL-7LJLJ||||||LJL-77
    F--JF--7||LJLJ7F7FJ-
    L---JF-JLJ.||-FJLJJ7
    |F|F-JF---7F7-L7L|7|
    |FFJF7L7F-JF7|JL---7
    7-L-JL7||F7|L7F-7F7|
    L.L7LFJ|||||FJL7||LJ
    L7JLJL-JLJLJL--JLJ.L";

    let landscape = day10::parse(input).unwrap();
    let found_loops = landscape.find_loops(&landscape.starting_title);
    let found_loop = found_loops.first().unwrap();
    let enclosed_tiles_count = day10::count_enclosed_tiles(found_loop, &landscape);
    assert_eq!(enclosed_tiles_count, 10)
}

#[test]
fn test_determine_connector_symbol_j() {
    let input: &str = "....
    .F7.
    .LS.";

    let landscape = day10::parse(input).unwrap();
    let loops = landscape.find_loops(&landscape.starting_title);
    let found_loop = loops.first().unwrap();
    assert_eq!(Some('J'), day10::determine_connector_symbol(found_loop, &landscape.starting_title))
}

#[test]
fn test_determine_connector_symbol_f() {
    let input: &str = "....
    .S7.
    .LJ.";

    let landscape = day10::parse(input).unwrap();
    let loops = landscape.find_loops(&landscape.starting_title);
    let found_loop = loops.first().unwrap();
    assert_eq!(Some('F'), day10::determine_connector_symbol(found_loop, &landscape.starting_title))
}

#[test]
fn test_determine_connector_symbol_7() {
    let input: &str = "....
    .FS.
    .LJ.";

    let landscape = day10::parse(input).unwrap();
    let loops = landscape.find_loops(&landscape.starting_title);
    let found_loop = loops.first().unwrap();
    assert_eq!(Some('7'), day10::determine_connector_symbol(found_loop, &landscape.starting_title))
}

#[test]
fn test_determine_connector_symbol_l() {
    let input: &str = "....
    .F7.
    .SJ.";

    let landscape = day10::parse(input).unwrap();
    let loops = landscape.find_loops(&landscape.starting_title);
    let found_loop = loops.first().unwrap();
    assert_eq!(Some('L'), day10::determine_connector_symbol(found_loop, &landscape.starting_title))
}

#[test]
fn test_determine_connector_symbol_not_in_loop() {
    let input: &str = "....
    .F7.
    .SJ.";

    let landscape = day10::parse(input).unwrap();
    let loops = landscape.find_loops(&landscape.starting_title);
    let found_loop = loops.first().unwrap();
    assert_eq!(None, day10::determine_connector_symbol(found_loop, &Coord::new(1, 0)));
}

#[test]
fn test_solution_1() {
    let parsed = day10::parse(day10::INPUT).unwrap();
    assert_eq!(day10::solution_part_1(&parsed), 8)
}

#[test]
fn test_solution_2() {
    let parsed = day10::parse(day10::INPUT).unwrap();
    assert_eq!(day10::solution_part_2(&parsed), 1)
}