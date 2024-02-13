use crate::day3::{Position, Schematic, SchematicNumber, SchematicSymbol};
#[cfg(test)]
use crate::days::day3;

#[test]
fn test_parse() {
    let input: &str = "12..*.
..34..";
    let result = day3::parse(input);
    assert_eq!(result, day3::Schematic {
        rows: vec![
            "12..*.".chars().collect(),
            "..34..".chars().collect()
        ],
        numbers: vec![
            SchematicNumber::new(12, vec![ Position::new(0, 0), Position::new(0, 1)]),
            SchematicNumber::new(34, vec![ Position::new(1, 2), Position::new(1, 3)]),
        ],
        symbols: vec![
            SchematicSymbol::new('*', Position::new(0, 4))
        ]
    })
}