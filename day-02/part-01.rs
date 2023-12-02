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
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let game_id = game_id(line);
    let cc = cube_count(line);
    if (cc.0 > max_red) || (cc.1 > max_green) || (cc.2 > max_blue) {
        return 0;
    } else {
        return game_id;
    }
}

fn game_id(line: &str) -> u32 {
    return Regex::new(r"^Game ([0-9]+):")
        .unwrap()
        .captures(line)
        .unwrap()
        .get(1)
        .map_or(0, |m| m.as_str().parse::<u32>().unwrap_or(0));
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
fn test_game_id() -> () {
    assert_eq!(
        1,
        game_id("Game 1: 13 green, 3 red; 4 red, 9 green, 4 blue; 9 green, 10 red, 2 blue")
    );
    assert_eq!(93, game_id("Game 93: 3 blue; 8 blue; 3 blue, 2 red; 2 red"));
    assert_eq!(
        100,
        game_id("Game 100: 5 red, 9 green, 2 blue; 9 blue, 6 green, 1 red; 8 blue, 7 green, 3 red")
    );
}

#[test]
fn test_cube_count() -> () {
    assert_eq!(
        (8, 2, 4),
        cube_count("Game 11: 8 red, 4 blue, 1 green; 3 red; 1 green; 2 green, 3 blue")
    );
    assert_eq!((4, 12, 8), cube_count("Game 70: 1 green, 8 blue, 2 red; 2 red, 10 green, 1 blue; 1 red, 12 green, 6 blue; 9 green, 4 blue, 4 red; 2 red, 6 green; 3 red, 8 green, 6 blue"));
    assert_eq!(
        (2, 1, 8),
        cube_count("Game 93: 3 blue; 8 blue; 3 blue, 2 red; 2 red, 1 green")
    );
    assert_eq!(
        (2, 0, 8),
        cube_count("Game 93: 3 blue; 8 blue; 3 blue, 2 red; 2 red")
    );
}
