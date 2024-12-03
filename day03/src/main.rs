use regex::Regex;
use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
}

fn exercise1(input: &String) -> u64 {
    let re = Regex::new(r"mul\((?<n1>[+-]?\d+),(?<n2>[+-]?\d+)\)").unwrap();
    let res: u64 = re
        .captures_iter(input)
        .map(|caps| {
            caps.name("n1").unwrap().as_str().parse::<u64>().unwrap()
                * caps.name("n2").unwrap().as_str().parse::<u64>().unwrap()
        })
        .sum();
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 161);
    }
}
