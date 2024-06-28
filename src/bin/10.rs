advent_of_code::solution!(10);

use std::cell::RefCell;
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug)]
struct ParseTileError;

#[derive(Debug, Clone, Copy)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NEBend,
    NWBend,
    SWBend,
    SEBend,
    Ground,
    Start,
}

impl FromStr for Tile {
    type Err = ParseTileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Tile::VerticalPipe),
            "-" => Ok(Tile::HorizontalPipe),
            "L" => Ok(Tile::NEBend),
            "J" => Ok(Tile::NWBend),
            "7" => Ok(Tile::SWBend),
            "F" => Ok(Tile::SEBend),
            "." => Ok(Tile::Ground),
            "S" => Ok(Tile::Start),
            _ => Err(ParseTileError),
        }
    }
}

#[derive(Debug)]
enum ParseMapError {
    TileError(ParseTileError),
}

impl From<ParseTileError> for ParseMapError {
    fn from(e: ParseTileError) -> Self {
        ParseMapError::TileError(e)
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn new(tiles: Vec<Vec<Tile>>) -> Map {
        Map { tiles }
    }

    fn get_tile(&self, row: usize, col: usize) -> &Tile {
        &self.tiles[row][col]
    }

    fn get_dimensions(&self) -> (usize, usize) {
        let rows = self.tiles.len();
        let cols = self.tiles[0].len();
        (rows, cols)
    }

    fn get_start(&self) -> (usize, usize) {
        let tiles: Vec<Option<usize>> = self
            .tiles
            .iter()
            .map(|row| row.iter().position(|tile| matches!(tile, Tile::Start)))
            .collect();
        let row = tiles.iter().position(|row| row.is_some()).unwrap();
        let col = tiles.iter().find(|row| row.is_some()).unwrap().unwrap();
        (row, col)
    }

    fn detect_tile(&self, row: usize, col: usize) -> Tile {
        let (rows, cols) = self.get_dimensions();
        let mut result = vec![];

        if row > 0 {
            if self.get_neighbours(row - 1, col).contains(&(row, col)) {
                result.push('N');
            }
        }
        if col < cols - 1 {
            if self.get_neighbours(row, col + 1).contains(&(row, col)) {
                result.push('E');
            }
        }
        if row < rows - 1 {
            if self.get_neighbours(row + 1, col).contains(&(row, col)) {
                result.push('S');
            }
        }
        if col > 0 {
            if self.get_neighbours(row, col - 1).contains(&(row, col)) {
                result.push('W');
            }
        }

        assert_eq!(result.len(), 2);

        if result.contains(&'N') {
            if result.contains(&'E') {
                Tile::NEBend
            } else if result.contains(&'S') {
                Tile::VerticalPipe
            } else {
                Tile::NWBend
            }
        } else if result.contains(&'E') {
            if result.contains(&'S') {
                Tile::SEBend
            } else {
                Tile::HorizontalPipe
            }
        } else {
            Tile::SWBend
        }
    }

    fn get_neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let (rows, cols) = self.get_dimensions();
        let tile = self.get_tile(row, col);
        let mut result = vec![];

        match tile {
            Tile::VerticalPipe => {
                if row > 0 {
                    result.push((row - 1, col));
                }
                if row < rows - 1 {
                    result.push((row + 1, col));
                }
            }
            Tile::HorizontalPipe => {
                if col > 0 {
                    result.push((row, col - 1));
                }
                if col < cols - 1 {
                    result.push((row, col + 1));
                }
            }
            Tile::NEBend => {
                if row > 0 {
                    result.push((row - 1, col));
                }
                if col < cols - 1 {
                    result.push((row, col + 1));
                }
            }
            Tile::NWBend => {
                if row > 0 {
                    result.push((row - 1, col));
                }
                if col > 0 {
                    result.push((row, col - 1));
                }
            }
            Tile::SWBend => {
                if row < rows - 1 {
                    result.push((row + 1, col));
                }
                if col > 0 {
                    result.push((row, col - 1));
                }
            }
            Tile::SEBend => {
                if row < rows - 1 {
                    result.push((row + 1, col));
                }
                if col < cols - 1 {
                    result.push((row, col + 1));
                }
            }
            Tile::Ground => {}
            Tile::Start => {
                if row > 0 {
                    if self.get_neighbours(row - 1, col).contains(&(row, col)) {
                        result.push((row - 1, col));
                    }
                }
                if col < cols - 1 {
                    if self.get_neighbours(row, col + 1).contains(&(row, col)) {
                        result.push((row, col + 1));
                    }
                }
                if row < rows - 1 {
                    if self.get_neighbours(row + 1, col).contains(&(row, col)) {
                        result.push((row + 1, col));
                    }
                }
                if col > 0 {
                    if self.get_neighbours(row, col - 1).contains(&(row, col)) {
                        result.push((row, col - 1));
                    }
                }
            }
        }

        result
    }
}

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Vec<Tile>> = s
            .lines()
            .map(|line| {
                (0..line.len())
                    .map(|i| line.get(i..i + 1).unwrap().parse::<Tile>())
                    .collect()
            })
            .collect::<Result<Vec<Vec<Tile>>, ParseTileError>>()?;
        Ok(Map::new(tiles))
    }
}

