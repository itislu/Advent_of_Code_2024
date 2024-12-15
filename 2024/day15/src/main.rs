use std::collections::VecDeque;
use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    let mut map = Map::new(input);
    let mut robot = Robot::new(&map, input);

    while !robot.movements.is_empty() {
        robot.mv(&mut map);
    }

    map.grid
        .iter()
        .flatten()
        .filter_map(|object| {
            if object.kind == ObjectKind::Box {
                Some(object.gps_coordinate())
            } else {
                None
            }
        })
        .sum()
}

fn exercise2(input: &str) -> usize {
    0
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        use Direction::*;
        match c {
            '^' => Up,
            'v' => Down,
            '>' => Right,
            '<' => Left,
            _ => panic!("Invalid character in movement found!"),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    fn to(&self, direction: Direction) -> Self {
        use Direction::*;
        match direction {
            Up => Self::new(self.row - 1, self.col),
            Down => Self::new(self.row + 1, self.col),
            Right => Self::new(self.row, self.col + 1),
            Left => Self::new(self.row, self.col - 1),
        }
    }
}

enum BoxPart {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
enum ObjectKind {
    Empty,
    Wall,
    Box(Position),
    Robot,
}

struct Object {
    kind: ObjectKind,
    pos: Position,
}

impl Object {
    fn new(c: char, row: usize, col: usize) -> Self {
        let pos = Position::new(row, col);
        let kind = match c {
            '.' => ObjectKind::Empty,
            '#' => ObjectKind::Wall,
            'O' => ObjectKind::Box(pos),
            '@' => ObjectKind::Robot,
            _ => panic!("Invalid character in map found!"),
        };
        Object { kind, pos }
    }

    fn twice(c: char, row: usize, col: usize) -> [Self; 2] {
        let (pos1, pos2) = (Position::new(row, col), Position::new(row, col + 1));
        let (kind1, kind2) = match c {
            '.' => (ObjectKind::Empty, ObjectKind::Empty),
            '#' => (ObjectKind::Wall, ObjectKind::Wall),
            'O' => (ObjectKind::Box(pos2), ObjectKind::Box(pos1)),
            '@' => (ObjectKind::Robot, ObjectKind::Empty),
            _ => panic!("Invalid character in map found!"),
        };
        [
            Object {
                kind: kind1,
                pos: pos1,
            },
            Object {
                kind: kind2,
                pos: pos2,
            },
        ]
    }

    fn gps_coordinate(&self) -> usize {
        self.pos.row * 100 + self.pos.col
    }
}

struct Robot {
    pos: Position,
    movements: VecDeque<Direction>,
}

impl Robot {
    fn new(map: &Map, input: &str) -> Self {
        Self {
            pos: map
                .grid
                .iter()
                .flatten()
                .find(|object| object.kind == ObjectKind::Robot)
                .expect("No robot in map found!")
                .pos,
            movements: input
                .split("\n\n")
                .skip(1)
                .collect::<String>()
                .lines()
                .flat_map(|line| line.chars())
                .map(Direction::from)
                .collect(),
        }
    }

    fn mv(&mut self, map: &mut Map) -> bool {
        if let Some(direction) = self.movements.pop_front() {
            if map.mv_object(self.pos, direction) {
                self.pos = self.pos.to(direction);
                return true;
            }
        }
        false
    }
}

struct WideMap {
    grid: Vec<Vec<Object>>,
}

impl WideMap {
    fn new(input: &str) -> Self {
        let mut grid: Vec<Vec<Object>> = Vec::new();

        for (row, line) in input.split("\n\n").nth(0).unwrap().lines().enumerate() {
            let mut grid_line: Vec<Object> = Vec::new();
            for (mut col, c) in line.chars().enumerate() {
                col *= 2;
                grid_line.extend(Object::twice(c, row, col));
            }
            grid.push(grid_line);
        }
        Self { grid }
    }

    fn can_move(&self, pos: Position, direction: Direction) -> bool {
        use ObjectKind::*;
        match self.at(pos).kind {
            Empty => return true,
            Wall => return false,
            Box(other_part) => {
                self.can_move(pos.to(direction), direction)
                    && self.can_move(other_part.to(direction), direction)
            }
            Robot => self.can_move(pos.to(direction), direction),
        }
    }

    fn mv_object(&mut self, pos: Position, direction: Direction) -> bool {
        use ObjectKind::*;

        if !self.can_move(pos, direction) {
            return false;
        }
        match self.at(pos).kind {
            Empty => return true,
            Wall => return false,
            Box(other_part) => {
                let new_pos = pos.to(direction);
                self.mv_object(new_pos, direction);
                self.swap(pos, new_pos);
                let new_pos = other_part.to(direction);
                self.mv_object(new_pos, direction);
                self.swap(other_part, new_pos);
                true
            }
            Robot => {
                let new_pos = pos.to(direction);
                if self.mv_object(new_pos, direction) {
                    self.swap(pos, new_pos);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn swap(&mut self, pos1: Position, pos2: Position) {
        let tmp = self.at(pos1).kind;
        self.at_mut(pos1).kind = self.at(pos2).kind;
        self.at_mut(pos2).kind = tmp;
    }

    fn at(&self, pos: Position) -> &Object {
        &self.grid[pos.row][pos.col]
    }

    fn at_mut(&mut self, pos: Position) -> &mut Object {
        &mut self.grid[pos.row][pos.col]
    }
}

struct Map {
    grid: Vec<Vec<Object>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut grid: Vec<Vec<Object>> = Vec::new();

        for (row, line) in input.split("\n\n").nth(0).unwrap().lines().enumerate() {
            let mut grid_line: Vec<Object> = Vec::new();
            for (col, c) in line.chars().enumerate() {
                grid_line.push(Object::new(c, row, col));
            }
            grid.push(grid_line);
        }
        Self { grid }
    }

    fn mv_object(&mut self, pos: Position, direction: Direction) -> bool {
        use ObjectKind::*;

        match self.at(pos).kind {
            Wall => return false,
            Empty => return true,
            Box | Robot => {}
        };
        let new_pos = pos.to(direction);
        if self.mv_object(new_pos, direction) {
            self.swap(pos, new_pos);
            true
        } else {
            false
        }
    }

    fn swap(&mut self, pos1: Position, pos2: Position) {
        let tmp = self.at(pos1).kind;
        self.at(pos1).kind = self.at(pos2).kind;
        self.at(pos2).kind = tmp;
    }

    fn at(&mut self, pos: Position) -> &mut Object {
        &mut self.grid[pos.row][pos.col]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 10092);
    }

    #[test]
    fn test_ex2() {
        let input = input::read_example();
        let res = exercise2(&input);
        assert_eq!(res, 9021);
    }
}
