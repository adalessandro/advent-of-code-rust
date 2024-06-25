advent_of_code::solution!(8);

use regex::{Regex, RegexBuilder};
use std::{collections::HashMap, str::FromStr};

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

        let node_regex =
            Regex::new(r"(?<node>[A-Z]{3})\s+=\s+\((?<left>[A-Z]{3}),\s+(?<right>[A-Z]{3})\)")
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
struct Solver<'a> {
    problem: &'a Problem,
    current: &'a Node,
    iterations: usize,
}

impl<'a> Solver<'a> {
    const START: &'static str = "AAA";
    const END: &'static str = "ZZZ";

    fn new(problem: &'a Problem) -> Solver<'a> {
        Solver {
            problem,
            current: problem.get_by_id(&Self::START.to_string()),
            iterations: 0,
        }
    }

    fn run(&mut self) {
        'outer: loop {
            for step in &self.problem.steps {
                if self.current.id == Self::END {
                    break 'outer;
                }
                self.current = self.problem.apply(&self.current, step);
                self.iterations += 1;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let problem = input.parse::<Problem>().unwrap();
    let mut solver = Solver::new(&problem);
    solver.run();
    Some(solver.iterations as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
