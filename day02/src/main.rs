use utils::input;

fn main() {
    let input = input::read_file("day02");
    println!("exercise 1: {}", exercise1(&input));
    // println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &String) -> usize {
    let mut res: usize = 0;
    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let mut is_close = true;
        for window in numbers.windows(2) {
            if (window[0] - window[1] < 0) != (numbers[0] - numbers[1] < 0)
                || window[0].abs_diff(window[1]) < 1
                || window[0].abs_diff(window[1]) > 3
            {
                is_close = false;
                break;
            }
        }
        if is_close {
            res += 1;
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_file("day02_example");
        let res = exercise1(&input);
        assert_eq!(res, 2);
    }

    // #[test]
    // fn test_ex2() {
    //     let input = input::read_file("day01_example");
    //     let res = exercise2(&input);
    //     assert_eq!(res, 31);
    // }
}
