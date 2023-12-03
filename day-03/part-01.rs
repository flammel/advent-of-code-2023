use regex::Regex;
use std::collections::HashSet;
use std::io;
use std::str::Lines;

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let sum = get_result(input.lines());
    print!("{}", sum);

    return Ok(());
}

fn get_result(lines: Lines) -> u32 {
    let symbols = get_symbols(lines.clone());
    let numbers = get_numbers(lines);
    return numbers
        .iter()
        .filter(|n| is_adjacent(&symbols, n))
        .map(|n| n.value)
        .sum();
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Sym {
    line_index: usize,
    index: usize,
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Num {
    line_index: usize,
    start_index: usize,
    end_index: usize,
    value: u32,
}

fn get_numbers(lines: Lines) -> Vec<Num> {
    return lines
        .enumerate()
        .flat_map(|(line_index, line)| get_numbers_in_line(line_index, line))
        .collect();
}

fn get_numbers_in_line(line_index: usize, line: &str) -> Vec<Num> {
    return Regex::new(r"[0-9]+")
        .unwrap()
        .find_iter(line)
        .map(|m| Num {
            line_index: line_index,
            start_index: m.start(),
            end_index: m.end(),
            value: m.as_str().parse::<u32>().unwrap(),
        })
        .collect();
}

fn get_symbols(lines: Lines) -> HashSet<Sym> {
    return lines
        .enumerate()
        .flat_map(|(line_index, line)| get_symbols_in_line(line_index, line))
        .collect();
}

fn get_symbols_in_line(line_index: usize, line: &str) -> HashSet<Sym> {
    return Regex::new(r"[^0-9.]")
        .unwrap()
        .find_iter(line)
        .map(|m| Sym {
            line_index: line_index,
            index: m.start(),
        })
        .collect();
}

fn is_adjacent(symbols: &HashSet<Sym>, num: &Num) -> bool {
    for i in num.start_index.checked_sub(1).unwrap_or(num.start_index)..(num.end_index + 1) {
        if symbols.contains(&Sym {
            line_index: num.line_index,
            index: i,
        }) || symbols.contains(&Sym {
            line_index: num.line_index + 1,
            index: i,
        }) || num
            .line_index
            .checked_sub(1)
            .map(|li| {
                symbols.contains(&Sym {
                    line_index: li,
                    index: i,
                })
            })
            .unwrap_or(false)
        {
            return true;
        }
    }
    return false;
}

#[test]
fn test_get_result() -> () {
    assert_eq!(
        4361,
        get_result(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
                .lines()
        )
    );
}
