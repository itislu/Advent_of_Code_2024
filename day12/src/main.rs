use std::{cell::RefCell, rc::Rc};

use strum::IntoEnumIterator;
use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    // println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    let map = Map::new(input);

    map.regions
        .iter()
        .map(|region| region.area * region.perimiter)
        .sum()
}

#[derive(strum_macros::EnumIter)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    fn to(&self, direction: Direction) -> Option<Self> {
        Some(match direction {
            Direction::Up => Position::new(self.row.checked_sub(1)?, self.col),
            Direction::Down => Position::new(self.row + 1, self.col),
            Direction::Right => Position::new(self.row, self.col + 1),
            Direction::Left => Position::new(self.row, self.col.checked_sub(1)?),
        })
    }
}

struct Tile {
    pos: Position,
    tile_type: char,
    is_in_region: bool,
}

impl Tile {
    fn new(row: usize, col: usize, tile_type: char) -> Self {
        Tile {
            pos: Position::new(row, col),
            tile_type,
            is_in_region: false,
        }
    }
}

struct Region {
    tile_type: char,
    tiles: Vec<Rc<RefCell<Tile>>>,
    area: usize,
    perimiter: usize,
}

impl Region {
    fn new(tiles: Vec<Rc<RefCell<Tile>>>, map: &Map) -> Self {
        let tile_type = tiles[0].as_ref().borrow().tile_type;
        let perimiter: usize = tiles
            .iter()
            .map(|tile| {
                map.get_neighbours(tile)
                    .iter()
                    .filter(|neighbour| neighbour.borrow().tile_type != tile_type)
                    .count()
            })
            .sum();
        Region {
            tile_type,
            area: tiles.len(),
            perimiter,
            tiles,
        }
    }
}

struct Map {
    grid: Vec<Vec<Rc<RefCell<Tile>>>>,
    regions: Vec<Region>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<Rc<RefCell<Tile>>>> = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, ch)| Rc::new(RefCell::new(Tile::new(row, col, ch))))
                    .collect()
            })
            .collect();
        let mut map = Map {
            regions: Vec::new(),
            height: grid.len(),
            width: grid[0].len(),
            grid,
        };
        map.set_regions();
        map
    }

    fn set_regions(&mut self) {
        for (tile_rc, tile) in self
            .grid
            .iter()
            .flatten()
            .map(|tile_rc| (tile_rc, tile_rc.as_ref()))
        {
            if !tile.borrow().is_in_region {
                let mut region_tiles: Vec<Rc<RefCell<Tile>>> = vec![Rc::clone(&tile_rc)];
                tile.borrow_mut().is_in_region = true;

                for (neighbour_rc, neighbour) in self
                    .get_neighbours(&tile)
                    .iter()
                    .map(|neighbour_rc| (neighbour_rc, neighbour_rc.as_ref()))
                {
                    if !neighbour.borrow().is_in_region
                        && neighbour.borrow().tile_type == tile.borrow().tile_type
                    {
                        region_tiles.push(Rc::clone(neighbour_rc));
                    }
                }
                self.regions.push(Region::new(region_tiles, self));
            }
        }
    }

    fn get_neighbours(&self, tile: &RefCell<Tile>) -> Vec<&Rc<RefCell<Tile>>> {
        let mut neighbours: Vec<&Rc<RefCell<Tile>>> = Vec::new();

        for direction in Direction::iter() {
            if let Some(new_pos) = tile.borrow().pos.to(direction) {
                if let Some(neighbour) = self.at(&new_pos) {
                    neighbours.push(neighbour);
                }
            }
        }
        neighbours
    }

    fn is_in(&self, pos: &Position) -> bool {
        (0..self.height).contains(&pos.row) && (0..self.width).contains(&pos.col)
    }

    fn at(&self, pos: &Position) -> Option<&Rc<RefCell<Tile>>> {
        if self.is_in(pos) {
            Some(&self.grid[pos.row][pos.col])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 1930);
    }

    // #[test]
    // fn test_ex2() {
    //     let input = input::read_example();
    //     let res = exercise2(&input);
    //     assert_eq!(res, 81);
    // }
}
