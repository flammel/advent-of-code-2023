use std::io;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Input {
    histories: Vec<Vec<i64>>,
}

fn main() -> io::Result<()> {
    let raw_input = io::read_to_string(io::stdin())?;
    let input = parse_input(raw_input);
    print!("{}", get_result(&input));

    return Ok(());
}

fn parse_input(raw_input: String) -> Input {
    return Input {
        histories: raw_input
            .lines()
            .map(|line| {
                line.split(" ")
                    .map(|item| item.parse::<i64>().unwrap())
                    .collect()
            })
            .collect(),
    };
}

fn get_result(input: &Input) -> i64 {
    return input
        .histories
        .iter()
        .map(|history| get_result_for_history(history))
        .sum();
}

fn get_result_for_history(history: &Vec<i64>) -> i64 {
    let mut sequences = vec![];
    let mut seq = history.clone();
    while !seq.iter().all_equal_value().eq(&Ok(&0)) {
        sequences.push(seq.clone());
        seq = next_seq(&seq);
    }

    sequences.reverse();

    return sequences
        .iter()
        .fold(0, |acc, seq| acc + seq.last().unwrap());
}

fn next_seq(seq: &Vec<i64>) -> Vec<i64> {
    let mut next_seq = vec![];
    for idx in 1..seq.len() {
        next_seq.push(seq[idx] - seq[idx - 1]);
    }
    return next_seq;
}

#[test]
fn test_parse_input() -> () {
    let input = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";
    assert_eq!(
        Input {
            histories: vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![1, 3, 6, 10, 15, 21],
                vec![10, 13, 16, 21, 30, 45]
            ]
        },
        parse_input(input.to_string())
    )
}

#[test]
fn test_get_result() -> () {
    let input = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";
    assert_eq!(114, get_result(&parse_input(input.to_string())));
}

#[test]
fn test_next_seq() -> () {
    assert_eq!(vec![3, 3, 3, 3, 3], next_seq(&vec![0, 3, 6, 9, 12, 15]));
}
