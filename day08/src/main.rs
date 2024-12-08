use std::collections::HashMap;

use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    // println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &String) -> usize {
    let mut res: usize = 0;



    res
}

struct Map<'a> {
    grid: Vec<Vec<Point>>,
    antennas: HashMap<char, Vec<&'a Point>>,
    height: usize,
    width: usize,
}

impl<'a> Map<'a> {
    fn new(input: &String) -> Self {
        let mut grid: Vec<Vec<Point>> = Vec::new();
        let mut antennas: HashMap<char, Vec<&'a Point>> = HashMap::new();
        for (i, line) in input.lines().enumerate() {
            let mut row: Vec<Point> = Vec::new();
            for (j, ch) in line.chars().enumerate() {
                let point = Point::new(i, j, ch);
                if point.data != '.' {
                    row.push(point);
                    antennas.entry(point.data).or_insert(Vec::new()).push(&point);
                }
            }
            grid.push(row);
        }
        Map {
            grid,
            antennas,
            height: grid.len(),
            width: grid[0].len(),
        }
    }
}

#[derive(Clone, Copy)]
struct Point {
    row: usize,
    col: usize,
    data: char,
}

impl Point {
    fn new(row: usize, col: usize, data: char) -> Self {
        Point {
            row,
            col,
            data,
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
        assert_eq!(res, 14);
    }

    // #[test]
    // fn test_ex2() {
    //     let input = input::read_example();
    //     let res = exercise2(&input);
    //     assert_eq!(res, 11387);
    // }
}
