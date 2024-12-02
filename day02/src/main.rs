use utils::input;

const DAY: u32 = 2;

fn main() {
    let input = input::read_file(&format!("day{:02}.txt", DAY));
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &String) -> usize {
    let mut res: usize = 0;
    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        if is_close(&numbers) {
            res += 1;
        }
    }
    res
}

fn exercise2(input: &String) -> usize {
    let mut res: usize = 0;
    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        for skip in 0..numbers.len() {
            let one_less: Vec<i32> = numbers
                .iter()
                .enumerate()
                .filter_map(|(i, &n)| if i != skip { Some(n) } else { None })
                .collect();
            if is_close(&one_less) {
                res += 1;
                break;
            }
        }
    }
    res
}

fn is_close(numbers: &Vec<i32>) -> bool {
    for window in numbers.windows(2) {
        if (window[0] - window[1] < 0) != (numbers[0] - numbers[1] < 0)
            || window[0].abs_diff(window[1]) < 1
            || window[0].abs_diff(window[1]) > 3
        {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_file(&format!("day{:02}_example.txt", DAY));
        let res = exercise1(&input);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_ex2() {
        let input = input::read_file(&format!("day{:02}_example.txt", DAY));
        let res = exercise2(&input);
        assert_eq!(res, 4);
    }
}
