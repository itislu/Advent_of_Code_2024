use std::{cell::RefCell, collections::VecDeque, rc::Rc};
use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    // println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    let mut map = Map::new(input);

    while let Some(direction) = map.movements.pop_front() {
        let robot_pos = map.robot.borrow().pos;
        map.move_object(robot_pos, direction);
    }

    map.grid
        .iter()
        .flatten()
        .filter_map(|object| {
            if object.borrow().kind == ObjectKind::Box {
                Some(object.borrow().gps_coordinate())
            } else {
                None
            }
        })
        .sum()
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

#[derive(Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    fn get(&self, direction: Direction) -> Self {
        use Direction::*;
        match direction {
            Up => Self::new(self.row - 1, self.col),
            Down => Self::new(self.row + 1, self.col),
            Right => Self::new(self.row, self.col + 1),
            Left => Self::new(self.row, self.col - 1),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum ObjectKind {
    Empty,
    Robot,
    Box,
    Wall,
}

impl From<char> for ObjectKind {
    fn from(c: char) -> Self {
        use ObjectKind::*;
        match c {
            '.' => Empty,
            '@' => Robot,
            'O' => Box,
            '#' => Wall,
            _ => panic!("Invalid character in map found!"),
        }
    }
}

struct Object {
    kind: ObjectKind,
    pos: Position,
}

impl Object {
    fn new(c: char, row: usize, col: usize) -> Self {
        Object {
            kind: ObjectKind::from(c),
            pos: Position::new(row, col),
        }
    }

    fn gps_coordinate(&self) -> usize {
        self.pos.row * 100 + self.pos.col
    }
}

struct Map {
    grid: Vec<Vec<Rc<RefCell<Object>>>>,
    robot: Rc<RefCell<Object>>,
    movements: VecDeque<Direction>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let split_input: Vec<&str> = input.split("\n\n").collect();
        let mut robot_opt: Option<Rc<RefCell<Object>>> = None;
        let mut grid: Vec<Vec<Rc<RefCell<Object>>>> = Vec::new();

        for (row, line) in split_input[0].lines().enumerate() {
            let mut grid_line: Vec<Rc<RefCell<Object>>> = Vec::new();
            for (col, c) in line.chars().enumerate() {
                let object = Rc::new(RefCell::new(Object::new(c, row, col)));
                if object.borrow().kind == ObjectKind::Robot {
                    robot_opt = Some(Rc::clone(&object));
                }
                grid_line.push(object);
            }
            grid.push(grid_line);
        }

        let movements: VecDeque<Direction> = split_input[1]
            .lines()
            .flat_map(|line| line.chars())
            .map(Direction::from)
            .collect();

        Self {
            robot: robot_opt.expect("No robot in map found!"),
            movements,
            height: grid.len(),
            width: grid[0].len(),
            grid,
        }
    }

    fn move_object(&mut self, pos: Position, direction: Direction) -> bool {
        use ObjectKind::*;
        let new_pos = pos.get(direction);
        match self.at(new_pos).borrow().kind {
            Empty | Robot => {
                self.swap(pos, new_pos);
                true
            }
            Box => self.move_object(new_pos, direction),
            Wall => false,
        }
    }

    fn swap(&mut self, pos1: Position, pos2: Position) {
        let tmp = self.at(pos1).borrow().kind;
        self.at(pos1).borrow_mut().kind = self.at(pos2).borrow().kind;
        self.at(pos2).borrow_mut().kind = tmp;
    }

    fn at(&mut self, pos: Position) -> Rc<RefCell<Object>> {
        Rc::clone(&self.grid[pos.row][pos.col])
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

    // #[test]
    // fn test_ex2() {
    //     let input = input::read_example();
    //     let res = exercise2(&input);
    //     println!("{}", res);
    // }
}
