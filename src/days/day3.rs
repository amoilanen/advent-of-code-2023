use core::num;

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

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Position {
    pub row: usize,
    pub column: usize
}

impl Position {
    fn is_adjacent_to(&self, other: &Position) -> bool {
        ((other.row as i16) - (self.row as i16)).abs() <= 1
        &&
        ((other.column as i16) - (self.column as i16)).abs() <= 1
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