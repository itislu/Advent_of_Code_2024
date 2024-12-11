use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    let mut res: usize = 0;
    let stones: Vec<usize> = parse_stones(input);

    for stone in stones {
        res += split_stone_n_times(stone, 25);
    }
    res
}

fn exercise2(input: &str) -> usize {
    let mut res: usize = 0;
    let stones: Vec<usize> = parse_stones(input);

    for stone in stones {
        res += split_stone_n_times(stone, 75);
    }
    res
}

fn split_stone_n_times(mut stone: usize, mut n: usize) -> usize {
    let mut res: usize = 1;
    while n > 0 {
        if stone == 0 {
            stone = 1;
        } else if let Some((left, right)) = split_if_even_digits(stone) {
            res = split_stone_n_times(left, n - 1) + split_stone_n_times(right, n - 1);
            break;
        } else {
            stone *= 2024;
        }
        n -= 1;
    }
    res
}

fn split_if_even_digits(num: usize) -> Option<(usize, usize)> {
    let string = num.to_string();
    if string.len() % 2 == 0 {
        let (left, right) = string.split_at(string.len() / 2);
        Some((left.parse().unwrap(), right.parse().unwrap()))
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

    #[test]
    fn test_ex2() {
        let input = input::read_example();
        let res = exercise2(&input);
        println!("{}", res);
    }
}
