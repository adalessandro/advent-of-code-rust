advent_of_code::solution!(2);

use regex::{Match, Regex};

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

type Bag = Set;

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn parse_set(input: &str) -> Set {
    let re = Regex::new(r"(?<red>\d+) red|(?<green>\d+) green|(?<blue>\d+) blue").unwrap();
    let mut set = Set {
        red: 0,
        green: 0,
        blue: 0,
    };
    let parse_match = |m: Match| m.as_str().parse::<u32>().unwrap();
    for c in re.captures_iter(input) {
        if let Some(color) = c.name("red") {
            set.red = parse_match(color);
        } else if let Some(color) = c.name("green") {
            set.green = parse_match(color);
        } else if let Some(color) = c.name("blue") {
            set.blue = parse_match(color);
        }
    }
    set
}

fn parse_game(input: &str) -> Game {
    let re = Regex::new(r"Game (?<id>\d+): (?<sets>.*)").unwrap();
    let caps = re.captures(input).unwrap();
    let id: u32 = caps["id"].parse().unwrap();
    let sets: Vec<Set> = caps["sets"]
        .split_terminator(';')
        .into_iter()
        .map(|set| parse_set(set.trim()))
        .collect();
    Game { id, sets }
}

fn valid_set(set: &Set, bag: &Bag) -> bool {
    set.red <= bag.red && set.green <= bag.green && set.blue <= bag.blue
}

fn valid_game(game: &Game, bag: &Bag) -> bool {
    game.sets.iter().all(|set| valid_set(set, bag))
}

fn minimal_bag(game: &Game) -> Bag {
    let mut bag = Bag {
        red: 0,
        green: 0,
        blue: 0,
    };
    for set in game.sets.iter() {
        bag.red = bag.red.max(set.red);
        bag.green = bag.green.max(set.green);
        bag.blue = bag.blue.max(set.blue);
    }
    bag
}

fn power_game(game: &Game) -> u32 {
    let bag = minimal_bag(game);
    bag.red * bag.green * bag.blue
}

pub fn part_one(input: &str) -> Option<u32> {
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    let games: Vec<Game> = input
        .split_terminator('\n')
        .into_iter()
        .map(|line| parse_game(line))
        .collect();
    let result: u32 = games
        .iter()
        .filter(|game| valid_game(game, &bag))
        .map(|game| game.id)
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games: Vec<Game> = input
        .split_terminator('\n')
        .into_iter()
        .map(|line| parse_game(line))
        .collect();
    let result: u32 = games.iter().map(|game| power_game(game)).sum();
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2286));
    }
}
