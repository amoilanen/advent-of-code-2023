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
    pub row: i16,
    pub column: i16
}

impl Position {
    pub fn is_within_rectangle(&self, left_top: &Position, right_bottom: &Position) -> bool {
        self.row >= left_top.row && self.row <= right_bottom.row
            && self.column >= left_top.column && self.column <= right_bottom.column
    }
    pub fn new(row: i16, column: i16) -> Position {
        Position {row, column}
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct SchematicNumber {
    pub value: u16,
    pub start: Position,
    pub end: Position
}

impl SchematicNumber {
    pub fn new(value: u16, start: Position, end: Position) -> SchematicNumber {
        SchematicNumber {value, start, end}
    }

    pub fn is_a_neighbor_of(&self, symbol: &SchematicSymbol) -> bool {
        let left_top = Position::new(self.start.row - 1, self.start.column - 1);
        let right_bottom = Position::new(self.end.row + 1, self.end.column + 1);
        symbol.position.is_within_rectangle(&left_top, &right_bottom)
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

    pub fn is_a_neighbor_of(&self, number: &SchematicNumber) -> bool {
        number.is_a_neighbor_of(self)
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Schematic {
    pub numbers: Vec<SchematicNumber>,
    pub symbols: Vec<SchematicSymbol>
}

fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

pub fn get_part_numbers(schematic: &Schematic) -> Vec<&SchematicNumber> {
    let mut part_numbers: Vec<&SchematicNumber> = Vec::new();
    for number in schematic.numbers.iter() {
        let is_part_number = schematic.symbols.iter().any(|symbol| number.is_a_neighbor_of(symbol));
        if is_part_number {
            part_numbers.push(number);
        }
    }
    part_numbers
}

pub fn get_gears_with_adjacent_numbers(schematic: &Schematic) -> Vec<(&SchematicSymbol, Vec<&SchematicNumber>)> {
    let potential_gears: Vec<&SchematicSymbol> = schematic.symbols.iter().filter(|symbol| symbol.value == '*').collect();
    let mut gears_with_numbers: Vec<(&SchematicSymbol, Vec<&SchematicNumber>)> = Vec::new();
    potential_gears.iter().for_each(|potential_gear| {
        let adjacent_part_numbers: Vec<&SchematicNumber> = schematic.numbers.iter()
            .filter(|number| number.is_a_neighbor_of(&potential_gear)).collect();
        if adjacent_part_numbers.len() == 2 {
            gears_with_numbers.push((potential_gear, adjacent_part_numbers))
        }
    });
    gears_with_numbers
}

pub fn parse(input: &str) -> Schematic {
    let rows: Vec<Vec<char>> = input.split_terminator('\n')
        .map(|line| parse_line(line.trim()))
        .collect();

    fn on_number_read_finished(row_index: i16, column_index: i16, current_number: &mut Vec<char>, numbers: &mut Vec<SchematicNumber>) {
        let number: String = current_number.iter().collect::<String>();
        let value = number.parse().unwrap_or(0);
        let start = Position::new(row_index, column_index + 1 - (number.len() as i16));
        let end = Position::new(row_index, column_index);
        numbers.push(SchematicNumber::new(value, start, end));
        current_number.clear()
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
                current_number.push(current_char);
            } else {
                // Non-numeric current_char
                if current_number.len() > 0 {
                    on_number_read_finished(row_index as i16, (column_index - 1) as i16, &mut current_number, &mut numbers);
                }
                if current_char != '.' {
                    symbols.push(SchematicSymbol::new(current_char, Position::new(row_index as i16, column_index as i16)))
                }
            }
            column_index = column_index + 1;
        }
        if current_number.len() > 0 {
            on_number_read_finished(row_index as i16, (column_index - 1) as i16, &mut current_number, &mut numbers);
        }
    }
    Schematic { numbers, symbols }
}

pub fn solution_part_1(schematic: &Schematic) -> u32 {
    let part_numbers = get_part_numbers(&schematic);
    part_numbers.iter().fold(0, |acc, part_number| acc + part_number.value as u32)
}

pub fn solution_part_2(schematic: &Schematic) -> u32 {
    let gears_with_numbers = get_gears_with_adjacent_numbers(schematic);
    gears_with_numbers.iter().map(|(_, numbers)| {
        let gear_ratio = numbers.iter().map(|number| number.value as u32).product::<u32>();
        gear_ratio
    }).sum()
}