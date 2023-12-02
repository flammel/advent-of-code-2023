use regex::Regex;
use std::io;

fn main() -> io::Result<()> {
    let sum: u32 = io::stdin()
        .lines()
        .map(|line| line_value(&line.unwrap_or_default()))
        .sum();

    print!("{}", sum);

    return Ok(());
}

fn line_value(line: &str) -> u32 {
    let (red, green, blue) = cube_count(line);
    return red * green * blue;
}

fn cube_count(line: &str) -> (u32, u32, u32) {
    let red = Regex::new(r" ([0-9]+) red")
        .unwrap()
        .captures_iter(line)
        .map(|c| c.extract())
        .map(|(_, [c])| c)
        .filter_map(|m| m.parse::<u32>().ok())
        .max()
        .unwrap_or(0);
    let green = Regex::new(r" ([0-9]+) green")
        .unwrap()
        .captures_iter(line)
        .map(|c| c.extract())
        .map(|(_, [c])| c)
        .filter_map(|m| m.parse::<u32>().ok())
        .max()
        .unwrap_or(0);
    let blue = Regex::new(r" ([0-9]+) blue")
        .unwrap()
        .captures_iter(line)
        .map(|c| c.extract())
        .map(|(_, [c])| c)
        .filter_map(|m| m.parse::<u32>().ok())
        .max()
        .unwrap_or(0);
    return (red, green, blue);
}

#[test]
fn test_line_value() -> () {
    assert_eq!(
        48,
        line_value("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
    );
    assert_eq!(
        12,
        line_value("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
    );
    assert_eq!(
        1560,
        line_value("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")
    );
    assert_eq!(
        630,
        line_value("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red")
    );
    assert_eq!(
        36,
        line_value("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
    );
}
