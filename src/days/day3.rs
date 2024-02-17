use std::collections::HashSet;

pub const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct Position {
    pub row: usize,
    pub column: usize
}

impl Position {

    pub fn adjacent_positions(&self) -> Vec<Position> {
        let mut adjacent: Vec<Position> = Vec::new();
        for adjacent_row in [self.row as i16 - 1, self.row as i16, self.row as i16 + 1] {
            for adjacent_column in [self.column as i16 - 1, self.column as i16, self.column as i16 + 1] {
                if adjacent_row >= 0 && adjacent_column >= 0 && (adjacent_row != self.row as i16 || adjacent_column != self.column as i16) {
                    adjacent.push(Position::new(adjacent_row as usize, adjacent_column as usize));
                } 
            }
        }
        adjacent
    }

    pub fn new(row: usize, column: usize) -> Position {
        Position {row, column}
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct SchematicNumber {
    pub value: u16,
    pub coordinates: Vec<Position>
}

impl SchematicNumber {
    pub fn new(value: u16, coordinates: Vec<Position>) -> SchematicNumber {
        SchematicNumber {value, coordinates}
    }

    pub fn adjacent_positions(&self) -> Vec<Position> {
        let mut adjacent: Vec<Position> = Vec::new();
        for coordinate in &self.coordinates {
            for position in coordinate.adjacent_positions() {
                if !self.coordinates.contains(&position) {
                    adjacent.push(position)
                }
            }
        }
        adjacent
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct SchematicSymbol {
    pub value: char,
    pub position: Position
}

impl SchematicSymbol {
    pub fn new(value: char, position: Position) -> SchematicSymbol {
        SchematicSymbol {value, position}
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Schematic {
    pub rows: Vec<Vec<char>>,
    pub numbers: Vec<SchematicNumber>,
    pub symbols: Vec<SchematicSymbol>
}

fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

pub fn get_part_numbers(schematic: &Schematic) -> Vec<&SchematicNumber> {
    let mut symbol_positions: HashSet<&Position> = HashSet::new();
    for symbol in schematic.symbols.iter() {
        symbol_positions.insert(&symbol.position);
    }

    let mut part_numbers: Vec<&SchematicNumber> = Vec::new();
    for number in schematic.numbers.iter() {
        let adjacent_positions = number.adjacent_positions();
        let is_part_number = adjacent_positions.iter().any(|position| symbol_positions.contains(position));
        if is_part_number {
            part_numbers.push(number);
        }
    }
    part_numbers
}

pub fn parse(input: &str) -> Schematic {
    let rows: Vec<Vec<char>> = input.split_terminator('\n')
        .map(|line| parse_line(line.trim()))
        .collect();


    fn on_number_read(current_char: char, current_number: &mut Vec<char>) {
        current_number.push(current_char)
    }

    fn on_number_read_finished(row_index: usize, column_index: usize, current_number: &mut Vec<char>, numbers: &mut Vec<SchematicNumber>) {
        let number: String = current_number.iter().collect::<String>();
        let coordinates: Vec<Position> = (column_index + 1 - number.len()..=column_index).map(|column| Position::new(row_index, column)).collect();
        numbers.push(SchematicNumber::new(number.parse().unwrap_or(0), coordinates));
        current_number.clear()
    }

    fn on_symbol_read_finished(row_index: usize, column_index: usize, current_char: char, symbols: &mut Vec<SchematicSymbol>) {
        symbols.push(SchematicSymbol::new(current_char, Position::new(row_index, column_index)))
    }

    let mut numbers: Vec<SchematicNumber> = Vec::new();
    let mut symbols: Vec<SchematicSymbol> = Vec::new();
    for (row_index, row) in rows.iter().enumerate() {
        let number_of_columns = row.len();
        let mut column_index = 0;
        let mut current_number: Vec<char> = Vec::new();
        while column_index < number_of_columns {
            let current_char = row[column_index];
            if current_char.is_numeric() {
                on_number_read(current_char, &mut current_number);
            } else {
                // Non-numeric current_char
                if current_number.len() > 0 {
                    on_number_read_finished(row_index, column_index - 1, &mut current_number, &mut numbers);
                }
                if current_char != '.' {
                    on_symbol_read_finished(row_index, column_index, current_char, &mut symbols);
                }
            }
            column_index = column_index + 1;
        }
        if current_number.len() > 0 {
            on_number_read_finished(row_index, column_index, &mut current_number, &mut numbers);
        }
    }
    Schematic { rows, numbers, symbols }
}

pub fn solution_part_1(parsed_input: &Schematic) -> u16 {
    1
}

pub fn solution_part_2(parsed_input: &Schematic) -> u32 {
    1
}