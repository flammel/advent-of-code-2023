use std::{collections::HashMap, io, str::Lines};

fn main() -> io::Result<()> {
    let raw_input = io::read_to_string(io::stdin())?;
    let input = parse_input(raw_input.lines());
    print!("{}", get_result(&input));

    return Ok(());
}

#[derive(Debug, PartialEq, Eq)]
struct Input {
    steps: Vec<Direction>,
    network: HashMap<String, (String, String)>,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

fn get_result(input: &Input) -> u64 {
    let mut steps = 0;
    let mut current = "AAA";
    for step in input.steps.iter().cycle() {
        let (left, right) = input.network.get(current).unwrap();
        current = match step {
            Direction::Left => left,
            Direction::Right => right,
        };
        steps = steps + 1;
        if current == "ZZZ" {
            return steps;
        }
    }
    return 0;
}

fn parse_input(mut lines: Lines) -> Input {
    let steps = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        })
        .collect::<Vec<Direction>>();
    lines.next();
    return Input {
        steps,
        network: lines.map(|line| parse_line(line)).collect(),
    };
}

fn parse_line(line: &str) -> (String, (String, String)) {
    let mut parts = line.split(" = ");
    let key = parts.next().unwrap();
    let value = parts.next().unwrap();
    let mut value_parts = value.split(", ");
    let left = value_parts.next().unwrap().replace("(", "");
    let right = value_parts.next().unwrap().replace(")", "");
    return (String::from(key), (left, right));
}

#[test]
fn test_parse_input() -> () {
    let input = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)";
    assert_eq!(
        Input {
            steps: vec![Direction::Right, Direction::Left],
            network: vec![
                ("AAA", ("BBB", "CCC")),
                ("BBB", ("DDD", "EEE")),
                ("CCC", ("ZZZ", "GGG")),
                ("DDD", ("DDD", "DDD")),
                ("EEE", ("EEE", "EEE")),
                ("GGG", ("GGG", "GGG")),
                ("ZZZ", ("ZZZ", "ZZZ")),
            ]
            .iter()
            .map(|(a, (b, c))| (a.to_string(), (b.to_string(), c.to_string())))
            .collect(),
        },
        parse_input(input.lines())
    )
}

#[test]
fn test_get_result_1() -> () {
    let input = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)";
    assert_eq!(2, get_result(&parse_input(input.lines())));
}

#[test]
fn test_get_result_2() -> () {
    let input = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";
    assert_eq!(6, get_result(&parse_input(input.lines())));
}
