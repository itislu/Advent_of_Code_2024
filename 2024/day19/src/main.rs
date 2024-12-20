use std::collections::HashMap;
use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    let available: Vec<&str> = parse_available(input);
    let wanted: Vec<&str> = parse_wanted(input);
    let mut cache: HashMap<&str, bool> = HashMap::new();
    let mut possible: usize = 0;

    for design in wanted {
        let selected_available: Vec<&str> = available
            .iter()
            .filter(|&a| design.contains(a))
            .copied()
            .collect();
        if is_possible(design, &selected_available, &mut cache) {
            possible += 1;
        }
    }
    possible
}

fn exercise2(input: &str) -> usize {
    let available: Vec<&str> = parse_available(input);
    let wanted: Vec<&str> = parse_wanted(input);
    let mut cache: HashMap<&str, usize> = HashMap::new();
    let mut possible: usize = 0;

    for design in wanted {
        let selected_available: Vec<&str> = available
            .iter()
            .filter(|&a| design.contains(a))
            .copied()
            .collect();
        possible += count_possible(design, &selected_available, &mut cache);
    }
    possible
}

fn is_possible<'a>(
    wanted: &'a str,
    available: &Vec<&str>,
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    if let Some(&cached_res) = cache.get(wanted) {
        return cached_res;
    }
    if wanted.is_empty() {
        return true;
    }
    for a in available {
        if let Some(substr) = wanted.strip_prefix(a) {
            if is_possible(substr, available, cache) {
                cache.insert(wanted, true);
                return true;
            }
        }
    }
    cache.insert(wanted, false);
    false
}

fn count_possible<'a>(
    wanted: &'a str,
    available: &Vec<&str>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&cached_res) = cache.get(wanted) {
        return cached_res;
    }
    if wanted.is_empty() {
        return 1;
    }
    let mut possible: usize = 0;

    for a in available {
        if let Some(substr) = wanted.strip_prefix(a) {
            possible += count_possible(substr, available, cache);
        }
    }
    cache.insert(wanted, possible);
    possible
}

fn parse_available(input: &str) -> Vec<&str> {
    input.lines().next().unwrap().split(", ").collect()
}

fn parse_wanted(input: &str) -> Vec<&str> {
    input.lines().skip(2).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_ex2() {
        let input = input::read_example();
        let res = exercise2(&input);
        assert_eq!(res, 16);
    }
}
