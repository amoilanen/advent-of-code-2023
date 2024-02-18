#[cfg(test)]
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
            SchematicNumber::new(12,Position::new(0, 0), Position::new(0, 1)),
            SchematicNumber::new(34, Position::new(1, 2), Position::new(1, 3)),
        ],
        symbols: vec![
            SchematicSymbol::new('*', Position::new(0, 4))
        ]
    })
}

#[test]
fn test_get_part_numbers() {
    let input: &str = 
"467..114..
 ...*......
 ..35..633.";
    let schematic: Schematic = day3::parse(input);
    let result = day3::get_part_numbers(&schematic);
    assert_eq!(result, vec![
        &SchematicNumber::new(467, Position::new(0, 0), Position::new(0, 2)),
        &SchematicNumber::new(35, Position::new(2, 2), Position::new(2, 3))
    ]);
}

#[test]
fn test_solution_part1() {
  let parsed = day3::parse(day3::INPUT);
  assert_eq!(day3::solution_part_1(&parsed), 4361)
}