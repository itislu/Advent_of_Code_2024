use core::fmt;
use std::mem::swap;
use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    let mut disk = Disk::new(input);

    disk.partition();
    println!("DISK:\n{}", disk);
    disk.checksum()
}

fn exercise2(input: &str) -> usize {
    let mut disk = Disk::new(input);

    disk.defragment();
    println!("DISK:\n{}", disk);
    disk.checksum()
}

struct Disk {
    data: Vec<Byte>,
    first_free_byte: usize,
    last_file_byte: usize,
    // first_free_range: usize,
    // last_file_range: usize,
    size: usize,
}

impl Disk {
    fn new(input: &str) -> Self {
        let mut disk = Disk {
            data: Vec::new(),
            first_free_byte: 0,
            last_file_byte: 0,
            // first_free_range: 0,
            // last_file_range: 0,
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

    fn defragment(&mut self) {
        let mut end: usize = self.size;
        while let Some(file_range) = self.last_file_range(end) {
            let range_len = file_range.len();
            if let Some(free_range) = self.first_free_range(range_len) {
                for (free_idx, file_idx) in free_range.zip(file_range) {
                    self.swap(free_idx, file_idx);
                }
            }
            end -= range_len;
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

    fn first_free_range(&self, size: usize) -> Option<std::ops::Range<usize>> {
        let mut start: usize = 0;
        loop {
            start = self.first_free_byte(start)?.index;
            let mut end = start;
            while end < self.size && !self.data[end].is_file() {
                end += 1;
            }
            if end - start >= size {
                return Some(start..end);
            }
            if end == self.size {
                return None;
            }
            start = end;
        }
    }

    fn last_file_range(&self, mut end: usize) -> Option<std::ops::Range<usize>> {
        let last_file_byte = self.last_file_byte(end)?;
        end = last_file_byte.index + 1;
        let mut start = end - 1;
        while start > 0
            && self.data[start].is_file()
            && self.data[start].file_id == last_file_byte.file_id
        {
            start -= 1;
        }
        Some(start..end)
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

    #[test]
    fn test_ex2() {
        let input = input::read_example();
        let res = exercise2(&input);
        assert_eq!(res, 2858);
    }
}
