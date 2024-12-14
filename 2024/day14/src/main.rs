use utils::input;

const COLUMNS: usize = 101;
const ROWS: usize = 103;
const ITERATIONS: usize = 100;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
}

fn exercise1(input: &str) -> usize {
    let mut robots: Vec<Robot> = parse_input(input);

    for robot in &mut robots {
        for _ in 0..ITERATIONS {
            robot.mv();
        }
    }
    count_quadrant(Quadrant::TopLeft, &robots)
        * count_quadrant(Quadrant::TopRight, &robots)
        * count_quadrant(Quadrant::BottomLeft, &robots)
        * count_quadrant(Quadrant::BottomRight, &robots)
}

enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Quadrant {
    fn is_in(&self, robot: &Robot) -> bool {
        let range_x = match self {
            Self::TopLeft | Self::BottomLeft => 0..robot.max_x / 2,
            Self::TopRight | Self::BottomRight => robot.max_x / 2 + 1..robot.max_x,
        };
        let range_y = match self {
            Self::TopLeft | Self::TopRight => 0..robot.max_y / 2,
            Self::BottomLeft | Self::BottomRight => robot.max_y / 2 + 1..robot.max_y,
        };
        range_x.contains(&robot.pos_x) && range_y.contains(&robot.pos_y)
    }
}

fn count_quadrant(quadrant: Quadrant, robots: &Vec<Robot>) -> usize {
    robots.iter().filter(|robot| quadrant.is_in(robot)).count()
}

struct Robot {
    pos_x: i64,
    pos_y: i64,
    vel_x: i64,
    vel_y: i64,
    max_x: i64,
    max_y: i64,
}

impl Robot {
    fn new(pos_x: i64, pos_y: i64, vel_x: i64, vel_y: i64) -> Self {
        Self {
            pos_x,
            pos_y,
            vel_x,
            vel_y,
            max_x: COLUMNS as i64,
            max_y: ROWS as i64,
        }
    }

    fn mv(&mut self) {
        self.pos_x = (self.max_x + self.pos_x + self.vel_x) % self.max_x;
        self.pos_y = (self.max_y + self.pos_y + self.vel_y) % self.max_y;
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (pos_x, pos_y, vel_x, vel_y) = parse_line(line);
            Robot::new(pos_x, pos_y, vel_x, vel_y)
        })
        .collect()
}

fn parse_line(line: &str) -> (i64, i64, i64, i64) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let pos: Vec<i64> = parts[0]
        .strip_prefix("p=")
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let vel: Vec<i64> = parts[1]
        .strip_prefix("v=")
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    (pos[0], pos[1], vel[0], vel[1])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 12);
    }
}
