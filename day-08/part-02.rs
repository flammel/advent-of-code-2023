use num::integer::lcm;
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
    let mut result = 1;
    for node in input.network.keys() {
        if node.ends_with("A") {
            let mut steps = 0;
            let mut current = node;
            for step in input.steps.iter().cycle() {
                let (left, right) = input.network.get(current).unwrap();
                current = match step {
                    Direction::Left => left,
                    Direction::Right => right,
                };
                steps = steps + 1;
                if current.ends_with("Z") {
                    result = lcm(result, steps);
                    break;
                }
            }
        }
    }
    return result;
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
fn test_get_result_1() -> () {
    let input = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)\n";
    assert_eq!(6, get_result(&parse_input(input.lines())));
}
