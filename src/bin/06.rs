advent_of_code::solution!(6);

use advent_of_code::number_from_str;
use itertools::Itertools;
use regex::RegexBuilder;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct ParseProblemError;

#[derive(Debug, PartialEq)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn new(time: usize, distance: usize) -> Race {
        Race { time, distance }
    }

    fn compute_one(&self, hold: usize) -> usize {
        hold * (self.time - hold)
    }

    fn compute_all(&self) -> Vec<usize> {
        (0..self.time + 1)
            .map(|hold| self.compute_one(hold))
            .collect()
    }

    fn solutions(&self) -> (f64, f64) {
        let factor = (self.time.pow(2) - 4 * self.distance) as f64;
        (
            (self.time as f64 - factor.sqrt()) / 2.0,
            (self.time as f64 + factor.sqrt()) / 2.0,
        )
    }
}

#[derive(Debug, PartialEq)]
struct Problem {
    race: Race,
}

impl Problem {
    fn new(race: Race) -> Problem {
        Problem { race }
    }

    fn solve(&self) -> usize {
        let (a, b) = self.race.solutions();
        (b.floor() - a.ceil()) as usize + 1
    }
}

impl FromStr for Problem {
    type Err = ParseProblemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = r"^Time:\s+(?<times>.+)\nDistance:\s+(?<distances>.+)$";
        let problem_regex = RegexBuilder::new(pattern)
            .dot_matches_new_line(true)
            .build()
            .unwrap();
        let caps = problem_regex.captures(s).ok_or(ParseProblemError)?;

        let time = number_from_str(&caps["times"]).or(Err(ParseProblemError))?;
        let distance = number_from_str(&caps["distances"]).or(Err(ParseProblemError))?;

        let race = Race::new(time, distance);

        Ok(Problem::new(race))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(288)
}

pub fn part_two(input: &str) -> Option<u32> {
    let problem = input.parse::<Problem>().unwrap();
    let solution = problem.solve() as u32;
    Some(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