#[derive(Debug)]
struct Cell<'a> {
    tile: &'a Tile,
    distance: RefCell<Option<usize>>,
    outside: RefCell<bool>,
}

impl<'a> Cell<'a> {
    fn new(tile: &'a Tile) -> Cell {
        Cell {
            tile,
            distance: RefCell::new(None),
            outside: RefCell::new(false),
        }
    }

    fn visited(&self) -> bool {
        self.distance.borrow().is_some()
    }

    fn is_outside(&self) -> bool {
        *self.outside.borrow()
    }
}

#[derive(Debug)]
struct Solver<'a> {
    map: &'a Map,
    cells: Vec<Vec<Cell<'a>>>,
    queue: RefCell<VecDeque<(usize, usize, usize)>>,
}

impl<'a> Solver<'a> {
    fn new(map: &'a Map) -> Solver {
        let cells: Vec<Vec<Cell>> = map
            .tiles
            .iter()
            .map(|row| row.iter().map(Cell::new).collect())
            .collect();
        let queue = RefCell::new(VecDeque::new());
        Solver { map, cells, queue }
    }

    fn get_cell(&self, row: usize, col: usize) -> &Cell {
        &self.cells[row][col]
    }

    fn visited_cell(&self, row: usize, col: usize) -> bool {
        self.get_cell(row, col).visited()
    }

    fn run_cell(&self) {
        let (row, col, distance) = self.queue.borrow_mut().pop_front().unwrap();
        let cell = self.get_cell(row, col);

        let neighbours: Vec<(usize, usize)> = self
            .map
            .get_neighbours(row, col)
            .into_iter()
            .filter(|(r, c)| !self.visited_cell(*r, *c))
            .filter(|(r, c)| !matches!(self.get_cell(*r, *c).tile, Tile::Ground))
            .collect();
        cell.distance.borrow_mut().replace(distance);

        for (r, c) in neighbours {
            self.queue.borrow_mut().push_back((r, c, distance + 1));
        }
    }

    fn get_max(&self) -> Option<usize> {
        self.cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| *cell.distance.borrow())
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }

    fn get_inside(&self) -> usize {
        self.cells
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|cell| !cell.visited() && !cell.is_outside())
                    .count()
            })
            .sum()
    }

    fn run_zone(&self) {
        let (rows, cols) = self.map.get_dimensions();

        for r in 0..rows {
            let mut outside = true;
            let mut status: Option<Tile> = None;

            for c in 0..cols {
                let cell = self.get_cell(r, c);

                if !cell.visited() {
                    *cell.outside.borrow_mut() = outside;
                    continue;
                }

                let tile = if matches!(cell.tile, Tile::Start) {
                    self.map.detect_tile(r, c)
                } else {
                    *cell.tile
                };

                if let Some(init) = status {
                    match tile {
                        Tile::HorizontalPipe => continue,
                        Tile::NWBend => {
                            if !matches!(init, Tile::NEBend) {
                                outside = !outside;
                            }
                            status = None;
                        }
                        Tile::SWBend => {
                            if !matches!(init, Tile::SEBend) {
                                outside = !outside;
                            }
                            status = None;
                        }
                        _ => panic!("Shouldn't happen: {:?}", tile),
                    }
                } else {
                    match tile {
                        Tile::VerticalPipe => outside = !outside,
                        Tile::NEBend | Tile::SEBend => status = Some(tile),
                        _ => panic!("Shouldn't happen: {:?}", tile),
                    }
                }
            }
        }
    }

    fn run_loop(&self) {
        let (row, col) = self.map.get_start();
        self.queue.borrow_mut().push_back((row, col, 0));
        while !self.queue.borrow().is_empty() {
            self.run_cell();
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = input.parse::<Map>().unwrap();
    let solver = Solver::new(&map);
    solver.run_loop();
    solver.get_max()
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = input.parse::<Map>().unwrap();
    let solver = Solver::new(&map);
    solver.run_loop();
    solver.run_zone();
    Some(solver.get_inside())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(10));
    }
}
