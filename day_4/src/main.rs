use aoc_lib::read_arg_file;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, newline, not_line_ending},
    error::{Error, ParseError},
    multi::{count, many1, many_till, separated_list1},
    IResult,
};
use std::collections::HashMap;
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
    let fold_number_in_map_fct = |mut map: HashMap<u32, bool>, value: &u32| -> HashMap<u32, bool> {
        map.insert(*value, true);
        map
    };
    let fold_check_winning_numbers_fct =
        |(mut exp, map): (u32, HashMap<u32, bool>), value: &u32| -> (u32, HashMap<u32, bool>) {
            if map.contains_key(value) {
                exp += 1;
            }
            (exp, map)
        };
    let mut task1_sum = 0;
    for card in cards {
        let (card_id, winner_numbers, your_numbers) = card;
        let winner_map: HashMap<u32, bool> = HashMap::new();
        let winner_map = winner_numbers
            .iter()
            .fold(winner_map, fold_number_in_map_fct);
        let mut amount_winners = 0;
        let (exp, _) = your_numbers
            .iter()
            .fold((amount_winners, winner_map), fold_check_winning_numbers_fct);
        let mut card_value = 0;
        if exp != 0 {
            card_value = 2_u32.pow(exp - 1);
        }
        println!("card_id:{} exp: {} value: {}", card_id, exp, card_value);
        task1_sum += card_value;
    }
    println!("task1_sum: {}", task1_sum);
}
