use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
}

fn exercise1(input: &String) -> usize {
    let mut map = Map::new(input);
    println!("BEFORE:\n{}\n", map);
    let mut guard = Guard::new(&mut map);
    
    loop {
        if !guard.move_forward() {
            break;
        }
    }
    println!("AFTER:\n{}\n", map);

    map.count_visited()
}

const START: char = '^';
const OBSTACLE: char = '#';
const VISITED: char = 'X';

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct Map {
    grid: Vec<Vec<char>>,
    height: i64,
    width: i64,
}

struct Guard<'a> {
    map: &'a mut Map,
    row: i64,
    col: i64,
    direction: Direction,
}

impl Map {
    fn new(input: &String) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
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

    fn at(&self, row: i64, col: i64) -> Option<char> {
        if self.is_in(row, col) {
            Some(self.grid[row as usize][col as usize])
        } else {
            None
        }
    }

    fn visit(&mut self, row: i64, col: i64) -> bool {
        if self.is_in(row, col) {
            self.grid[row as usize][col as usize] = VISITED;
            true
        } else {
            false
        }
    }

    fn count_visited(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .filter(|&&cell| cell == VISITED)
            .count()
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map = self
            .grid
            .iter()
            .map(|line| line.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", map)
    }
}

impl<'a> Guard<'a> {
    fn new(map: &'a mut Map) -> Self {
        let mut row: i64 = -1;
        let mut col: i64 = -1;

        'outer: for (y, line) in map.grid.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if *cell == START {
                    row = y as i64;
                    col = x as i64;
                    map.grid[row as usize][col as usize] = VISITED;
                    break 'outer;
                }
            }
        }
        Guard {
            map,
            row,
            col,
            direction: Direction::Up,
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
            if cell == OBSTACLE {
                self.move_right()
            } else {
                self.row = new_row;
                self.map.visit(self.row, self.col)
            }
        } else {
            false
        }
    }

    fn move_right(&mut self) -> bool {
        let new_col = self.col + 1;
        self.direction = Direction::Right;
        if let Some(cell) = self.map.at(self.row, new_col) {
            if cell == OBSTACLE {
                self.move_down()
            } else {
                self.col = new_col;
                self.map.visit(self.row, self.col)
            }
        } else {
            false
        }
    }

    fn move_down(&mut self) -> bool {
        let new_row = self.row + 1;
        self.direction = Direction::Down;
        if let Some(cell) = self.map.at(new_row, self.col) {
            if cell == OBSTACLE {
                self.move_left()
            } else {
                self.row = new_row;
                self.map.visit(self.row, self.col)
            }
        } else {
            false
        }
    }

    fn move_left(&mut self) -> bool {
        let new_col = self.col - 1;
        self.direction = Direction::Left;
        if let Some(cell) = self.map.at(self.row, new_col) {
            if cell == OBSTACLE {
                self.move_up()
            } else {
                self.col = new_col;
                self.map.visit(self.row, self.col)
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
}
