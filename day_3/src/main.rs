use aoc_lib::read_arg_file;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, newline, not_line_ending},
    error::{Error, ParseError},
    multi::{count, separated_list1},
    IResult,
};
use std::io::{self, prelude::*, BufReader};

struct Pos {
    x: i32,
    y: i32,
}

struct Number {
    id: usize,
    pos: Pos,
    len: i32,
}

struct Symbol {
    pos: Pos,
}

struct EngineSchematic {
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}

fn parse_schematic_line(input: &str, lin_number: i32) -> (Vec<Symbol>, Vec<Number>) {
    for (pos, char) in input.chars().enumerate() {
        println!("char {} at pos {}", pos, char);
    }
    (Vec::new(), Vec::new())
}

fn main() {
    let reader = read_arg_file().unwrap();
    for (pos, line) in reader.lines().enumerate() {
        parse_schematic_line(&line.unwrap(), pos as i32);
    }
}
