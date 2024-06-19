advent_of_code::solution!(4);

use itertools::iproduct;
use regex::Regex;
use std::cell::Cell;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    numbers: Vec<u32>,
    copies: Cell<u32>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;

impl Card {
    fn new(id: u32, winning: Vec<u32>, numbers: Vec<u32>) -> Card {
        Card {
            id,
            winning,
            numbers,
            copies: Cell::new(1),
        }
    }

    fn winning_numbers(&self) -> usize {
        iproduct!(&self.winning, &self.numbers)
            .filter(|(i, j)| i == j)
            .count()
    }

    fn points(&self) -> u32 {
        let winning_numbers = self.winning_numbers();
        if winning_numbers > 0 {
            2u32.pow(winning_numbers as u32 - 1)
        } else {
            0
        }
    }

    fn add_copy(&self) {
        self.copies.set(self.copies.get() + 1);
    }
}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let card_regex = Regex::new(r"Card\s+(?<id>\d+):(?<winning>.*)\|(?<numbers>.*)").unwrap();
        let number_regex = Regex::new(r"(\d+)").unwrap();

        let caps = card_regex.captures(s).ok_or(ParseCardError)?;

        let id = caps["id"].parse::<u32>().map_err(|_| ParseCardError)?;

        let winning: Vec<Result<u32, ParseCardError>> = number_regex
            .captures_iter(&caps["winning"])
            .map(|cap| cap[1].parse::<u32>().map_err(|_| ParseCardError))
            .collect();
        if winning.iter().any(|x| x.is_err()) {
            return Err(ParseCardError);
        }
        let winning = winning.into_iter().map(|x| x.unwrap()).collect();

        let numbers: Vec<Result<u32, ParseCardError>> = number_regex
            .captures_iter(&caps["numbers"])
            .map(|cap| cap[1].parse::<u32>().map_err(|_| ParseCardError))
            .collect();
        if numbers.iter().any(|x| x.is_err()) {
            return Err(ParseCardError);
        }
        let numbers = numbers.into_iter().map(|x| x.unwrap()).collect();

        let card = Card::new(id, winning, numbers);
        Ok(card)
    }
}

#[derive(Debug, PartialEq)]
struct Problem {
    cards: Vec<Card>,
}

impl Problem {
    fn get(&self, id: usize) -> Option<&Card> {
        if id > 0 {
            self.cards.get(id - 1)
        } else {
            None
        }
    }

    fn total_points(&self) -> u32 {
        self.cards.iter().map(|card| card.points()).sum()
    }

    fn process_copies(&self) {
        for card in self.cards.iter() {
            let matching = card.winning_numbers() as u32;
            for id in (card.id + 1)..(card.id + 1 + matching) {
                if let Some(next) = self.get(id as usize) {
                    for _ in 0..(card.copies.get()) {
                        next.add_copy()
                    }
                }
            }
        }
    }

    fn total_copies(&self) -> u32 {
        self.cards.iter().map(|card| card.copies.get()).sum()
    }
}

impl FromStr for Problem {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: Vec<Result<Card, ParseCardError>> = s
            .split_terminator('\n')
            .into_iter()
            .map(|line| line.parse::<Card>())
            .collect();
        if cards.iter().any(|x| x.is_err()) {
            return Err(ParseCardError);
        }
        let cards = cards.into_iter().map(|x| x.unwrap()).collect();

        let problem = Problem { cards };
        Ok(problem)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let problem = input.parse::<Problem>().unwrap();
    Some(problem.total_points())
}

pub fn part_two(input: &str) -> Option<u32> {
    let problem = input.parse::<Problem>().unwrap();
    problem.process_copies();
    Some(problem.total_copies())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(30));
    }
}
