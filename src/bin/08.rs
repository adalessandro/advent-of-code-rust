advent_of_code::solution!(8);

use itertools::Itertools;
use num::Integer;
use regex::{Regex, RegexBuilder};
use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
struct ParseProblemError;

#[derive(Debug)]
enum Step {
    Left,
    Right,
}

impl Step {
    fn from_char(c: char) -> Result<Self, ParseProblemError> {
        match c {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(ParseProblemError),
        }
    }
}

#[derive(Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

impl Node {
    fn new(id: String, left: String, right: String) -> Node {
        Node { id, left, right }
    }
}

#[derive(Debug)]
struct Problem {
    steps: Vec<Step>,
    nodes: HashMap<String, Node>,
}

impl Problem {
    fn new(steps: Vec<Step>, nodes_vec: Vec<(String, String, String)>) -> Problem {
        let mut nodes: HashMap<String, Node> = HashMap::new();

        for (node, left, right) in nodes_vec {
            nodes.insert(node.clone(), Node::new(node, left, right));
        }

        Problem { steps, nodes }
    }

    fn get_by_id(&self, id: &String) -> &Node {
        self.nodes.get(id).unwrap()
    }

    fn apply(&self, node: &Node, step: &Step) -> &Node {
        match step {
            Step::Left => self.get_by_id(&node.left),
            Step::Right => self.get_by_id(&node.right),
        }
    }
}

impl FromStr for Problem {
    type Err = ParseProblemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = r"^(?<steps>.+)\n\n(?<nodes>.+)$";
        let problem_regex = RegexBuilder::new(pattern)
            .dot_matches_new_line(true)
            .build()
            .unwrap();
        let caps = problem_regex.captures(s).ok_or(ParseProblemError)?;
        let steps: Vec<Step> = caps["steps"]
            .chars()
            .map(|c| Step::from_char(c).unwrap())
            .collect();

        let node_regex = Regex::new(
            r"(?<node>[0-9A-Z]{3})\s+=\s+\((?<left>[0-9A-Z]{3}),\s+(?<right>[0-9A-Z]{3})\)",
        )
        .unwrap();
        let nodes: Vec<(String, String, String)> = node_regex
            .captures_iter(&caps["nodes"])
            .map(|node_caps| {
                (
                    node_caps["node"].to_string(),
                    node_caps["left"].to_string(),
                    node_caps["right"].to_string(),
                )
            })
            .collect();
        Ok(Problem::new(steps, nodes))
    }
}

#[derive(Debug)]
struct Solution {
    seeds: VecDeque<usize>,
    first: usize,
    len: usize,
}

impl Solution {
    fn new(seeds: VecDeque<usize>, first: usize, len: usize) -> Solution {
        let seeds: VecDeque<usize> = seeds.into_iter().filter(|&x| x >= first).collect();
        Solution { seeds, first, len }
    }

    fn min(&self) -> &usize {
        &self.seeds[0]
    }

    fn next(&mut self) {
        let seed = self.seeds.pop_front().unwrap();
        self.seeds.push_back(seed + self.len);
    }
}

#[derive(Debug)]
struct Solver<'a> {
    problem: &'a Problem,
    starts: Vec<&'a Node>,
    results: Vec<Solution>,
}

impl<'a> Solver<'a> {
    const START: char = 'A';
    const END: char = 'Z';

    fn new(problem: &'a Problem) -> Solver<'a> {
        let starts: Vec<&Node> = problem
            .nodes
            .keys()
            .filter(|k| Self::is_start(k))
            .map(|k| problem.get_by_id(k))
            .collect();
        Solver {
            problem,
            starts,
            results: vec![],
        }
    }

    fn is_start(id: &String) -> bool {
        id.chars().last().unwrap() == Self::START
    }

    fn is_end(id: &String) -> bool {
        id.chars().last().unwrap() == Self::END
    }

    fn is_match(&self) -> bool {
        self.results.iter().map(|x| x.min()).all_equal()
    }

    fn min(&self) -> &usize {
        self.results.iter().map(|x| x.min()).min().unwrap()
    }

    fn next_min(&mut self) {
        let min = self
            .results
            .iter_mut()
            .min_by(|a, b| a.min().cmp(b.min()))
            .unwrap();
        min.next();
    }

    fn lcm(&self) -> usize {
        // NOTE: this will only work if all solutions cycle on the first node,
        // i.e. the value of first is 0
        let values: Vec<usize> = self.results.iter().map(|x| x.len).collect();
        values.into_iter().reduce(|acc, v| acc.lcm(&v)).unwrap()
    }

    fn prepare(&mut self) {
        self.results = self
            .starts
            .iter()
            .map(|&start| {
                let mut visited: HashMap<String, usize> = HashMap::new();
                let mut current = start;
                let mut result: VecDeque<usize> = VecDeque::new();
                let mut iterations = 0;
                loop {
                    if let Some(prev) = visited.insert(current.id.clone(), iterations) {
                        return Solution::new(result, prev, iterations - prev);
                    }
                    for step in &self.problem.steps {
                        if Self::is_end(&current.id) {
                            result.push_back(iterations);
                        }
                        current = self.problem.apply(&current, step);
                        iterations += 1;
                    }
                }
            })
            .collect();
    }

    fn validate(&self) -> bool {
        self.results.iter().map(|x| x.first).all_equal()
    }

    fn run(&mut self) -> usize {
        while !self.is_match() {
            dbg!(&self.results[0].seeds[0]);
            self.next_min();
        }
        *self.min()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let problem = input.parse::<Problem>().unwrap();
    let mut solver = Solver::new(&problem);
    solver.prepare();
    assert!(solver.validate());
    //let result = solver.run();
    let result = solver.lcm();
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
