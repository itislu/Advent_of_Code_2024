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
        let mut operator_map: usize = numbers.len();

        loop {
            let mut operator = Operator::new(operator_map);

            for number in numbers.iter() {
                operator.calculate(*number);
            }
            if let Some(cur_res) = operator.res {
                if cur_res == target {
                    res += cur_res;
                    break;
                }
            } else {
                break;
            }
            if operator_map == 0 {
                break;
            }
            operator_map -= 1;
        }
    }
    res
}

struct Operator {
    operator_map: usize,
    size: usize,
    res: Option<i64>,
}

impl Operator {
    fn new(operator_map: usize) -> Self {
        Operator {
            operator_map,
            size: operator_map + 1,
            res: None,
        }
    }

    fn calculate(&mut self, n: i64) -> Option<i64> {
        if self.size == 0 {
            return None;
        }
        if let Some(cur_res) = self.res {
            match self.operator_map & 1 {
                0 => self.res = Some(cur_res + n),
                1 => self.res = Some(cur_res * n),
                _ => panic!(),
            };
            self.operator_map <<= 1;
            self.size -= 1;
        } else {
            self.res = Some(n);
        }
        self.res
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
