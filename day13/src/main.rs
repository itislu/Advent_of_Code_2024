use std::ops::{Add, Mul};
use itertools::Itertools;
use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
}

const COST_A: usize = 3;
const COST_B: usize = 1;
const MAX_PRESSES: usize = 100;

fn exercise1(input: &str) -> usize {
    let mut res: usize = 0;
    for (button_a, button_b, prize) in parse_input(input) {
        if let Some(min_cost) = try_all_combinations(button_a, button_b, prize) {
            res += min_cost;
        }
    }
    res
}

fn try_all_combinations(button_a: Button, button_b: Button, prize: Position) -> Option<usize> {
    let mut min_cost: Option<usize> = None;

    for combination in (0..=MAX_PRESSES).permutations(2) {
        if let Some(cost) = calc_cost(
            Button::new(
                button_a.movement.x,
                button_a.movement.y,
                combination[0],
                COST_A,
            ),
            Button::new(
                button_b.movement.x,
                button_b.movement.y,
                combination[1],
                COST_B,
            ),
            prize,
        ) {
            if min_cost.map_or(true, |min_cost| cost < min_cost) {
                min_cost = Some(cost);
            }
        }
    }
    min_cost
}

fn calc_cost(button_a: Button, button_b: Button, prize: Position) -> Option<usize> {
    if button_a.apply() + button_b.apply() == prize {
        Some(button_a.get_cost() + button_b.get_cost())
    } else {
        None
    }
}

struct Button {
    movement: Position,
    presses: usize,
    cost: usize,
}

impl Button {
    fn new(x: usize, y: usize, presses: usize, cost: usize) -> Self {
        Button {
            movement: Position::new(x, y),
            presses,
            cost,
        }
    }

    fn apply(&self) -> Position {
        self.movement * self.presses
    }

    fn get_cost(&self) -> usize {
        self.presses * self.cost
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<usize> for Position {
    type Output = Position;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = (Button, Button, Position)> + '_ {
    input.split("\n\n").map(|block| {
        let numbers: Vec<Vec<usize>> = block.lines().map(|line| parse_numbers(line)).collect();
        (
            Button::new(numbers[0][0], numbers[0][1], 0, COST_A),
            Button::new(numbers[1][0], numbers[1][1], 0, COST_B),
            Position::new(numbers[2][0], numbers[2][1]),
        )
    })
}

fn parse_numbers(s: &str) -> Vec<usize> {
    let mut numbers: Vec<usize> = Vec::new();
    let mut current_number = String::new();

    for c in s.chars() {
        if c.is_digit(10) {
            current_number.push(c);
        } else if !current_number.is_empty() {
            numbers.push(current_number.parse().unwrap());
            current_number.clear();
        }
    }
    if !current_number.is_empty() {
        numbers.push(current_number.parse().unwrap());
    }
    numbers
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 480);
    }
}
