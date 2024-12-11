use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
}

fn exercise1(input: &str) -> usize {
    let mut stones: Vec<usize> = parse_stones(input);

    for _ in 0..25 {
        let mut new_stones: Vec<usize> = Vec::with_capacity(stones.len());
        for stone in stones {
            if stone == 0 {
                new_stones.push(1);
            } else if let Some(split) = split_if_even_digits(stone) {
                split.iter().for_each(|&n| new_stones.push(n));
            } else {
                new_stones.push(stone * 2024);
            }
        }
        stones = new_stones;
    }
    stones.len()
}

fn split_if_even_digits(num: usize) -> Option<[usize; 2]> {
    let string = num.to_string();
    if string.len() % 2 == 0 {
        let (left, right) = string.split_at(string.len() / 2);
        Some([left.parse().unwrap(), right.parse().unwrap()])
    } else {
        None
    }
}

fn parse_stones(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 55312);
    }
}
