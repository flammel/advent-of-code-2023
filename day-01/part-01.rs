use std::io;

fn main() -> io::Result<()> {
    let sum = io::stdin().lines().fold(0, |acc, line| {
        acc + line_value(&line.unwrap_or("".to_string()))
    });

    print!("{}", sum);

    return Ok(());
}

fn line_value(line: &str) -> u32 {
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    return digits.first().unwrap_or(&0) * 10 + digits.last().unwrap_or(&0);
}

#[test]
fn test_line_value() -> () {
    assert_eq!(12, line_value("1abc2"));
    assert_eq!(38, line_value("pqr3stu8vwx"));
    assert_eq!(15, line_value("a1b2c3d4e5f"));
    assert_eq!(77, line_value("treb7uchet"));
}
