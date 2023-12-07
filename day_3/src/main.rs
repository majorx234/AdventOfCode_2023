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
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Number {
    id: usize,
    pos: Pos,
    len: i32,
}

#[derive(Debug, PartialEq)]
struct Symbol {
    pos: Pos,
}

#[derive(Debug, PartialEq)]
struct EngineSchematic {
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}

#[derive(Debug, PartialEq)]
enum ParseState {
    None,
    Numeric,
}

fn is_symbol(my_char: char) -> bool {
    if my_char != '.' && !my_char.is_numeric() {
        return true;
    }
    return false;
}

fn parse_schematic_line(input: &str, lin_number: i32) -> (Vec<Symbol>, Vec<Number>) {
    let mut state = ParseState::None;
    let symbols: Vec<Symbol> = Vec::new();
    let numbers: Vec<Number> = Vec::new();
    let mut number_len = 0;
    let new_number = 0;
    for (pos, char) in input.chars().enumerate() {
        if char.is_numeric() {
            state = ParseState::Numeric;
            number_len += 1;
        } else {
            // none numeric now
            if state == ParseState::Numeric {
                state = ParseState::None;
                // save number
                // check if symbol
            } else {
                todo!()
            }
        }
    }
    (Vec::new(), Vec::new())
}

fn main() {
    let reader = read_arg_file().unwrap();
    for (pos, line) in reader.lines().enumerate() {
        parse_schematic_line(&line.unwrap(), pos as i32);
    }
}
