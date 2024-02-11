#[cfg(test)]
use crate::days::day2;
#[cfg(test)]
use crate::days::day2::Game;
#[cfg(test)]
use crate::days::day2::CubeSet;

#[test]
fn test_parse_single_line_input() {
    let input: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let result = day2::parse(input);
    assert_eq!(result, vec![
        Game:: new(1, vec![
            CubeSet::new(4, 0, 3),
            CubeSet::new(1, 2, 6),
            CubeSet::new(0, 2, 0)
        ])
    ])
}

#[test]
fn test_parse_multiline_input() {
    let input: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let result = day2::parse(input);
    assert_eq!(result, vec![
        Game:: new(1, vec![
            CubeSet::new(4, 0, 3),
            CubeSet::new(1, 2, 6),
            CubeSet::new(0, 2, 0)
        ]),
        Game:: new(2, vec![
            CubeSet::new(0, 2, 1),
            CubeSet::new(1, 3, 4),
            CubeSet::new(0, 1, 1)
        ]),
        Game:: new(3, vec![
            CubeSet::new(20, 8, 6),
            CubeSet::new(4, 13, 5),
            CubeSet::new(1, 5, 0)
        ]),
        Game:: new(4, vec![
            CubeSet::new(3, 1, 6),
            CubeSet::new(6, 3, 0),
            CubeSet::new(14, 3, 15)
        ]),
        Game:: new(5, vec![
            CubeSet::new(6, 3, 1),
            CubeSet::new(1, 2, 2)
        ])
    ])
}

#[test]
fn test_fewest_number_of_cubes() {
    let game = Game:: new(1, vec![
        CubeSet::new(4, 0, 3),
        CubeSet::new(1, 2, 6),
        CubeSet::new(0, 2, 0)
    ]);
    assert_eq!(game.minimal_cube_set(), CubeSet::new(4, 2, 6))
}

#[test]
fn test_solution_part1() {
  let parsed = day2::parse(day2::INPUT);
  assert_eq!(day2::solution_part_1(&parsed), 8)
}

#[test]
fn test_solution_part2() {
  let parsed = day2::parse(day2::INPUT);
  assert_eq!(day2::solution_part_2(&parsed), 2286)
}