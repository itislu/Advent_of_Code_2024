use core::fmt;
use std::mem::swap;
use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
}

fn exercise1(input: &str) -> usize {
    let mut disk = Disk::new(input);

    disk.partition();
    println!("DISK:\n{}", disk);
    disk.checksum()
}

struct Disk {
    data: Vec<Byte>,
    first_free_byte: usize,
    last_file_byte: usize,
    size: usize,
}

impl Disk {
    fn new(input: &str) -> Self {
        let mut disk = Disk {
            data: Vec::new(),
            first_free_byte: 0,
            last_file_byte: 0,
            size: 0,
        };
        let mut index: usize = 0;

        for (i, ch) in input.chars().enumerate() {
            let file_id = match i % 2 {
                0 => Some(i / 2),
                1 => None,
                _ => panic!(),
            };
            for _ in 0..ch.to_digit(10).unwrap() {
                disk.data.push(Byte::new(file_id, index));
                index += 1;
            }
        }
        disk.size = disk.data.len();
        Disk::update(&mut disk);
        disk
    }

    fn partition(&mut self) {
        while self.first_free_byte < self.last_file_byte {
            self.swap(self.first_free_byte, self.last_file_byte);
        }
    }

    fn checksum(&self) -> usize {
        let mut checksum: usize = 0;
        for i in 0..=self.last_file_byte {
            if let Some(file_id) = self.data[i].file_id {
                checksum += i * file_id;
            }
        }
        checksum
    }

    fn update(&mut self) {
        if let Some(first_free_byte) = self.first_free_byte(0) {
            self.first_free_byte = first_free_byte.index;
        }
        if let Some(last_file_byte) = self.last_file_byte(self.size) {
            self.last_file_byte = last_file_byte.index
        }
    }

    fn swap(&mut self, mut low: usize, mut high: usize) {
        if high < low {
            swap(&mut high, &mut low);
        }
        self.data.swap(low, high);
        self.data[low].index = low;
        self.data[high].index = high;
        if low == self.first_free_byte {
            self.first_free_byte = self.first_free_byte(low).unwrap().index;
        }
        if high == self.last_file_byte {
            self.last_file_byte = self.last_file_byte(high + 1).unwrap().index;
        }
    }

    fn first_free_byte(&self, start: usize) -> Option<&Byte> {
        self.data.iter().skip(start).find(|byte| !byte.is_file())
    }

    fn last_file_byte(&self, end: usize) -> Option<&Byte> {
        self.data
            .iter()
            .rev()
            .skip(self.size.checked_sub(end)?)
            .find(|byte| byte.is_file())
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.data {
            write!(f, "{}", byte)?;
        }
        Ok(())
    }
}

struct Byte {
    file_id: Option<usize>,
    index: usize,
}

impl Byte {
    fn new(file_id: Option<usize>, index: usize) -> Self {
        Byte { file_id, index }
    }

    fn is_file(&self) -> bool {
        self.file_id.is_some()
    }
}

impl fmt::Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(file_id) = self.file_id {
            write!(f, "{}", file_id)
        } else {
            write!(f, ".")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 1928);
    }
}
