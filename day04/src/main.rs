use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    // println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &String) -> usize {
    let mut res: usize = 0;
    let grid: Grid = Grid::new(input);

    for direction in Direction::Right {
        for line in grid.iter_direction(direction) {
            res += line.matches("XMAS").count();
        }
    }

    res
}

// Direction of strings in Grid
#[derive(Clone, Copy)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
    DiagonalDownRight,
    DiagonalDownLeft,
    DiagonalUpRight,
    DiagonalUpLeft,
}

impl Iterator for Direction {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let current = *self;
        *self = match *self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Up => Direction::DiagonalDownRight,
            Direction::DiagonalDownRight => Direction::DiagonalDownLeft,
            Direction::DiagonalDownLeft => Direction::DiagonalUpRight,
            Direction::DiagonalUpRight => Direction::DiagonalUpLeft,
            Direction::DiagonalUpLeft => return None,
        };
        Some(current)
    }
}

struct Grid {
    matrix: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

struct GridIterator<'a> {
    grid: &'a Grid,
    current_row: usize,
    current_col: usize,
    direction: Direction,
}

impl Grid {
    fn new(input: &String) -> Self {
        let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = matrix.len();
        let width = matrix[0].len();
        Grid {
            matrix,
            height,
            width,
        }
    }

    fn iter_direction(&self, direction: Direction) -> GridIterator {
        GridIterator::new(self, direction)
    }
}

impl<'a> GridIterator<'a> {
    fn new(grid: &'a Grid, direction: Direction) -> Self {
        let (current_row, current_col) = match direction {
            Direction::Right | Direction::Left | Direction::Down | Direction::Up => (0, 0),
            Direction::DiagonalDownRight | Direction::DiagonalUpLeft => (0, grid.width - 1),
            Direction::DiagonalUpRight | Direction::DiagonalDownLeft => (0, 0),
        };
        GridIterator {
            grid,
            current_row,
            current_col,
            direction,
        }
    }
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res = String::new();

        match self.direction {
            Direction::Right | Direction::Left => {
                if self.current_row == self.grid.height {
                    return None;
                }
                res = self.grid.matrix[self.current_row].iter().collect();
                self.current_row += 1;
            }
            Direction::Down | Direction::Up => {
                if self.current_col == self.grid.width {
                    return None;
                }
                res = self
                    .grid
                    .matrix
                    .iter()
                    .map(|row| row[self.current_col])
                    .collect();
                self.current_col += 1;
            }
            Direction::DiagonalDownRight | Direction::DiagonalUpLeft => {
                if self.current_row == self.grid.height && self.current_col == 0 {
                    return None;
                }
                res.extend(
                    (self.current_row..self.grid.height)
                        .zip(self.current_col..self.grid.width)
                        .map(|(row, col)| self.grid.matrix[row][col]),
                );
                if self.current_col > 0 {
                    self.current_col -= 1;
                } else {
                    self.current_row += 1;
                }
            }
            Direction::DiagonalDownLeft | Direction::DiagonalUpRight => {
                if self.current_row == self.grid.height && self.current_col == self.grid.width - 1 {
                    return None;
                }
                res.extend(
                    (self.current_row..self.grid.height)
                        .zip(self.current_col..=0)
                        .map(|(row, col)| self.grid.matrix[row][col]),
                );
                if self.current_col < self.grid.width - 1 {
                    self.current_col += 1;
                } else {
                    self.current_row += 1;
                }
            }
        }
        match self.direction {
            Direction::Left
            | Direction::Up
            | Direction::DiagonalUpRight
            | Direction::DiagonalUpLeft => {
                res = res.chars().rev().collect();
            }
            _ => {}
        }
        Some(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 18);
    }

    // #[test]
    // fn test_ex2() {
    //     let input = input::read_example();
    //     let res = exercise2(&input);
    //     assert_eq!(res, 48);
    // }
}
