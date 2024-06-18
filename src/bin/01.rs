advent_of_code::solution!(1);

fn get_digits(input: &str) -> u32 {
    let digits: Vec<char> = input.chars().filter(|c| c.is_numeric()).collect();
    let first = digits.first().unwrap().to_digit(10).unwrap();
    let last = digits.last().unwrap().to_digit(10).unwrap();
    first * 10 + last
}

fn get_words(input: &str) -> u32 {
    let patterns = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let (_, first) = patterns
        .iter()
        .enumerate()
        .map(|(i, p)| (input.find(p), i))
        .filter(|(n, _)| n.is_some())
        .map(|(n, i)| (n.unwrap(), i))
        .fold(None, |acc, item| {
            let min = acc.unwrap_or(item);
            Some(min.min(item))
        })
        .unwrap();
    let (_, last) = patterns
        .iter()
        .enumerate()
        .map(|(i, p)| (input.rfind(p), i))
        .filter(|(n, _)| n.is_some())
        .map(|(n, i)| (n.unwrap(), i))
        .fold(None, |acc, item| {
            let max = acc.unwrap_or(item);
            Some(max.max(item))
        })
        .unwrap();
    (first % 9 + 1) as u32 * 10 + (last % 9 + 1) as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let v: u32 = input
        .split_terminator('\n')
        .into_iter()
        .map(|s| get_digits(s))
        .sum();
    Some(v)
}

pub fn part_two(input: &str) -> Option<u32> {
    let v: u32 = input
        .split_terminator('\n')
        .into_iter()
        .map(|s| get_words(s))
        .sum();
    Some(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
