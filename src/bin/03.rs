advent_of_code::solution!(3);

use regex::Regex;

#[derive(Debug)]
enum Content {
    Part(u32),
    Symbol(char),
}

#[derive(Debug)]
struct Item {
    line: usize,
    start: usize,
    end: usize,
    content: Content,
}

impl Item {
    fn collide(&self, other: &Self) -> bool {
        self.line.abs_diff(other.line) <= 1 && self.start <= other.end && self.end >= other.start
    }
}

type Schematic = Vec<Item>;

fn parse_schematic(input: &str) -> Schematic {
    let re = Regex::new(r"(?<part>\d+)|(?<symbol>[\D--\.])").unwrap();
    let mut schematic: Schematic = vec![];

    for (i, val) in input.split_terminator('\n').into_iter().enumerate() {
        for c in re.captures_iter(val) {
            if let Some(part) = c.name("part") {
                schematic.push(Item {
                    line: i,
                    start: part.start(),
                    end: part.end(),
                    content: Content::Part(part.as_str().parse().unwrap()),
                })
            } else if let Some(symbol) = c.name("symbol") {
                schematic.push(Item {
                    line: i,
                    start: symbol.start(),
                    end: symbol.end(),
                    content: Content::Symbol(symbol.as_str().parse().unwrap()),
                })
            }
        }
    }
    schematic
}

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = parse_schematic(input);

    let symbols: Vec<&Item> = schematic
        .iter()
        .filter(|x| matches!(x.content, Content::Symbol(_)))
        .collect();

    let result = schematic
        .iter()
        .filter(|x| matches!(x.content, Content::Part(_)))
        .filter(|p| symbols.iter().any(|s| p.collide(s)))
        .map(|x| &x.content)
        .map(|content| match content {
            Content::Part(x) => x,
            Content::Symbol(_) => &0,
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let schematic = parse_schematic(input);

    let parts: Vec<&Item> = schematic
        .iter()
        .filter(|x| matches!(x.content, Content::Part(_)))
        .collect();

    let result: Vec<Vec<&&Item>> = schematic
        .iter()
        .filter(|x| matches!(x.content, Content::Symbol('*')))
        .map(|s| {
            parts
                .iter()
                .filter(|p| s.collide(p))
                .collect::<Vec<&&Item>>()
        })
        .filter(|s| s.len() == 2)
        .collect();

    let result: Vec<Vec<&u32>> = result
        .into_iter()
        .map(|s| {
            s.iter()
                .map(|p| match &p.content {
                    Content::Part(x) => x,
                    Content::Symbol(_) => &0,
                })
                .collect::<Vec<&u32>>()
        })
        .collect();

    let result: u32 = result
        .into_iter()
        .map(|s| s.into_iter().fold(1, |x, y| x * y))
        .sum();

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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(467835));
    }
}
