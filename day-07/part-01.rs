use std::{io, ops::Mul, str::Lines};

use itertools::Itertools;

fn main() -> io::Result<()> {
    let raw_input = io::read_to_string(io::stdin())?;
    let input = parse_input(raw_input.lines());
    print!("{}", get_result(&input));

    return Ok(());
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Input {
    hands: Vec<(Hand, u64)>,
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Hand {
    card_1: u32,
    card_2: u32,
    card_3: u32,
    card_4: u32,
    card_5: u32,
    hand_type: HandType,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self
            .hand_type
            .cmp(&other.hand_type)
            .then(self.card_1.cmp(&other.card_1))
            .then(self.card_2.cmp(&other.card_2))
            .then(self.card_3.cmp(&other.card_3))
            .then(self.card_4.cmp(&other.card_4))
            .then(self.card_5.cmp(&other.card_5));
    }
}

fn get_result(input: &Input) -> u64 {
    return input
        .hands
        .iter()
        .sorted_by_key(|h| &h.0)
        .enumerate()
        .map(|(i, h)| ((i + 1) as u64).mul(h.1))
        .sum();
}

fn parse_input(lines: Lines) -> Input {
    return Input {
        hands: lines.map(|line| parse_hand_and_bid(line)).collect(),
    };
}

fn parse_hand_and_bid(line: &str) -> (Hand, u64) {
    let mut parts = line.split(" ");
    let mut hand = parts.next().unwrap().chars();
    let bid = parts.next().unwrap().parse::<u64>().unwrap();
    let card_1 = parse_card(hand.next().unwrap());
    let card_2 = parse_card(hand.next().unwrap());
    let card_3 = parse_card(hand.next().unwrap());
    let card_4 = parse_card(hand.next().unwrap());
    let card_5 = parse_card(hand.next().unwrap());
    return (
        Hand {
            card_1,
            card_2,
            card_3,
            card_4,
            card_5,
            hand_type: get_hand_type(card_1, card_2, card_3, card_4, card_5),
        },
        bid,
    );
}

fn get_hand_type(card_1: u32, card_2: u32, card_3: u32, card_4: u32, card_5: u32) -> HandType {
    let mut vals: Vec<u32> = vec![0; 15];
    vals[card_1 as usize] += 1;
    vals[card_2 as usize] += 1;
    vals[card_3 as usize] += 1;
    vals[card_4 as usize] += 1;
    vals[card_5 as usize] += 1;
    vals.sort_by(|a, b| b.cmp(a));
    return match (vals[0], vals[1], vals[2]) {
        (5, _, _) => HandType::FiveOfAKind,
        (4, _, _) => HandType::FourOfAKind,
        (3, 2, _) => HandType::FullHouse,
        (3, _, _) => HandType::ThreeOfAKind,
        (2, 2, _) => HandType::TwoPair,
        (2, _, _) => HandType::OnePair,
        (_, _, _) => HandType::HighCard,
    };
}

fn parse_card(card: char) -> u32 {
    return match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
    };
}

#[test]
fn test_parse_input() -> () {
    let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
    assert_eq!(
        Input {
            hands: vec![
                (
                    Hand {
                        card_1: 3,
                        card_2: 2,
                        card_3: 10,
                        card_4: 3,
                        card_5: 13,
                        hand_type: HandType::OnePair,
                    },
                    765
                ),
                (
                    Hand {
                        card_1: 10,
                        card_2: 5,
                        card_3: 5,
                        card_4: 11,
                        card_5: 5,
                        hand_type: HandType::ThreeOfAKind,
                    },
                    684
                ),
                (
                    Hand {
                        card_1: 13,
                        card_2: 13,
                        card_3: 6,
                        card_4: 7,
                        card_5: 7,
                        hand_type: HandType::TwoPair,
                    },
                    28
                ),
                (
                    Hand {
                        card_1: 13,
                        card_2: 10,
                        card_3: 11,
                        card_4: 11,
                        card_5: 10,
                        hand_type: HandType::TwoPair,
                    },
                    220
                ),
                (
                    Hand {
                        card_1: 12,
                        card_2: 12,
                        card_3: 12,
                        card_4: 11,
                        card_5: 14,
                        hand_type: HandType::ThreeOfAKind,
                    },
                    483
                )
            ]
        },
        parse_input(input.lines())
    )
}

#[test]
fn test_get_result() -> () {
    let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
    assert_eq!(6440, get_result(&parse_input(input.lines())));
}
