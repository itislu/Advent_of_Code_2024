use core::fmt;
use std::{collections::HashMap, iter, thread, time::Duration};
use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input, 100));
}

fn exercise1(input: &str, min_cheat: usize) -> usize {
    let racetrack = RaceTrack::new(input);
    let mut cheats = 0;
    let mut first_time = true;

    println!("{}", racetrack);

    for cur_tile in racetrack.iter() {
        // cheats += cur_tile
        //     .pos
        //     .distant_neighbors(2)
        //     .filter_map(|neighbor| racetrack.at(&neighbor))
        //     .filter(|cheat_tile| cheat_tile.time - cur_tile.time + 2 >= min_cheat as i64)
        //     .count();
        for cheat_tile in cur_tile
            .pos
            .distant_neighbors(2)
            .filter_map(|neighbor| racetrack.at(&neighbor))
            .filter(|cheat_tile| cheat_tile.time - cur_tile.time - 2 >= min_cheat as i64)
        {
            // #[cfg(debug_assertions)]
            // {
            //     print_track_with_cheat(&racetrack, cur_tile, cheat_tile, first_time);
            //     first_time = false;
            //     thread::sleep(Duration::from_millis(10));
            // }
            cheats += 1;
        }
    }
    cheats
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn neighbors(&self) -> impl Iterator<Item = Self> {
        self.distant_neighbors(1)
    }

    fn distant_neighbors(&self, distance: usize) -> impl Iterator<Item = Self> {
        let up = self
            .row
            .checked_sub(distance)
            .map(|row| Self::new(row, self.col));
        let down = Some(Self::new(self.row + distance, self.col));
        let right = Some(Self::new(self.row, self.col + distance));
        let left = self
            .col
            .checked_sub(distance)
            .map(|col| Self::new(self.row, col));

        [up, down, right, left].into_iter().flatten()
    }
}

struct TrackTile {
    pos: Position,
    time: i64,
    next: Option<Position>,
}

impl TrackTile {
    fn new(row: usize, col: usize) -> Self {
        Self {
            pos: Position::new(row, col),
            time: 0,
            next: None,
        }
    }
}

struct RaceTrack {
    track: HashMap<Position, TrackTile>,
    start: Position,
    finish: Position,
    height: usize,
    width: usize,
}

impl RaceTrack {
    fn new(input: &str) -> Self {
        let mut track: HashMap<Position, TrackTile> = HashMap::new();
        let mut start_opt: Option<Position> = None;
        let mut end_opt: Option<Position> = None;

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        start_opt = Some(Position::new(row, col));
                        track.insert(Position::new(row, col), TrackTile::new(row, col));
                    }
                    'E' => {
                        end_opt = Some(Position::new(row, col));
                        track.insert(Position::new(row, col), TrackTile::new(row, col));
                    }
                    '.' => {
                        track.insert(Position::new(row, col), TrackTile::new(row, col));
                    }
                    _ => {}
                }
            }
        }
        let mut racetrack = Self {
            track,
            start: start_opt.expect("No start tile found!"),
            finish: end_opt.expect("No end tile found!"),
            height: input.lines().count(),
            width: input.find('\n').unwrap(),
        };
        racetrack.build_track();
        racetrack
    }

    fn build_track(&mut self) {
        let mut cur = self.start;
        let mut prev = self.start;
        let mut time = 0;

        while let Some(next) = cur
            .neighbors()
            .filter_map(|neighbor| Some(self.at(&neighbor)?.pos))
            .find(|&pos| pos != prev)
        {
            let tile = self.at_mut(&cur).unwrap();
            tile.next = Some(next);
            tile.time = time;

            prev = cur;
            cur = next;
            time += 1;
        }
        self.at_mut(&cur).unwrap().time = time;
    }

    fn at(&self, pos: &Position) -> Option<&TrackTile> {
        self.track.get(&pos)
    }

    fn at_mut(&mut self, pos: &Position) -> Option<&mut TrackTile> {
        self.track.get_mut(&pos)
    }

    fn iter(&self) -> impl Iterator<Item = &TrackTile> {
        iter::successors(self.at(&self.start), |tile| self.at(&tile.next?))
    }
}

fn print_track_with_cheat(
    racetrack: &RaceTrack,
    cur_tile: &TrackTile,
    cheat_tile: &TrackTile,
    first_time: bool,
) {
    let mut buffer: Vec<Vec<char>> = vec![vec![' '; racetrack.width]; racetrack.height];

    for row in 0..racetrack.height {
        for col in 0..racetrack.width {
            if let Some(tile) = racetrack.track.get(&Position::new(row, col)) {
                if tile.pos == racetrack.start {
                    buffer[row][col] = 'S';
                } else if tile.pos == racetrack.finish {
                    buffer[row][col] = 'E';
                } else {
                    buffer[row][col] = '.';
                }
            } else {
                buffer[row][col] = '#';
            }
        }
    }
    buffer[cur_tile.pos.row][cur_tile.pos.col] = '0';
    buffer[(cur_tile.pos.row + cheat_tile.pos.row) / 2]
        [(cur_tile.pos.col + cheat_tile.pos.col) / 2] = '1';
    buffer[cheat_tile.pos.row][cheat_tile.pos.col] = '2';

    // if !first_time {
    //     print!("\x1B[{}A", racetrack.height);
    // }
    println!(
        "{}",
        buffer
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
}

impl fmt::Display for RaceTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                if let Some(tile) = self.track.get(&Position::new(row, col)) {
                    if tile.pos == self.start {
                        write!(f, "S")?;
                    } else if tile.pos == self.finish {
                        write!(f, "E")?;
                    } else {
                        write!(f, ".")?;
                    }
                } else {
                    write!(f, "#")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min2_ex1() {
        let input = input::read_example();
        let res = exercise1(&input, 2);
        assert_eq!(res, 44);
    }

    #[test]
    fn test_min4_ex1() {
        let input = input::read_example();
        let res = exercise1(&input, 4);
        assert_eq!(res, 30);
    }

    #[test]
    fn test_min6_ex1() {
        let input = input::read_example();
        let res = exercise1(&input, 6);
        assert_eq!(res, 16);
    }

    #[test]
    fn test_min8_ex1() {
        let input = input::read_example();
        let res = exercise1(&input, 8);
        assert_eq!(res, 14);
    }

    #[test]
    fn test_min10_ex1() {
        let input = input::read_example();
        let res = exercise1(&input, 10);
        assert_eq!(res, 10);
    }

    #[test]
    fn test_min12_ex1() {
        let input = input::read_example();
        let res = exercise1(&input, 12);
        assert_eq!(res, 8);
    }

    #[test]
    fn test_min20_ex1() {
        let input = input::read_example();
        let res = exercise1(&input, 20);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_min36_ex1() {
        let input = input::read_example();
        let res = exercise1(&input, 36);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_min38_ex1() {
        let input = input::read_example();
        let res = exercise1(&input, 38);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_min40_ex1() {
        let input = input::read_example();
        let res = exercise1(&input, 40);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_min64_ex1() {
        let input = input::read_example();
        let res = exercise1(&input, 64);
        assert_eq!(res, 1);
    }
}
