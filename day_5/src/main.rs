use aoc_lib::read_arg_file;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, newline, not_line_ending},
    error::{Error, ParseError},
    multi::{count, many1, many_till, separated_list1},
    IResult,
};
use std::io::{self, prelude::*, BufReader};

fn parse_seeds(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(many1(tag(" ")), complete::i32)(input)?;
    let (input, _) = many1(tag("\n"))(input)?;
    Ok((input, seeds))
}

fn parse_3tuple(input: &str) -> IResult<&str, (i32, i32, i32)> {
    let (input, first) = complete::i32(input)?;
    let (input, _) = many1(tag(" "))(input)?;
    let (input, second) = complete::i32(input)?;
    let (input, _) = many1(tag(" "))(input)?;
    let (input, third) = complete::i32(input)?;
    Ok((input, (first, second, third)))
}

fn parse_map(input: &str) -> IResult<&str, Vec<(i32, i32, i32)>> {
    let (input, _) = tag("map_name")(input)?;
    let (input, _) = tag(":\n ")(input)?;
    let (input, map) = separated_list1(tag("\n"), parse_3tuple)(input)?;
    let (input, _) = many1(tag("\n"))(input)?;
    Ok((input, Vec::new()))
}

fn parse_seeds_to_soil_map(input: &str) -> IResult<&str, Vec<(i32, i32, i32)>> {
    let (input, _) = tag("seed-to-soil map")(input)?;
    let (input, _) = tag(":\n")(input)?;
    let (input, map) = separated_list1(tag("\n"), parse_3tuple)(input)?;
    let (input, _) = many1(tag("\n"))(input)?;
    Ok((input, map))
}

fn parse_soil_to_fertilizer_map(input: &str) -> IResult<&str, Vec<(i32, i32, i32)>> {
    let (input, _) = tag("soil-to-fertilizer map")(input)?;
    let (input, _) = tag(":\n")(input)?;
    let (input, map) = separated_list1(tag("\n"), parse_3tuple)(input)?;
    let (input, _) = many1(tag("\n"))(input)?;
    Ok((input, map))
}

fn parse_fertilizer_to_water_map(input: &str) -> IResult<&str, Vec<(i32, i32, i32)>> {
    let (input, _) = tag("fertilizer-to-water map")(input)?;
    let (input, _) = tag(":\n")(input)?;
    let (input, map) = separated_list1(tag("\n"), parse_3tuple)(input)?;
    let (input, _) = many1(tag("\n"))(input)?;
    Ok((input, map))
}

fn parse_water_to_light_map(input: &str) -> IResult<&str, Vec<(i32, i32, i32)>> {
    let (input, _) = tag("water-to-light map")(input)?;
    let (input, _) = tag(":\n")(input)?;
    let (input, map) = separated_list1(tag("\n"), parse_3tuple)(input)?;
    let (input, _) = many1(tag("\n"))(input)?;
    Ok((input, map))
}

fn parse_light_to_temperature_map(input: &str) -> IResult<&str, Vec<(i32, i32, i32)>> {
    let (input, _) = tag("light-to-temperature map")(input)?;
    let (input, _) = tag(":\n")(input)?;
    let (input, map) = separated_list1(tag("\n"), parse_3tuple)(input)?;
    let (input, _) = many1(tag("\n"))(input)?;
    Ok((input, map))
}

fn parse_temperature_to_humidity_map(input: &str) -> IResult<&str, Vec<(i32, i32, i32)>> {
    let (input, _) = tag("temperature-to-humidity map")(input)?;
    let (input, _) = tag(":\n")(input)?;
    let (input, map) = separated_list1(tag("\n"), parse_3tuple)(input)?;
    let (input, _) = many1(tag("\n"))(input)?;
    Ok((input, map))
}

fn parse_humidity_to_location_map(input: &str) -> IResult<&str, Vec<(i32, i32, i32)>> {
    let (input, _) = tag("humidity-to-location map")(input)?;
    let (input, _) = tag(":\n")(input)?;
    let (input, map) = separated_list1(tag("\n"), parse_3tuple)(input)?;
    let (input, _) = many1(tag("\n"))(input)?;
    Ok((input, map))
}

fn follow_mapping_chain(seeds: Vec<i32>, mappings: Vec<Vec<(i32, i32, i32)>>) {
    let mut mapped_seeds = seeds.clone();
    let mut next_mapped_seeds = Vec::new();

    for maps in mappings.iter() {
        println!("start:{:?}", mapped_seeds);
        for seed in mapped_seeds.iter_mut() {
            let mut not_mapped = true;
            for (dest_start, src_start, range) in maps.iter() {
                if *seed >= *src_start && *seed < *src_start + *range {
                    next_mapped_seeds.push(*seed + *dest_start - src_start);
                    not_mapped = false;
                    break;
                }
            }
            if (not_mapped) {
                next_mapped_seeds.push(*seed);
            }
        }
        println!("mapped_seeds: {:?}", mapped_seeds);
        println!("next_mapped_seeds: {:?}", next_mapped_seeds);

        mapped_seeds.clear();
        mapped_seeds.append(&mut next_mapped_seeds);
    }
    println!("end:{:?}", mapped_seeds);
}

fn main() {
    let mut reader = read_arg_file().unwrap();
    let mut input = String::new();
    if let Ok(string_size) = reader.read_to_string(&mut input) {
        let (input, seeds) = parse_seeds(&input).unwrap();
        let (input, seeds_to_soil_map) = parse_seeds_to_soil_map(input).unwrap();
        let (input, soil_to_fertilizer_map) = parse_soil_to_fertilizer_map(input).unwrap();
        let (input, fertilizer_to_water_map) = parse_fertilizer_to_water_map(input).unwrap();
        let (input, water_to_light_map) = parse_water_to_light_map(input).unwrap();
        let (input, light_to_temperature_map) = parse_light_to_temperature_map(input).unwrap();
        let (input, temperature_to_humidity_map) =
            parse_temperature_to_humidity_map(input).unwrap();
        let (input, humidity_to_location_map) = parse_humidity_to_location_map(input).unwrap();
        println!("{:?}", seeds);
        println!("{:?}", seeds_to_soil_map);
        println!("{:?}", soil_to_fertilizer_map);
        println!("{:?}", fertilizer_to_water_map);
        println!("{:?}", water_to_light_map);
        println!("{:?}", light_to_temperature_map);
        println!("{:?}", temperature_to_humidity_map);
        println!("{:?}", humidity_to_location_map);

        let mappings = vec![
            seeds_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        ];
        follow_mapping_chain(seeds, mappings);
    }
}
