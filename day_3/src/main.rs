use aoc_lib::read_arg_file;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, newline, not_line_ending},
    error::{Error, ParseError},
    multi::{count, separated_list1},
    IResult,
};
use std::collections::HashMap;
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
    pub is_star: bool,
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

fn parse_schematic_line(
    input: &str,
    lin_number: i32,
    idx: &mut usize,
) -> (Vec<Symbol>, Vec<Number>) {
    let mut state = ParseState::None;
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();
    let mut number_str: Vec<char> = Vec::new();
    let new_number = 0;
    let mut last_pos_x = 0;
    for (pos_x, read_char) in input.chars().enumerate() {
        last_pos_x = pos_x;
        if read_char.is_numeric() {
            state = ParseState::Numeric;
            number_str.push(read_char);
        } else {
            // none numeric now
            if state == ParseState::Numeric {
                let number_string = number_str.iter().collect::<String>();
                let number_usize = number_string.parse::<usize>().unwrap();
                let number = Number {
                    id: *idx,
                    value: number_usize,
                    pos: Pos {
                        x: (pos_x - number_str.len()) as i32,
                        y: lin_number,
                    },
                    len: number_str.len() as i32,
                };
                numbers.push(number);
                *idx += 1;
                number_str.clear();
                // save number
                // check if symbol
            }
            state = ParseState::None;
            if is_symbol(&read_char) {
                let is_star = if read_char == '*' { true } else { false };
                let pos = Pos {
                    x: pos_x as i32,
                    y: lin_number,
                };
                let symbol = Symbol { is_star, pos };
                symbols.push(symbol);
            }
        }
    }
    if state == ParseState::Numeric {
        let number_string = number_str.iter().collect::<String>();
        let number_usize = number_string.parse::<usize>().unwrap();
        let number = Number {
            id: *idx,
            value: number_usize,
            pos: Pos {
                x: (last_pos_x - number_str.len()) as i32,
                y: lin_number,
            },
            len: number_str.len() as i32,
        };
        numbers.push(number);
        *idx += 1;
    }
    (symbols, numbers)
}

fn main() {
    let reader = read_arg_file().unwrap();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();
    let mut number_idx = 0;

    for (pos, line) in reader.lines().enumerate() {
        let (mut symbols_new, mut numbers_new) =
            parse_schematic_line(&line.unwrap(), pos as i32, &mut number_idx);
        symbols.append(&mut symbols_new);
        numbers.append(&mut numbers_new);
    }
    let engine_schematic = EngineSchematic { symbols, numbers };
    let (adjacent_numbers, gear_ratios) = find_adjacent_numbers(engine_schematic);
    // transfer to Hashmap to eliminate multiple entries for same number
    let mut unique_adjacent_numbers = HashMap::new();
    let mut sum_of_numbers = 0;
    let mut sum_of_unique_numbers = 0;
    for number in adjacent_numbers {
        sum_of_numbers += number.value;

        unique_adjacent_numbers.insert(number.value, number);
    }

    for (_, number) in unique_adjacent_numbers {
        sum_of_unique_numbers += number.value;
    }
    println!(
        "sum of numbers: {},\n unique: {}",
        sum_of_numbers, sum_of_unique_numbers
    );

    let mut sum_gear_ratio = 0;
    for gear_ratio in gear_ratios {
        sum_gear_ratio += gear_ratio;
    }
    println!("sum of gear_ratios: {}", sum_gear_ratio);
}

fn is_adjacent(symbol: &Symbol, number: &Number) -> bool {
    if !(symbol.pos.y == number.pos.y
        || symbol.pos.y == number.pos.y - 1
        || symbol.pos.y == number.pos.y + 1)
    {
        return false;
    }
    for offset in 0..number.len {
        if symbol.pos.x == number.pos.x + offset
            || symbol.pos.x == number.pos.x + offset - 1
            || symbol.pos.x == number.pos.x + offset + 1
        {
            return true;
        }
    }
    false
}

fn find_adjacent_numbers(engine_schematic: EngineSchematic) -> (Vec<Number>, Vec<usize>) {
    let numbers = engine_schematic.numbers;
    let symbols = engine_schematic.symbols;
    let mut idx_row: i32 = symbols[0].pos.y;
    let row_max = symbols[symbols.len() - 1].pos.y + 1;
    let mut numbers_row_min_idx = idx_row - 1;
    let mut numbers_real_idx = 0;
    let mut symbol_real_idx = 0;
    let mut adjacent_numbers: Vec<Number> = Vec::new();
    let mut gear_ratios: Vec<usize> = Vec::new();

    loop {
        if idx_row > row_max {
            break;
        }
        let my_symbol = symbols[symbol_real_idx].clone();
        let mut star_adjacent_numbers: Vec<Number> = Vec::new();
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
        println!(
            "debug: {:?},\nnumbers to check: {:?},\n idx_row{:?}",
            my_symbol, numbers_to_check, idx_row
        );
        for number_to_check in numbers_to_check {
            if is_adjacent(&my_symbol, &number_to_check) {
                adjacent_numbers.push(number_to_check.clone());
                if my_symbol.is_star {
                    star_adjacent_numbers.push(number_to_check.clone());
                }
            }
        }
        if star_adjacent_numbers.len() == 2 {
            let gear_ratio = star_adjacent_numbers[0].value * star_adjacent_numbers[1].value;
            gear_ratios.push(gear_ratio);
        }
        for number in adjacent_numbers.iter() {
            print!("{} ", number.value);
        }

        if symbol_real_idx >= symbols.len() {
            break;
        }

        idx_row = symbols[symbol_real_idx].pos.y;
        loop {
            if numbers_real_idx >= numbers.len() {
                break;
            }
            if numbers[numbers_real_idx].pos.y >= idx_row - 1 {
                break;
            }
            numbers_real_idx += 1;
        }
        if numbers_real_idx >= numbers.len() {
            break;
        }
    }
    println!("gear_ratios: {:?}", gear_ratios);
    (adjacent_numbers, gear_ratios)
}
