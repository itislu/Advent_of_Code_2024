use utils::input;

fn main() {
    let input = input::read_file("day01.txt");
    println!("similarity score 1: {}", similarity_score1(&input));
    println!("similarity score 2: {}", similarity_score2(&input));
}

fn similarity_score1(input: &String) -> usize {
    let (mut list1, mut list2) = create_lists(input);
    list1.sort();
    list2.sort();

    let mut res = 0;
    for (a, b) in list1.iter().zip(list2.iter()) {
        res += a.abs_diff(*b);
    }
    res
}

fn similarity_score2(input: &String) -> usize {
    let (list1, list2) = create_lists(input);

    let mut res = 0;
    for a in list1.iter() {
        res += a * list2.iter().filter(|b| *b == a).count();
    }
    res
}

fn create_lists(input: &String) -> (Vec<usize>, Vec<usize>) {
    let mut list1: Vec<usize> = Vec::new();
    let mut list2: Vec<usize> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();
        list1.push(parts.next().unwrap().parse().unwrap());
        list2.push(parts.next().unwrap().parse().unwrap());
    }
    (list1, list2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_file("day01_example.txt");
        let res = similarity_score1(&input);
        assert_eq!(res, 11);
    }

    #[test]
    fn test_ex2() {
        let input = input::read_file("day01_example.txt");
        let res = similarity_score2(&input);
        assert_eq!(res, 31);
    }
}
