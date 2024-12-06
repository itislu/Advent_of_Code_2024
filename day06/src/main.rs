use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &String) -> usize {
    let mut map = Map::new(input);
    println!("BEFORE:\n{}\n", map);
    let mut guard = Guard::new(input);

    loop {
        if !guard.move_forward() {
            break;
        }
    }
    println!("AFTER:\n{}\n", map);

    map.count_visited()
}

fn exercise2(input: &String) -> usize {
    let mut res: usize = 0;
    let mut guard = Guard::new(input);
    let mut guard_states: Vec<Guard> = Vec::new();

    loop {
        guard_states.push(guard.clone());
        if !guard.move_forward() {
            break;
        }
    }

    for window in guard_states.windows(2).rev() {
        let mut guard = window[1].clone();
        let obstacle = &window[0];
        guard
            .map
            .put(obstacle.row, obstacle.col, Indicator::Obstacle);
        loop {
            if !guard.move_forward() {
                break;
            }
        }
    }

    res
}

#[derive(Clone, Copy, PartialEq)]
enum Indicator {
    Start,
    Obstacle,
    Visited,
    Unvisited,
}

impl From<char> for Indicator {
    fn from(c: char) -> Self {
        match c {
            '^' => Indicator::Start,
            '#' => Indicator::Obstacle,
            'X' => Indicator::Visited,
            '.' => Indicator::Unvisited,
            _ => panic!("Invalid character in map found!"),
        }
    }
}

impl std::fmt::Display for Indicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Indicator::Start => '^',
            Indicator::Obstacle => '#',
            Indicator::Visited => 'X',
            Indicator::Unvisited => '.',
        };
        write!(f, "{}", c)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone)]
struct Cell {
    indicator: Indicator,
    row: i64,
    col: i64,
    visited: Vec<Visit>,
}

impl Cell {
    fn new(c: char, row: i64, col: i64) -> Self {
        Cell {
            indicator: Indicator::from(c),
            row,
            col,
            visited: Vec::new(),
        }
    }

    fn visit(&mut self, direction: Direction) -> Result<(), &Vec<Visit>> {
        self.visited.push(Visit::new(self.row, self.col, direction));
        if self.indicator == Indicator::Obstacle {
            Err(&self.visited)
        } else {
            self.indicator = Indicator::Visited;
            Ok(())
        }
    }
}

#[derive(Clone, Copy)]
struct Visit {
    row: i64,
    col: i64,
    with: Direction,
}

impl Visit {
    fn new(row: i64, col: i64, with: Direction) -> Self {
        Visit { row, col, with }
    }
}

#[derive(Clone)]
struct Map {
    grid: Vec<Vec<Cell>>,
    height: i64,
    width: i64,
}

impl Map {
    fn new(input: &String) -> Self {
        let grid: Vec<Vec<Cell>> = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| Cell::new(c, row as i64, col as i64))
                    .collect()
            })
            .collect();
        let height: i64 = grid.len() as i64;
        let width: i64 = grid[0].len() as i64;
        Map {
            grid,
            height,
            width,
        }
    }

    fn is_in(&self, row: i64, col: i64) -> bool {
        (0..self.height).contains(&row) && (0..self.width).contains(&col)
    }

    fn at(&mut self, row: i64, col: i64) -> Option<&mut Cell> {
        if self.is_in(row, col) {
            Some(&mut self.grid[row as usize][col as usize])
        } else {
            None
        }
    }

    fn put(&mut self, row: i64, col: i64, indicator: Indicator) -> bool {
        if self.is_in(row, col) {
            self.grid[row as usize][col as usize].indicator = indicator;
            true
        } else {
            false
        }
    }

    fn count_visited(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .filter(|&cell| cell.indicator == Indicator::Visited)
            .count()
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map = self
            .grid
            .iter()
            .map(|line| {
                line.iter()
                    .map(|cell| cell.indicator.to_string())
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", map)
    }
}

#[derive(Clone)]
struct Guard {
    map: Map,
    row: i64,
    col: i64,
    direction: Direction,
    path: Vec<Visit>,
}

impl Guard {
    fn new(input: &String) -> Self {
        let mut row: i64 = -1;
        let mut col: i64 = -1;
        let direction = Direction::Up;
        let mut map = Map::new(input);

        'outer: for (y, line) in map.grid.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if cell.indicator == Indicator::Start {
                    row = y as i64;
                    col = x as i64;
                    map.grid[row as usize][col as usize].indicator = Indicator::Visited;
                    break 'outer;
                }
            }
        }
        Guard {
            map,
            row,
            col,
            direction,
            path: Vec::new(),
        }
    }

    fn move_forward(&mut self) -> bool {
        match self.direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Right => self.move_right(),
            Direction::Left => self.move_left(),
        }
    }

    fn move_up(&mut self) -> bool {
        let new_row = self.row - 1;
        self.direction = Direction::Up;
        if let Some(cell) = self.map.at(new_row, self.col) {
            match cell.visit(self.direction) {
                Ok(_) => {
                    self.row = new_row;
                    self.path
                        .push(Visit::new(self.row, self.col, self.direction));
                    true
                }
                Err(visited) => {
                    if visited
                        .iter()
                        .filter(|&&visit| visit.with == self.direction)
                        .count()
                        <= 1
                    {
                        self.move_right()
                    } else {
                        false
                    }
                }
            }
        } else {
            false
        }
    }

    fn move_right(&mut self) -> bool {
        let new_col = self.col + 1;
        self.direction = Direction::Right;
        if let Some(cell) = self.map.at(self.row, new_col) {
            match cell.visit(self.direction) {
                Ok(_) => {
                    self.col = new_col;
                    self.path
                        .push(Visit::new(self.row, self.col, self.direction));
                    true
                }
                Err(visited) => {
                    if visited
                        .iter()
                        .filter(|&&visit| visit.with == self.direction)
                        .count()
                        <= 1
                    {
                        self.move_down()
                    } else {
                        false
                    }
                }
            }
        } else {
            false
        }
    }

    fn move_down(&mut self) -> bool {
        let new_row = self.row + 1;
        self.direction = Direction::Down;
        if let Some(cell) = self.map.at(new_row, self.col) {
            match cell.visit(self.direction) {
                Ok(_) => {
                    self.row = new_row;
                    self.path
                        .push(Visit::new(self.row, self.col, self.direction));
                    true
                }
                Err(visited) => {
                    if visited
                        .iter()
                        .filter(|&&visit| visit.with == self.direction)
                        .count()
                        <= 1
                    {
                        self.move_left()
                    } else {
                        false
                    }
                }
            }
        } else {
            false
        }
    }

    fn move_left(&mut self) -> bool {
        let new_col = self.col - 1;
        self.direction = Direction::Left;
        if let Some(cell) = self.map.at(self.row, new_col) {
            match cell.visit(self.direction) {
                Ok(_) => {
                    self.col = new_col;
                    self.path
                        .push(Visit::new(self.row, self.col, self.direction));
                    true
                }
                Err(visited) => {
                    if visited
                        .iter()
                        .filter(|&&visit| visit.with == self.direction)
                        .count()
                        <= 1
                    {
                        self.move_up()
                    } else {
                        false
                    }
                }
            }
        } else {
            false
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
        assert_eq!(res, 41);
    }

    #[test]
    fn test_ex2() {
        let input = input::read_example();
        let res = exercise2(&input);
        assert_eq!(res, 6);
    }
}
