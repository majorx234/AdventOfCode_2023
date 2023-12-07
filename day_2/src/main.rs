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
enum Color {
    BLUE,
    GREEN,
    RED,
    None,
}
impl From<&str> for Color {
    fn from(s: &str) -> Self {
        match s {
            "red" => Color::RED,
            "blue" => Color::BLUE,
            "green" => Color::GREEN,
            _ => Color::None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Round {
    pub sets: Vec<(u32, Color)>,
}

#[derive(Debug, PartialEq)]
struct Game {
    pub game_id: u32,
    pub rounds: Vec<Round>,
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let input_len = input.len();
    if let Some(pos) = input.find("blue") {
        if pos < 2 {
            return Ok((&input[pos + 4..input_len], Color::BLUE));
        }
    }
    if let Some(pos) = input.find("red") {
        if pos < 2 {
            return Ok((&input[pos + 3..input_len], Color::RED));
        }
    }
    if let Some(pos) = input.find("green") {
        if pos < 2 {
            return Ok((&input[pos + 5..input_len], Color::GREEN));
        }
    }

    /*   if let Ok((input, _)) = tag("blue")(input) {
        return Ok((input, Color::BLUE));
    }
    if let Ok((input, _)) = tag("red")(input) {
        return Ok((input, Color::RED));
    }
    let (input, _) = tag("green")(input)?;*/
    return Ok((input, Color::None));
}

fn parse_dice(input: &str) -> IResult<&str, (u32, Color)> {
    let (input, value) = complete::u32(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = parse_color(input)?;
    return Ok((input, (value, color)));
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, sets) = separated_list1(tag(", "), parse_dice)(input)?;
    let (input, color) = parse_color(input)?;
    return Ok((input, Round { sets }));
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, game_id) = complete::u32(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, rounds) = separated_list1(tag("; "), parse_round)(input)?;
    return Ok((input, Game { game_id, rounds }));
}

fn main() {
    let reader = read_arg_file().unwrap();
    let mut games: Vec<Game> = Vec::new();
    for line in reader.lines() {
        //println!("line: {}", line.unwrap());
        let line_cpy = (line.unwrap()).clone();
        //println!("line: {}", line_cpy);
        if let Ok((_, game)) = parse_game(&line_cpy) {
            games.push(game);
            // println!("game: {:?}", game);
        }
    }
    let mut sum = 0;
    let mut minimal_cubes_sum = 0;
    for game in games {
        if validate_game(&game) {
            sum += game.game_id;
        }
        let (min_red, min_green, min_blue) = minimal_cubes(&game);
        let minimal_cubes_product = min_red * min_green * min_blue;
        minimal_cubes_sum += minimal_cubes_product;
    }
    println!("sum: {}, minimal_cubes_sum: {}", sum, minimal_cubes_sum);
}

fn validate_game(game: &Game) -> bool {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    for round in game.rounds.iter() {
        for (value, color) in round.sets.iter() {
            if *color == Color::RED {
                if *value > max_red {
                    return false;
                }
            }
            if *color == Color::GREEN {
                if *value > max_green {
                    return false;
                }
            }
            if *color == Color::BLUE {
                if *value > max_blue {
                    return false;
                }
            }
        }
    }
    return true;
}

fn minimal_cubes(game: &Game) -> (u32, u32, u32) {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    for round in game.rounds.iter() {
        for (value, color) in round.sets.iter() {
            if *color == Color::RED {
                if *value > min_red {
                    min_red = *value;
                }
            }
            if *color == Color::GREEN {
                if *value > min_green {
                    min_green = *value;
                }
            }
            if *color == Color::BLUE {
                if *value > min_blue {
                    min_blue = *value
                }
            }
        }
    }
    return (min_red, min_green, min_blue);
}
