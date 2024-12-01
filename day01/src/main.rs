use utils::input;

fn main() {
    let input = input::read_file(1, false);
    let res = similarity_score(input);
    println!("{}", res);
}

fn similarity_score(input: String) -> u64 {
    let mut list1: Vec<u64> = Vec::new();
    let mut list2: Vec<u64> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();
        list1.push(parts.next().unwrap().parse().unwrap());
        list2.push(parts.next().unwrap().parse().unwrap());
    }
    list1.sort();
    list2.sort();

    let mut res = 0;
    for (a, b) in list1.iter().zip(list2.iter()) {
        res += a.abs_diff(*b);
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = input::read_file(1, true);
        let res = similarity_score(input);
        assert_eq!(res, 11);
    }
}
