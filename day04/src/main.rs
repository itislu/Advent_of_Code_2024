use std::ops::Index;

use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    // println!("exercise 2: {}", exercise2(&input));
}

// Direction of strings in Grid
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
                let mut row = self.current_row;
                let mut col = self.current_col;
                while row < self.grid.height && col < self.grid.width {
                    res.push(self.grid.matrix[row][col]);
                    row += 1;
                    col += 1;
                }
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
                let mut row = self.current_row;
                let mut col = self.current_col;
                while row < self.grid.height && col < self.grid.width {
                    res.push(self.grid.matrix[row][col]);
                    row += 1;
                    col -= 1;
                }
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

fn exercise1(input: &String) -> usize {
    let mut res: usize = 0;
    let grid: Grid = Grid::new(input);
    let target = "target".chars().collect::<Vec<char>>();

    for i in 0..grid.height {
        let mut matched: usize = 0;
        for j in 0..grid.width {
            if grid.matrix[i][j] == target[matched] {
                matched += 1;
            }
            if matched == target.len() {
                res += 1;
                matched = 0;
            }
        }
    }

    res
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
