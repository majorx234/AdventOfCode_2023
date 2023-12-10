use aoc_lib::read_arg_file;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, newline, not_line_ending},
    error::{Error, ParseError},
    multi::{count, separated_list1},
    IResult,
};
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Clone, PartialEq)]
struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Number {
    pub id: usize,
    pub value: usize,
    pub pos: Pos,
    pub len: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Symbol {
    pub pos: Pos,
}

#[derive(Debug, Clone, PartialEq)]
struct EngineSchematic {
    pub symbols: Vec<Symbol>,
    pub numbers: Vec<Number>,
}

#[derive(Debug, PartialEq)]
enum ParseState {
    None,
    Numeric,
}

fn is_symbol(my_char: &char) -> bool {
    if *my_char != '.' && !my_char.is_numeric() {
        return true;
    }
    false
}

fn parse_schematic_line(input: &str, lin_number: i32) -> (Vec<Symbol>, Vec<Number>) {
    let mut state = ParseState::None;
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();
    let mut number_str: Vec<char> = Vec::new();
    let new_number = 0;
    let mut idx = 0;
    for (pos_x, read_char) in input.chars().enumerate() {
        if read_char.is_numeric() {
            state = ParseState::Numeric;
            number_str.push(read_char);
        } else {
            // none numeric now
            if state == ParseState::Numeric {
                let number_string = number_str.iter().collect::<String>();
                let number_usize = number_string.parse::<usize>().unwrap();
                let number = Number {
                    id: idx,
                    value: number_usize,
                    pos: Pos {
                        x: (pos_x - number_str.len()) as i32,
                        y: lin_number,
                    },
                    len: number_str.len() as i32,
                };
                numbers.push(number);
                idx += 1;
                number_str.clear();
                // save number
                // check if symbol
            }
            state = ParseState::None;
            if is_symbol(&read_char) {
                let pos = Pos {
                    x: pos_x as i32,
                    y: lin_number,
                };
                let symbol = Symbol { pos };
                symbols.push(symbol);
            }
        }
    }
    (symbols, numbers)
}

fn main() {
    let reader = read_arg_file().unwrap();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();

    for (pos, line) in reader.lines().enumerate() {
        let (mut symbols_new, mut numbers_new) = parse_schematic_line(&line.unwrap(), pos as i32);
        symbols.append(&mut symbols_new);
        numbers.append(&mut numbers_new);
    }
    let engine_schematic = EngineSchematic { symbols, numbers };
    println!("{:?}", engine_schematic);
    let adjacent_numbers = find_adjacent_numbers(engine_schematic);
    println!("{:?}", adjacent_numbers);
}

fn is_adjacent(symbol: &Symbol, number: &Number) -> bool {
    if !(symbol.pos.y == number.pos.y
        || symbol.pos.y == number.pos.y - 1
        || symbol.pos.y == number.pos.y + 1)
    {
        return false;
    }
    for offset in 0..number.len {
        if symbol.pos.x == number.pos.x
            || symbol.pos.x == number.pos.x - 1
            || symbol.pos.x == number.pos.x + 1
        {
            return true;
        }
    }
    false
}

fn find_adjacent_numbers(engine_schematic: EngineSchematic) -> Vec<Number> {
    let numbers = engine_schematic.numbers;
    let symbols = engine_schematic.symbols;
    let mut idx_row: i32 = symbols[0].pos.y;
    let row_max = symbols[symbols.len() - 1].pos.y + 1;
    let mut numbers_row_min_idx = idx_row - 1;
    let mut numbers_real_idx = 0;
    let mut symbol_real_idx = 0;
    let mut adjacent_numbers: Vec<Number> = Vec::new();

    loop {
        if idx_row > row_max {
            break;
        }
        if symbol_real_idx >= symbols.len() {
            break;
        }
        let my_symbol = symbols[symbol_real_idx].clone();
        symbol_real_idx += 1;
        let mut numbers_to_check: Vec<Number> = Vec::new();
        let mut number_work_idx = numbers_real_idx;
        loop {
            if number_work_idx >= numbers.len() {
                break;
            }
            let number_to_check = numbers[number_work_idx].clone();
            if number_to_check.pos.y > idx_row + 1 {
                break;
            }
            numbers_to_check.push(number_to_check.clone());
            number_work_idx += 1;
        }
        for number_to_check in numbers_to_check {
            if is_adjacent(&my_symbol, &number_to_check) {
                adjacent_numbers.push(number_to_check.clone());
            }
        }
        idx_row = my_symbol.pos.y;

        loop {
            if numbers_real_idx >= numbers.len() {
                break;
            }
            if numbers[numbers_real_idx].pos.y >= idx_row - 1 {
                break;
            }
            numbers_real_idx += 1;
        }
    }
    return adjacent_numbers;
}
