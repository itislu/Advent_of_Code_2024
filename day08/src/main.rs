use itertools::Itertools;
use std::{cell::RefCell, cmp, collections::HashMap};
use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    // println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &String) -> usize {
    let mut res: usize = 0;
    let mut map = Map::new(input);
    let mut antinodes = Vec::new();

    for antennas in map.antennas.values() {
        for combination in antennas.iter().combinations(2) {
            /* `combination` is a `Vec<&RefCell<Point>>`. `RefCell` is a dynamic borrow checker.
            First index into the vector, then dereference the reference to the `RefCell`, then use `borrow()` to borrow the value from the `RefCell`, and then pass a reference to that value.
            This would lead to `&*combination[0]`, but Rust is able to dereference this automatically. */
            antinodes.extend(get_antinodes(
                &combination[0].borrow(),
                &combination[1].borrow(),
            ));
        }
    }
    for antinode in antinodes {
        if map.put_antinode(&antinode) {
            res += 1;
        }
    }
    res
}

fn get_antinodes(antenna1: &Point, antenna2: &Point) -> Vec<Point> {
    let row_diff: i32 = antenna1.row.abs_diff(antenna2.row) as i32;
    let col_diff: i32 = antenna1.col.abs_diff(antenna2.col) as i32;
    vec![
        Point::new(
            cmp::min(antenna1.row, antenna2.row) - row_diff,
            cmp::min(antenna1.col, antenna2.col) - col_diff,
            '#',
        ),
        Point::new(
            cmp::max(antenna1.row, antenna2.row) + row_diff,
            cmp::max(antenna1.col, antenna2.col) + col_diff,
            '#',
        ),
    ]
}

struct Map {
    grid: Vec<Vec<RefCell<Point>>>,
    antennas: HashMap<char, Vec<RefCell<Point>>>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(input: &String) -> Self {
        let mut grid: Vec<Vec<RefCell<Point>>> = Vec::new();
        let mut antennas: HashMap<char, Vec<RefCell<Point>>> = HashMap::new();
        for (i, line) in input.lines().enumerate() {
            let mut row: Vec<RefCell<Point>> = Vec::new();
            for (j, ch) in line.chars().enumerate() {
                let point = RefCell::new(Point::new(i as i32, j as i32, ch));
                if point.borrow().data[0] != '.' {
                    antennas
                        .entry(point.borrow().data[0])
                        .or_insert(Vec::new())
                        .push(point.clone());
                }
                row.push(point);
            }
            grid.push(row);
        }
        Map {
            height: grid.len(),
            width: grid[0].len(),
            grid,
            antennas,
        }
    }

    fn is_in(&self, point: &Point) -> bool {
        (0..self.height as i32).contains(&point.row) && (0..self.width as i32).contains(&point.col)
    }

    fn get(&self, point: &Point) -> &RefCell<Point> {
        &self.grid[point.row as usize][point.col as usize]
    }

    fn put_antinode(&mut self, point: &Point) -> bool {
        if self.is_in(&point) && !self.get(&point).borrow().contains('#') {
            self.get(&point).borrow_mut().data.push('#');
            true
        } else {
            false
        }
    }
}

#[derive(Clone)]
struct Point {
    row: i32,
    col: i32,
    data: Vec<char>,
}

impl Point {
    fn new(row: i32, col: i32, data: char) -> Self {
        Point {
            row,
            col,
            data: vec![data],
        }
    }

    fn contains(&self, ch: char) -> bool {
        self.data.contains(&ch)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 14);
    }

    // #[test]
    // fn test_ex2() {
    //     let input = input::read_example();
    //     let res = exercise2(&input);
    //     assert_eq!(res, 11387);
    // }
}
