use itertools::Itertools;
use rayon::prelude::*;
use std::{collections::HashMap, io, str::Lines, vec};

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let almanac = get_almanac(input.lines());
    let lowest_location = get_lowest_location(almanac);
    print!("{}", lowest_location);

    return Ok(());
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Almanac {
    seeds: Vec<(u64, u64)>,
    seed_to_soil: Vec<CategoryMapEntry>,
    soil_to_fertilizer: Vec<CategoryMapEntry>,
    fertilizer_to_water: Vec<CategoryMapEntry>,
    water_to_light: Vec<CategoryMapEntry>,
    light_to_temperature: Vec<CategoryMapEntry>,
    temperature_to_humidity: Vec<CategoryMapEntry>,
    humidity_to_location: Vec<CategoryMapEntry>,
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct CategoryMapEntry {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

fn get_lowest_location(almanac: Almanac) -> u64 {
    return almanac
        .seeds
        .par_iter()
        .filter_map(|seed| {
            (seed.0..(seed.0 + seed.1))
                .into_par_iter()
                .map(|s| get_location(s, &almanac))
                .min()
        })
        .min()
        .unwrap();
}

fn get_location(seed: u64, almanac: &Almanac) -> u64 {
    let soil = apply_mapping(seed, &almanac.seed_to_soil);
    let fertilizer = apply_mapping(soil, &almanac.soil_to_fertilizer);
    let water = apply_mapping(fertilizer, &almanac.fertilizer_to_water);
    let light = apply_mapping(water, &almanac.water_to_light);
    let temperature = apply_mapping(light, &almanac.light_to_temperature);
    let humidity = apply_mapping(temperature, &almanac.temperature_to_humidity);
    let location = apply_mapping(humidity, &almanac.humidity_to_location);
    return location;
}

fn apply_mapping(value: u64, mappings: &Vec<CategoryMapEntry>) -> u64 {
    for entry in mappings {
        if value >= entry.source_range_start
            && value < entry.source_range_start + entry.range_length
        {
            return entry.destination_range_start + (value - entry.source_range_start);
        }
    }
    return value;
}

fn get_almanac(lines: Lines) -> Almanac {
    let mut seeds = vec![];
    let mut key: Option<&str> = None;
    let mut tmp_map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines {
        if line.trim().starts_with("seeds: ") {
            seeds = line
                .trim()
                .split(" ")
                .filter_map(|s| s.parse::<u64>().ok())
                .tuples()
                .collect();
            continue;
        }
        if line.trim().ends_with(" map:") {
            key = Some(line.trim());
            continue;
        }
        if !line.trim().is_empty() && key.is_some() {
            tmp_map
                .entry(key.unwrap())
                .or_insert(vec![])
                .push(line.trim());
        }
    }

    let mut almanac = Almanac {
        seeds: seeds,
        seed_to_soil: vec![],
        soil_to_fertilizer: vec![],
        fertilizer_to_water: vec![],
        water_to_light: vec![],
        light_to_temperature: vec![],
        temperature_to_humidity: vec![],
        humidity_to_location: vec![],
    };

    for entry in tmp_map {
        let entries = entry
            .1
            .iter()
            .map(|line| {
                let nums: Vec<u64> = line
                    .trim()
                    .split(" ")
                    .filter_map(|s| s.parse::<u64>().ok())
                    .collect();
                return CategoryMapEntry {
                    destination_range_start: nums[0],
                    source_range_start: nums[1],
                    range_length: nums[2],
                };
            })
            .collect();
        match entry.0 {
            "seed-to-soil map:" => almanac.seed_to_soil = entries,
            "soil-to-fertilizer map:" => almanac.soil_to_fertilizer = entries,
            "fertilizer-to-water map:" => almanac.fertilizer_to_water = entries,
            "water-to-light map:" => almanac.water_to_light = entries,
            "light-to-temperature map:" => almanac.light_to_temperature = entries,
            "temperature-to-humidity map:" => almanac.temperature_to_humidity = entries,
            "humidity-to-location map:" => almanac.humidity_to_location = entries,
            _ => (),
        }
    }

    return almanac;
}

#[test]
fn test_get_almanac() -> () {
    let test_input = "
    seeds: 79 14 55 13
    
    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    ";
    let expected = Almanac {
        seeds: vec![(79, 14), (55, 13)],
        seed_to_soil: vec![
            CategoryMapEntry {
                destination_range_start: 50,
                source_range_start: 98,
                range_length: 2,
            },
            CategoryMapEntry {
                destination_range_start: 52,
                source_range_start: 50,
                range_length: 48,
            },
        ],
        soil_to_fertilizer: vec![
            CategoryMapEntry {
                destination_range_start: 0,
                source_range_start: 15,
                range_length: 37,
            },
            CategoryMapEntry {
                destination_range_start: 37,
                source_range_start: 52,
                range_length: 2,
            },
            CategoryMapEntry {
                destination_range_start: 39,
                source_range_start: 0,
                range_length: 15,
            },
        ],
        fertilizer_to_water: vec![],
        water_to_light: vec![],
        light_to_temperature: vec![],
        temperature_to_humidity: vec![],
        humidity_to_location: vec![],
    };
    assert_eq!(expected, get_almanac(test_input.lines()));
}

#[test]
fn test_get_lowest_location() -> () {
    let test_input = "
    seeds: 79 14 55 13
    
    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4
    ";
    assert_eq!(46, get_lowest_location(get_almanac(test_input.lines())));
}
