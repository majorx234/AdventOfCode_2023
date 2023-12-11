use aoc_lib::read_arg_file;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, newline, not_line_ending},
    error::{Error, ParseError},
    multi::{count, many1, many_till, separated_list1},
    IResult,
};
use std::io::{self, prelude::*, BufReader};

fn parse_seeds(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(many1(tag(" ")), complete::u32)(input)?;
    let (input, _) = many1(tag("\n"))(input)?;
    Ok((input, seeds))
}

fn parse_3tuple(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let (input, first) = complete::u32(input)?;
    let (input, _) = many1(tag(" "))(input)?;
    let (input, second) = complete::u32(input)?;
    let (input, _) = many1(tag(" "))(input)?;
    let (input, third) = complete::u32(input)?;
    Ok((input, (first, second, third)))
}

fn parse_map(input: &str) -> IResult<&str, Vec<(u32, u32, u32)>> {
    let (input, _) = tag("map_name")(input)?;
    let (input, _) = tag(":\n ")(input)?;
    let (input, map) = separated_list1(tag("\n"), parse_3tuple)(input)?;
    let (input, _) = many1(tag("\n"))(input)?;
    Ok((input, Vec::new()))
}

fn main() {
    let reader = read_arg_file().unwrap();
}
