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
    return symbols
        .iter()
        .map(|s| get_adjacent_numbers(s, &numbers))
        .filter(|ns| ns.len() == 2)
        .map(|ns| ns[0] * ns[1])
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
            end_index: m.end().checked_sub(1).unwrap_or(m.end()),
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
    return Regex::new(r"\*")
        .unwrap()
        .find_iter(line)
        .map(|m| Sym {
            line_index: line_index,
            index: m.start(),
        })
        .collect();
}

fn get_adjacent_numbers(symbol: &Sym, numbers: &Vec<Num>) -> Vec<u32> {
    return numbers
        .iter()
        .filter(|n| is_adjacent(symbol, n))
        .map(|n| n.value)
        .collect();
}

fn is_adjacent(symbol: &Sym, num: &Num) -> bool {
    let min_li = symbol
        .line_index
        .checked_sub(1)
        .unwrap_or(symbol.line_index);
    let max_li = symbol.line_index + 1;

    let min_i = symbol.index.checked_sub(1).unwrap_or(symbol.index);
    let max_i = symbol.index + 1;

    return num.line_index >= min_li
        && num.line_index <= max_li
        && num.start_index <= max_i
        && num.end_index >= min_i;
}

#[test]
fn test_get_result() -> () {
    assert_eq!(0, get_result("..2..\n..*..\n.....".lines()));
    assert_eq!(0, get_result("..2..\n..*..\n.3.5.".lines()));
    assert_eq!(10, get_result("..2..\n..*..\n...5.".lines()));
    assert_eq!(6, get_result("..2..\n..*..\n.3...".lines()));
    assert_eq!(15, get_result(".....\n..*..\n.3.5.".lines()));
    assert_eq!(11 * 13, get_result("11...\n..*..\n...13".lines()));
    assert_eq!(11 * 13, get_result("...11\n..*..\n13...".lines()));
    assert_eq!(
        467 * 35,
        get_result("467..114..\n...*......\n..35..633.".lines()),
    );
    assert_eq!(
        755 * 598,
        get_result("......755.\n...$.*....\n.664.598..".lines()),
    );
    assert_eq!(
        467835,
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
