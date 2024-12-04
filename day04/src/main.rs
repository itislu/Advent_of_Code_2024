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
    direction: Direction,
    current_row: usize,
    current_col: usize,
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
            direction: Direction::Right,
            current_row: 0,
            current_col: 0,
        }
    }    
    
    fn with_direction(input: &String, direction: Direction) -> Self {
        let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = matrix.len();
        let width = matrix[0].len();
        let (current_row, current_col) = match direction {
            Direction::Right | Direction::Left | Direction::Down | Direction::Up => (0, 0),
            Direction::DiagonalDownRight | Direction::DiagonalUpLeft => (0, width - 1),
            Direction::DiagonalUpRight | Direction::DiagonalDownLeft => (0, 0),
        };
        Grid {
            matrix,
            height,
            width,
            direction,
            current_row,
            current_col,
        }
    }
}

impl Iterator for Grid {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res = String::new();

        match self.direction {
            Direction::Right | Direction::Left => {
                if self.current_row == self.height {
                    return None;
                }
                res = self.matrix[self.current_row].iter().collect();
                self.current_row += 1;
            },
            Direction::Down | Direction::Up => {
                if self.current_col == self.width {
                    return None;
                }
                res = self.matrix.iter().map(|row| row[self.current_col]).collect();
                self.current_col += 1;
            },
            Direction::DiagonalDownRight | Direction::DiagonalUpLeft => {
                if self.current_row == self.height && self.current_col == 0 {
                    return None;
                }
                let mut row = self.current_row;
                let mut col = self.current_col;
                while row < self.height && col < self.width {
                    res.push(self.matrix[row][col]);
                    row += 1;
                    col += 1;
                }
                if self.current_col > 0 {
                    self.current_col -= 1;
                } else {
                    self.current_row += 1;
                }
            },
            Direction::DiagonalDownLeft | Direction::DiagonalUpRight => {
                if self.current_row == self.height && self.current_col == self.width - 1 {
                    return None;
                }
                let mut row = self.current_row;
                let mut col = self.current_col;
                while row < self.height && col < self.width {
                    res.push(self.matrix[row][col]);
                    row += 1;
                    col -= 1;
                }
                if self.current_col < self.width - 1 {
                    self.current_col += 1;
                } else {
                    self.current_row += 1;
                }
            },
        }

        match self.direction {
            Direction::Left | Direction::Up | Direction::DiagonalUpRight | Direction::DiagonalUpLeft => {
                res = res.chars().rev().collect();
            },
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
