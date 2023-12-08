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
    seeds: Vec<u64>,
    maps: Vec<CategoryMap>,
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct CategoryMap {
    from: String,
    to: String,
    entries: Vec<CategoryMapEntry>,
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
        .iter()
        .map(|seed| get_location(*seed, &almanac))
        .min()
        .unwrap();
}

fn get_location(seed: u64, almanac: &Almanac) -> u64 {
    let mut category = "seed";
    let mut location = seed;
    while category != "location" {
        for map in &almanac.maps {
            if map.from == category {
                category = &map.to;
                location = apply_mapping(location, map)
            }
        }
    }
    return location;
}

fn apply_mapping(value: u64, map: &CategoryMap) -> u64 {
    for entry in map.entries.iter() {
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

    let mut maps = vec![];
    for entry in tmp_map {
        maps.push(CategoryMap {
            from: entry.0.trim().split("-to-").next().unwrap().to_string(),
            to: entry
                .0
                .trim()
                .split("-to-")
                .last()
                .unwrap()
                .split(" ")
                .next()
                .unwrap()
                .to_string(),
            entries: entry
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
                .collect(),
        })
    }

    return Almanac {
        seeds: seeds,
        maps: maps,
    };
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
        seeds: vec![79, 14, 55, 13],
        maps: vec![
            CategoryMap {
                from: "seed".to_string(),
                to: "soil".to_string(),
                entries: vec![
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
            },
            CategoryMap {
                from: "soil".to_string(),
                to: "fertilizer".to_string(),
                entries: vec![
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
            },
        ],
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
    assert_eq!(35, get_lowest_location(get_almanac(test_input.lines())));
}
