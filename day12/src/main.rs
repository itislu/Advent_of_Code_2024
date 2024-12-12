use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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
}

impl Tile {
    fn new(row: usize, col: usize, tile_type: char) -> Self {
        Tile {
            pos: Position::new(row, col),
            tile_type,
        }
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tile_type)
    }
}

struct Region {
    tiles: HashMap<Position, Rc<RefCell<Tile>>>,
    tile_type: char,
    area: usize,
    perimiter: usize,
}

impl Region {
    fn new(tiles: HashMap<Position, Rc<RefCell<Tile>>>, map: &Map) -> Self {
        let tile_type = tiles.values().nth(0).unwrap().as_ref().borrow().tile_type;
        let area = tiles.len();
        let perimiter: usize = tiles
            .values()
            .map(|tile| {4 -
                map.get_neighbours(tile)
                    .iter()
                    .filter(|neighbour| neighbour.as_ref().borrow().tile_type == tile_type)
                    .count()
            })
            .sum();
        Region {
            tiles,
            tile_type,
            area,
            perimiter,
        }
    }

    fn collect(
        map: &Map,
        pos: &Position,
        tiles_without_regions: &mut HashSet<Position>,
    ) -> Option<HashMap<Position, Rc<RefCell<Tile>>>> {
        if let Some(tile_rc) = map.at(pos) {
            let tile = tile_rc.as_ref().borrow();
            if tiles_without_regions.remove(&tile.pos) {
                let mut region_tiles: HashMap<Position, Rc<RefCell<Tile>>> = HashMap::new();
                region_tiles.insert(tile.pos, Rc::clone(&tile_rc));
                for neighbour_rc in map.get_neighbours(&tile_rc) {
                    let neighbour = neighbour_rc.as_ref().borrow();
                    if neighbour.tile_type == tile.tile_type {
                        if let Some(new_tiles) =
                            Region::collect(map, &neighbour.pos, tiles_without_regions)
                        {
                            region_tiles.extend(new_tiles);
                        }
                    }
                }
                Some(region_tiles)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} * {}, area: {}, perimiter: {}",
            self.tiles.len(),
            self.tile_type,
            self.area,
            self.perimiter
        )
    }
}

struct Map {
    grid: Vec<Vec<Rc<RefCell<Tile>>>>,
    tiles_without_regions: HashSet<Position>,
    regions: Vec<Region>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut map = Map {
            grid: Vec::new(),
            tiles_without_regions: HashSet::new(),
            regions: Vec::new(),
            height: 0,
            width: 0,
        };
        let mut tiles_without_regions: HashSet<Position> = HashSet::new();
        let grid: Vec<Vec<Rc<RefCell<Tile>>>> = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, ch)| {
                        let tile = Rc::new(RefCell::new(Tile::new(row, col, ch)));
                        tiles_without_regions.insert(tile.as_ref().borrow().pos);
                        tile
                    })
                    .collect()
            })
            .collect();
        map.height = grid.len();
        map.width = grid[0].len();
        map.grid = grid;
        for tile in map
            .grid
            .iter()
            .flatten()
            .map(|tile_rc| tile_rc.as_ref().borrow())
        {
            if let Some(region_tiles) = Region::collect(&map, &tile.pos, &mut tiles_without_regions)
            {
                let region = Region::new(region_tiles, &map);
                println!("new region: {}", region);
                map.regions.push(region);
            }
        }
        map.tiles_without_regions = tiles_without_regions;
        map
    }

    //TODO Change to return an iterator
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
