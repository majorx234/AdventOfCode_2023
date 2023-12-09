use aoc_lib::read_arg_file;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, newline, not_line_ending},
    error::{Error, ParseError},
    multi::{count, separated_list1},
    IResult,
};
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, PartialEq)]
struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq)]
struct Number {
    pub id: usize,
    pub value: usize,
    pub pos: Pos,
    pub len: i32,
}

#[derive(Debug, PartialEq)]
struct Symbol {
    pub pos: Pos,
}

#[derive(Debug, PartialEq)]
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
}

fn is_adjacent(symbol: Symbol, number: Number) -> bool {
    if !(symbol.pos.y == number.pos.y
        || symbol.pos.y == number.pos.y - 1
        || symbol.pos.y == number.pos.y + 1)
    {
        return false;
    }
    for offset in 0..number.len {
        if (symbol.pos.x == number.pos.x
            || symbol.pos.x == number.pos.x - 1
            || symbol.pos.x == number.pos.x + 1)
        {
            return true;
        }
    }
    false
}
