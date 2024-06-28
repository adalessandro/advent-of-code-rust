advent_of_code::solution!(9);

use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
struct ParseProblemError;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Sequence {
    values: Vec<isize>,
}

impl Sequence {
    fn all_zeroes(&self) -> bool {
        self.values.iter().all(|&x| x == 0)
    }

    fn derive(&self) -> Sequence {
        let values = self
            .values
            .iter()
            .tuple_windows()
            .map(|(x, y)| y - x)
            .collect();
        Sequence { values }
    }

    fn derive_all(&self) -> Vec<Sequence> {
        if self.all_zeroes() {
            return vec![];
        }
        let child = self.derive();
        let mut children = child.derive_all();
        children.push(child);
        children
    }

    fn integrate_one(&self, value: isize) -> isize {
        self.values.last().unwrap() + value
    }

    fn integrate_two(&self, value: isize) -> isize {
        self.values.first().unwrap() - value
    }
}

impl FromStr for Sequence {
    type Err = ParseProblemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<Result<isize, ParseProblemError>> = s
            .split_terminator(' ')
            .map(|x| x.parse::<isize>().map_err(|_| ParseProblemError))
            .collect();

        if values.iter().any(|x| x.is_err()) {
            return Err(ParseProblemError);
        }

        let values: Vec<isize> = values.into_iter().map(|x| x.unwrap()).collect();

        Ok(Sequence { values })
    }
}

#[derive(Debug)]
struct Problem {
    sequences: Vec<Sequence>,
}

impl Problem {
    fn new(sequences: Vec<Sequence>) -> Problem {
        Problem { sequences }
    }
}

impl FromStr for Problem {
    type Err = ParseProblemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sequences: Vec<Result<Sequence, ParseProblemError>> = s
            .split_terminator('\n')
            .map(|x| x.parse::<Sequence>().map_err(|_| ParseProblemError))
            .collect();

        if sequences.iter().any(|x| x.is_err()) {
            return Err(ParseProblemError);
        }

        let sequences: Vec<Sequence> = sequences.into_iter().map(|x| x.unwrap()).collect();

        Ok(Problem::new(sequences))
    }
}

#[derive(Debug)]
struct SolverOne<'a> {
    problem: &'a Problem,
}

impl<'a> SolverOne<'a> {
    fn new(problem: &Problem) -> SolverOne {
        SolverOne { problem }
    }

    fn integrate_all(subsequences: &Vec<Sequence>) -> isize {
        subsequences.iter().fold(0, |x, y| y.integrate_one(x))
    }

    fn run(&self) -> isize {
        let result = self
            .problem
            .sequences
            .iter()
            .map(|seq| (seq, seq.derive_all()));

        let subsequences: HashMap<&'a Sequence, Vec<Sequence>> = HashMap::from_iter(result);

        let result: isize = subsequences
            .iter()
            .map(|(&x, y)| x.integrate_one(Self::integrate_all(y)))
            .sum();

        result
    }
}

#[derive(Debug)]
struct SolverTwo<'a> {
    problem: &'a Problem,
}

impl<'a> SolverTwo<'a> {
    fn new(problem: &Problem) -> SolverTwo {
        SolverTwo { problem }
    }

    fn integrate_all(subsequences: &Vec<Sequence>) -> isize {
        subsequences.iter().fold(0, |x, y| y.integrate_two(x))
    }

    fn run(&self) -> isize {
        let result = self
            .problem
            .sequences
            .iter()
            .map(|seq| (seq, seq.derive_all()));

        let subsequences: HashMap<&'a Sequence, Vec<Sequence>> = HashMap::from_iter(result);

        let result: isize = subsequences
            .iter()
            .map(|(&x, y)| x.integrate_two(Self::integrate_all(y)))
            .sum();

        result
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let problem = input.parse::<Problem>().unwrap();
    let solver = SolverOne::new(&problem);
    let result = solver.run();
    Some(result)
}

pub fn part_two(input: &str) -> Option<isize> {
    let problem = input.parse::<Problem>().unwrap();
    let solver = SolverTwo::new(&problem);
    let result = solver.run();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
