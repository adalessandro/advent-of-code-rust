advent_of_code::solution!(11);

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Item {
    Empty,
    Galaxy,
}

impl Item {
    fn from_char(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Item::Empty),
            '#' => Ok(Item::Galaxy),
            _ => Err(anyhow!("Failed to parse Item")).context(c),
        }
    }

    fn is_empty(&self) -> bool {
        matches!(self, Item::Empty)
    }

    fn is_galaxy(&self) -> bool {
        matches!(self, Item::Galaxy)
    }
}

#[derive(Debug)]
struct Map {
    items: Vec<Vec<Item>>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
    factor: usize,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<Vec<Item>> = s
            .lines()
            .map(|line| line.chars().map(Item::from_char).collect())
            .collect::<Result<Vec<Vec<Item>>>>()?;
        Ok(Map {
            items,
            empty_rows: vec![],
            empty_cols: vec![],
            factor: 1,
        })
    }
}

impl Map {
    fn set_factor(&mut self, factor: usize) {
        self.factor = factor;
    }

    fn get_dimensions(&self) -> (usize, usize) {
        let rows = self.items.len();
        let cols = self.items[0].len();
        (rows, cols)
    }

    fn compute_empty(&mut self) {
        let (_rows, cols) = self.get_dimensions();

        self.empty_rows = self
            .items
            .iter()
            .map(|row| row.iter().all(|item| item.is_empty()))
            .enumerate()
            .filter(|(_, x)| *x)
            .map(|(i, _)| i)
            .rev()
            .collect();

        self.empty_cols = (0..cols)
            .map(|col| {
                self.items
                    .iter()
                    .map(|row| &row[col])
                    .all(|item| item.is_empty())
            })
            .enumerate()
            .filter(|(_, x)| *x)
            .map(|(i, _)| i)
            .rev()
            .collect();
    }

    fn get_galaxies(&self) -> Vec<(usize, usize)> {
        self.items
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, item)| item.is_galaxy())
                    .map(move |(j, _)| (i, j))
                    .collect()
            })
            .concat()
    }

    fn compute_distance(&self, a: (usize, usize), b: (usize, usize)) -> usize {
        let rows = (a.0.min(b.0), a.0.max(b.0));
        let cols = (a.1.min(b.1), a.1.max(b.1));
        let empty_rows: Vec<&usize> = self
            .empty_rows
            .iter()
            .filter(|&x| rows.0 < *x && *x < rows.1)
            .collect();
        let empty_cols: Vec<&usize> = self
            .empty_cols
            .iter()
            .filter(|&x| cols.0 < *x && *x < cols.1)
            .collect();
        a.0.abs_diff(b.0)
            + a.1.abs_diff(b.1)
            + (empty_rows.len() + empty_cols.len()) * (self.factor - 1)
    }

    fn compute_distances(&self) -> Vec<usize> {
        self.get_galaxies()
            .into_iter()
            .combinations(2)
            .map(|v| self.compute_distance(v[0], v[1]))
            .collect()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut map = input.parse::<Map>().unwrap();
    map.set_factor(2);
    map.compute_empty();
    let distances = map.compute_distances();
    Some(distances.iter().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map = input.parse::<Map>().unwrap();
    map.set_factor(1000000);
    map.compute_empty();
    let distances = map.compute_distances();
    Some(distances.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(82000210));
    }
}
