use aoc_lib::read_arg_file;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, newline, not_line_ending},
    error::{Error, ParseError},
    multi::{count, many1, many_till, separated_list1},
    IResult,
};
use std::io::{self, prelude::*, BufReader};

fn parse_number(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, numbers) = separated_list1(many1(tag(" ")), complete::u32)(input)?;
    Ok((input, numbers))
}

fn parse_card(input: &str) -> IResult<&str, (u32, Vec<u32>, Vec<u32>)> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = many1(tag(" "))(input)?;
    let (input, card_id) = complete::u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = many1(tag(" "))(input)?;
    let (input, (winner_numbers, _)) = many_till(parse_number, tag(" |"))(input)?;
    let (input, _) = many1(tag(" "))(input)?;
    let (input, your_numbers) = separated_list1(many1(tag(" ")), complete::u32)(input)?;
    let vec_winner_numbers = winner_numbers[0].clone();
    Ok((input, (card_id, vec_winner_numbers, your_numbers)))
}

fn main() {
    let reader = read_arg_file().unwrap();
    let mut cards: Vec<(u32, Vec<u32>, Vec<u32>)> = Vec::new();
    for line in reader.lines() {
        if let Ok((_, card)) = parse_card(&line.unwrap()) {
            cards.push(card);
            // println!("game: {:?}", game);
        }
    }
    println!("cards: {:?}", cards);
}
