use aoc_lib::read_arg_file;
use std::io::{self, prelude::*, BufReader};

fn find_last_numeric_in_string(x: &String) -> Option<u8> {
    let substring_list = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let mut best_pos = 0;
    let mut best_string = "0";
    for substring in substring_list {
        if let Some(start) = x.rfind(substring) {
            if start >= best_pos {
                best_string = substring;
                best_pos = start;
            }
        }
    }
    let best_digit_string = best_string
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9");
    return Some(best_digit_string.chars().last().unwrap() as u8 - '0' as u8);
}

fn find_first_numeric_in_string(x: &String) -> Option<u8> {
    let substring_list = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let mut best_pos = x.len() + 1;
    let mut best_string = "0";
    for substring in substring_list {
        if let Some(start) = x.find(substring) {
            if start < best_pos {
                best_string = substring;
                best_pos = start;
            }
        }
    }
    let best_digit_string = best_string
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9");
    return Some(best_digit_string.chars().last().unwrap() as u8 - '0' as u8);
}

fn main() {
    let reader = read_arg_file().unwrap();
    let mut sum: u32 = 0;
    let fold_fct_task = |mut acc: u32, x: Result<String, _>| -> u32 {
        if let Ok(x) = x {
            let mut first: Option<u8> = None;
            let mut last: Option<u8> = None;

            first = find_first_numeric_in_string(&x);
            last = find_last_numeric_in_string(&x);
            acc += first.unwrap() as u32 * 10 + last.unwrap() as u32;
            println!(
                "x: {} first: {} last: {} acc: {}",
                x,
                first.unwrap(),
                last.unwrap(),
                acc
            );
        };
        acc
    };
    let sum = reader.lines().fold(sum, fold_fct_task);
    println!("task1 sum: {}", sum);
}
