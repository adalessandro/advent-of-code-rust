pub mod template;

use std::num::ParseIntError;

pub fn numbers_from_str(s: &str) -> Result<Vec<usize>, ParseIntError> {
    let mut numbers: Vec<usize> = vec![];
    for number in s.split_whitespace() {
        numbers.push(number.parse::<usize>()?);
    }
    Ok(numbers)
}

pub fn number_from_str(s: &str) -> Result<usize, ParseIntError> {
    s.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<usize>()
}
