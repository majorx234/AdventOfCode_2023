use aoc_lib::read_arg_file;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let reader = read_arg_file().unwrap();
    let mut sum: u32 = 0;
    let fold_fct_task = |mut acc: u32, x: Result<String, _>| -> u32 {
        if let Ok(x) = x {
            let mut first: Option<u8> = None;
            let mut last: Option<u8> = None;
            for value_char in x.chars() {
                if char::is_numeric(value_char) {
                    if let Some(first1) = first {
                        // nothing to do
                    } else {
                        first = Some(value_char as u8 - '0' as u8);
                    }
                    last = Some(value_char as u8 - '0' as u8);
                }
            }
            acc += first.unwrap() as u32 * 10 + last.unwrap() as u32;
        }
        acc
    };
    let sum = reader.lines().fold(sum, fold_fct_task);
    println!("task1 sum: {}", sum);
}
