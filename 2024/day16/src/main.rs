use std::collections::{BinaryHeap, HashMap};
use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
}

fn exercise1(input: &str) -> i64 {
    let map = Map::new(input);
    let path = dijkstra(&map);
    path.get(&map.goal)
        .expect("No path to the goal found!")
        .cost
}

fn dijkstra(map: &Map) -> HashMap<Position, Visit> {
    let mut queue: BinaryHeap<Visit> = BinaryHeap::new();
    let mut path: HashMap<Position, Visit> = HashMap::new();
    let mut cost_table: HashMap<Position, i64> = HashMap::new();

    queue.push(Visit::new(map.start, map.start, 0));
    path.insert(map.start, Visit::new(map.start, map.start, 0));
    cost_table.insert(map.start, 0);

    while let Some(cur) = queue.pop() {
        if cur.pos == map.goal {
            break;
        }

        for neighbor_visit in map
            .neighbors(cur.pos)
            .filter_map(|neighbor| cur.visit(neighbor))
        {
            if !cost_table.contains_key(&neighbor_visit.pos)
                || neighbor_visit.cost < cost_table[&neighbor_visit.pos]
            {
                cost_table.insert(neighbor_visit.pos, neighbor_visit.cost);
                path.insert(neighbor_visit.pos, neighbor_visit);
                queue.push(neighbor_visit);
            }
        }
    }
    path
}

enum Movement {
    Straight,
    Right,
    Left,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn neighbors(&self) -> impl Iterator<Item = Self> {
        [
            Self::new(self.row - 1, self.col),
            Self::new(self.row + 1, self.col),
            Self::new(self.row, self.col + 1),
            Self::new(self.row, self.col - 1),
        ]
        .into_iter()
    }

    fn to(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self::new(self.row - 1, self.col),
            Direction::South => Self::new(self.row + 1, self.col),
            Direction::East => Self::new(self.row, self.col + 1),
            Direction::West => Self::new(self.row, self.col - 1),
        }
    }

    fn dir(&self, from: Position) -> Direction {
        let row_diff = self.row as i64 - from.row as i64;
        let col_diff = self.col as i64 - from.col as i64;

        if col_diff.abs() >= row_diff.abs() {
            match col_diff >= 0 {
                true => Direction::East,
                false => Direction::West,
            }
        } else {
            match row_diff <= 0 {
                true => Direction::North,
                false => Direction::South,
            }
        }
    }
}

#[derive(PartialEq, Eq)]
enum TileKind {
    Wall,
    Path,
    Start,
    Goal,
}

impl From<char> for TileKind {
    fn from(c: char) -> Self {
        match c {
            '#' => TileKind::Wall,
            '.' => TileKind::Path,
            'S' => TileKind::Start,
            'E' => TileKind::Goal,
            _ => panic!("Invalid character in map found!"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn counter_clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Visit {
    pos: Position,
    facing: Direction,
    came_from: Position,
    cost: i64,
}

impl Visit {
    fn new(pos: Position, came_from: Position, cost: i64) -> Self {
        Self {
            pos,
            facing: pos.dir(came_from),
            came_from,
            cost,
        }
    }

    fn visit(&self, target: &Tile) -> Option<Visit> {
        match (
            self.pos.to(self.facing),
            self.pos.to(self.facing.clockwise()),
            self.pos.to(self.facing.counter_clockwise()),
        ) {
            (pos, _, _) if pos == target.pos => Some(Visit::new(pos, self.pos, self.cost + 1)),
            (_, pos, _) if pos == target.pos => Some(Visit::new(pos, self.pos, self.cost + 1001)),
            (_, _, pos) if pos == target.pos => Some(Visit::new(pos, self.pos, self.cost + 1001)),
            _ => None,
        }
    }
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Tile {
    pos: Position,
    kind: TileKind,
}

impl Tile {
    fn new(row: usize, col: usize, c: char) -> Self {
        Self {
            pos: Position::new(row, col),
            kind: TileKind::from(c),
        }
    }
}

struct Map {
    grid: Vec<Vec<Tile>>,
    start: Position,
    goal: Position,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut grid: Vec<Vec<Tile>> = Vec::new();
        let mut start_opt: Option<Position> = None;
        let mut goal_opt: Option<Position> = None;

        for (row, line) in input.lines().enumerate() {
            let mut grid_line: Vec<Tile> = Vec::new();
            for (col, c) in line.chars().enumerate() {
                let tile = Tile::new(row, col, c);
                match tile.kind {
                    TileKind::Start => start_opt = Some(tile.pos),
                    TileKind::Goal => goal_opt = Some(tile.pos),
                    _ => {}
                };
                grid_line.push(tile);
            }
            grid.push(grid_line);
        }
        Self {
            grid,
            start: start_opt.expect("No start tile ('S') found!"),
            goal: goal_opt.expect("No goal tile ('E') found!"),
        }
    }

    fn neighbors(&self, pos: Position) -> impl Iterator<Item = &Tile> {
        {}
        pos.neighbors()
            .map(|neighbor_pos| self.at(neighbor_pos))
            .filter(|neighbor| neighbor.kind != TileKind::Wall)
    }

    fn at(&self, pos: Position) -> &Tile {
        &self.grid[pos.row][pos.col]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 7036);
    }
}
