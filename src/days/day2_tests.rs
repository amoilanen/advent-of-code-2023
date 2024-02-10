#[cfg(test)]
use crate::days::day2;

use crate::days::day2::Game;
use crate::days::day2::CubeSet;

#[test]
fn test_parse_single_line_input() {
    let input: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let result = day2::parse(input);
    //TODO: Create an utility method to create games and cubesets in a shorter way
    assert_eq!(result, vec![
        Game {
            id: 1,
            drawings: vec![
                CubeSet { red: 4, green: 0, blue: 3 },
                CubeSet { red: 1, green: 2, blue: 6 },
                CubeSet { red: 0, green: 2, blue: 0 }
            ]
        }
    ])
}

//TODO: Test parsing multiline input