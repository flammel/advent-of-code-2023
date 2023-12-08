use std::{io, str::Lines};

fn main() -> io::Result<()> {
    let raw_input = io::read_to_string(io::stdin())?;
    let input = parse_input(raw_input.lines());
    print!("{}", get_result(input));

    return Ok(());
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Input {
    races: Vec<Race>,
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn get_result(input: Input) -> u64 {
    return input.races.iter().map(|r| winnable_ways(r)).product();
}

fn winnable_ways(race: &Race) -> u64 {
    let mut ways = 0;
    for holding_ms in 0..(race.time - 1) {
        if (race.time - holding_ms) * holding_ms > race.distance {
            ways += 1;
        }
    }
    return ways;
}

fn parse_input(mut lines: Lines) -> Input {
    let times = lines
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|s| s.parse::<u64>().ok());
    let distances = lines
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|s| s.parse::<u64>().ok());
    return Input {
        races: times
            .zip(distances)
            .map(|(t, d)| Race {
                time: t,
                distance: d,
            })
            .collect(),
    };
}

#[test]
fn test_parse_input() -> () {
    let input = "Time:      7  15   30\nDistance:  9  40  200";
    assert_eq!(
        Input {
            races: vec![
                Race {
                    time: 7,
                    distance: 9
                },
                Race {
                    time: 15,
                    distance: 40
                },
                Race {
                    time: 30,
                    distance: 200
                }
            ]
        },
        parse_input(input.lines())
    )
}

#[test]
fn test_winnable_ways() -> () {
    assert_eq!(
        4,
        winnable_ways(&Race {
            time: 7,
            distance: 9
        })
    );
    assert_eq!(
        8,
        winnable_ways(&Race {
            time: 15,
            distance: 40
        })
    );
    assert_eq!(
        9,
        winnable_ways(&Race {
            time: 30,
            distance: 200
        })
    );
}
