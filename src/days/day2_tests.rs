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

//TODO: Test parsing multiline input