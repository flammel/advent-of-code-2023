use std::io;

fn main() -> io::Result<()> {
    let sum = io::stdin().lines().fold(0, |acc, line| {
        acc + line_value(&line.unwrap_or("".to_string()))
    });

    print!("{}", sum);

    Ok(())
}

fn line_value(line: &str) -> u32 {
    let lookup = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let first = lookup
        .iter()
        .filter_map(|(key, value)| line.find(key).map(|idx| (idx, value)))
        .min_by_key(|(idx, _)| *idx)
        .map_or(0, |(_, value)| *value);
    let last = lookup
        .iter()
        .filter_map(|(key, value)| line.rfind(key).map(|idx| (idx, value)))
        .max_by_key(|(idx, _)| *idx)
        .map_or(0, |(_, value)| *value);

    return first * 10 + last;
}

#[test]
fn test_line_value() -> () {
    assert_eq!(29, line_value("two1nine"));
    assert_eq!(83, line_value("eightwothree"));
    assert_eq!(13, line_value("abcone2threexyz"));
    assert_eq!(24, line_value("xtwone3four"));
    assert_eq!(42, line_value("4nineeightseven2"));
    assert_eq!(14, line_value("zoneight234"));
    assert_eq!(76, line_value("7pqrstsixteen"));
}
