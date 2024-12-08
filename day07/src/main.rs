use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    // println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &String) -> i64 {
    let mut res: i64 = 0;

    for line in input.lines() {
        let target = parse_target(line);
        let numbers = parse_numbers(line);
        let mut punch_card: usize = 2_usize.pow(numbers.len() as u32 - 1);

        loop {
            let operator = Operator::new(punch_card, &numbers);
            if operator.calculate() == target {
                res += target;
                break;
            }
            if punch_card == 0 {
                break;
            }
            punch_card -= 1;
        }
    }
    res
}

struct Operator<'a> {
    punch_card: usize,
    numbers: &'a Vec<i64>,
}

impl<'a> Operator<'a> {
    fn new(punch_card: usize, numbers: &'a Vec<i64>) -> Self {
        Operator {
            punch_card,
            numbers,
        }
    }

    fn calculate(&self) -> i64 {
        if self.numbers.len() == 0 {
            return 0;
        }
        let mut res: i64 = self.numbers[0];
        for (i, number) in self.numbers.iter().skip(1).enumerate() {
            match (self.punch_card >> i) & 1 {
                0 => res += number,
                1 => res *= number,
                _ => panic!(),
            };
        }
        res
    }
}

fn parse_target(line: &str) -> i64 {
    line.split(':').nth(0).unwrap().parse().unwrap()
}

fn parse_numbers(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .skip(1)
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
        assert_eq!(res, 3749);
    }

    // #[test]
    // fn test_ex2() {
    //     let input = input::read_example();
    //     let res = exercise2(&input);
    //     assert_eq!(res, 6);
    // }
}
