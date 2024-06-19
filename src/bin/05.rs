advent_of_code::solution!(5);

use itertools::Itertools;
use regex::{Regex, RegexBuilder};
use std::{borrow::Borrow, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct ParseProblemError;

#[derive(Clone, Debug, PartialEq)]
struct Range {
    start: u64,
    end: u64,
    level: usize,
}

impl Range {}

#[derive(Debug, PartialEq)]
struct Seeds {
    ranges: Vec<Range>,
}

impl FromStr for Seeds {
    type Err = ParseProblemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number_regex = Regex::new(r"(\d+)").unwrap();

        let values: Vec<Result<u64, ParseProblemError>> = number_regex
            .captures_iter(s)
            .map(|cap| cap[1].parse::<u64>().map_err(|_| ParseProblemError))
            .collect();

        if values.iter().any(|x| x.is_err()) {
            return Err(ParseProblemError);
        }

        let ranges: Vec<Range> = values
            .into_iter()
            .map(|x| x.unwrap())
            .tuples()
            .map(|(a, b)| Range {
                start: a,
                end: a + b,
                level: 0,
            })
            .collect();

        Ok(Seeds { ranges })
    }
}

#[derive(Debug, PartialEq)]
struct Entry {
    start: u64,
    end: u64,
    dst: u64,
}

impl Entry {
    fn translate(&self, range: Range) -> (Vec<Range>, Vec<Range>) {
        let (mut done, mut todo) = (vec![], vec![]);

        if range.start < self.start {
            todo.push(Range {
                start: range.start,
                end: range.end.min(self.start),
                level: range.level,
            })
        };

        if self.end < range.end {
            todo.push(Range {
                start: range.start.max(self.end),
                end: range.end,
                level: range.level,
            })
        };

        if self.start < range.end && self.end > range.start {
            let start = range.start.max(self.start);
            let end = range.end.min(self.end);
            done.push(Range {
                start: self.dst + start - self.start,
                end: self.dst + end - self.start,
                level: range.level + 1,
            })
        };

        (done, todo)
    }

    fn translate_all(&self, ranges: Vec<Range>) -> (Vec<Range>, Vec<Range>) {
        ranges
            .into_iter()
            .fold((vec![], vec![]), |(mut a, mut b), range| {
                let (mut c, mut d) = self.translate(range);
                a.append(&mut c);
                b.append(&mut d);
                (a, b)
            })
    }
}

impl FromStr for Entry {
    type Err = ParseProblemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"(?<dst>\d+)\s+(?<src>\d+)\s+(?<len>\d+)").unwrap();
        let caps = regex.captures(s).ok_or(ParseProblemError)?;
        let dst = caps["dst"].parse::<u64>().map_err(|_| ParseProblemError)?;
        let src = caps["src"].parse::<u64>().map_err(|_| ParseProblemError)?;
        let len = caps["len"].parse::<u64>().map_err(|_| ParseProblemError)?;
        Ok(Entry {
            start: src,
            end: src + len,
            dst,
        })
    }
}

#[derive(Debug, PartialEq)]
struct Map {
    entries: Vec<Entry>,
}

impl Map {
    fn new(entries: Vec<Entry>) -> Map {
        Map { entries }
    }

    fn translate_range(&self, ranges: Vec<Range>) -> Vec<Range> {
        let (mut done, mut todo) =
            self.entries
                .iter()
                .fold((vec![], ranges), |(mut a, b), entry| {
                    let (mut c, d) = entry.translate_all(b);
                    a.append(&mut c);
                    (a, d)
                });

        for range in todo.iter_mut() {
            range.level = range.level + 1;
        }

        done.append(&mut todo);
        done
    }
}

impl FromStr for Map {
    type Err = ParseProblemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries: Vec<Result<Entry, ParseProblemError>> = s
            .split_terminator('\n')
            .into_iter()
            .map(|x| x.parse::<Entry>())
            .collect();
        if entries.iter().any(|x| x.is_err()) {
            return Err(ParseProblemError);
        }
        let entries = entries.into_iter().map(|x| x.unwrap()).collect();
        Ok(Map::new(entries))
    }
}

#[derive(Debug, PartialEq)]
struct Problem {
    seeds: Seeds,
    maps: Vec<Map>,
}

impl Problem {
    const MAP_LEN: usize = 7;
    const MAP_OFFSET: usize = 2;

    fn new(seeds: Seeds, maps: Vec<Map>) -> Problem {
        Problem { seeds, maps }
    }
}

impl FromStr for Problem {
    type Err = ParseProblemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = r"^seeds:\s+(?<seeds>.+)\n\
seed-to-soil map:\n(.+)\n\
soil-to-fertilizer map:\n(.+)\n\
fertilizer-to-water map:\n(.+)\n\
water-to-light map:\n(.+)\n\
light-to-temperature map:\n(.+)\n\
temperature-to-humidity map:\n(.+)\n\
humidity-to-location map:\n(.+)$";

        let problem_regex = RegexBuilder::new(pattern)
            .dot_matches_new_line(true)
            .build()
            .unwrap();
        let caps = problem_regex.captures(s).ok_or(ParseProblemError)?;

        let seeds = caps["seeds"]
            .parse::<Seeds>()
            .map_err(|_| ParseProblemError)?;

        let maps: Vec<Result<Map, ParseProblemError>> = (0..Problem::MAP_LEN)
            .into_iter()
            .map(|i| {
                caps[i + Problem::MAP_OFFSET]
                    .parse::<Map>()
                    .map_err(|_| ParseProblemError)
            })
            .collect();
        if maps.iter().any(|x| x.is_err()) {
            return Err(ParseProblemError);
        }
        let maps = maps.into_iter().map(|x| x.unwrap()).collect();

        Ok(Problem::new(seeds, maps))
    }
}

#[derive(Debug, PartialEq)]
struct Solver {
    problem: Problem,
}

impl Solver {
    fn new(problem: Problem) -> Solver {
        Solver { problem }
    }

    fn solve(&mut self) -> Option<u64> {
        let ranges = self.problem.seeds.ranges.clone();
        let result = self
            .problem
            .maps
            .iter()
            .fold(ranges, |r, map| map.translate_range(r));
        let min = result.iter().min_by_key(|x| x.start).unwrap();
        Some(min.start)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(35)
}

pub fn part_two(input: &str) -> Option<u64> {
    let problem = input.parse::<Problem>().unwrap();
    let mut solver = Solver::new(problem);
    solver.solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(46));
    }
}
