use utils::input;

const ITERATIONS: usize = 100;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
}

fn exercise1(input: &str) -> usize {
    let mut world = World::new(input);

    for _ in 0..ITERATIONS {
        world.mv_robots();
    }
    world.count_quadrant(Quadrant::TopLeft)
        * world.count_quadrant(Quadrant::TopRight)
        * world.count_quadrant(Quadrant::BottomLeft)
        * world.count_quadrant(Quadrant::BottomRight)
}

enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Quadrant {
    fn is_in(&self, robot: &Robot, world: &World) -> bool {
        let range_x = match self {
            Self::TopLeft | Self::BottomLeft => 0..world.rows / 2,
            Self::TopRight | Self::BottomRight => world.rows / 2 + 1..world.rows,
        };
        let range_y = match self {
            Self::TopLeft | Self::TopRight => 0..world.cols / 2,
            Self::BottomLeft | Self::BottomRight => world.cols / 2 + 1..world.cols,
        };
        range_x.contains(&robot.row) && range_y.contains(&robot.col)
    }
}

struct World {
    robots: Vec<Robot>,
    rows: i64,
    cols: i64,
}

impl World {
    fn new(input: &str) -> Self {
        let mut world = Self {
            robots: parse_input(input),
            rows: 0,
            cols: 0,
        };
        world.rows = world.robots.iter().map(|robot| robot.row).max().unwrap() + 1;
        world.cols = world.robots.iter().map(|robot| robot.col).max().unwrap() + 1;
        world
    }

    fn mv_robots(&mut self) {
        for robot in &mut self.robots {
            robot.mv();
            robot.row = (robot.row + self.rows) % self.rows;
            robot.col = (robot.col + self.cols) % self.cols;
        }
    }

    fn count_quadrant(&self, quadrant: Quadrant) -> usize {
        self.robots
            .iter()
            .filter(|robot| quadrant.is_in(robot, self))
            .count()
    }
}

struct Robot {
    row: i64,
    col: i64,
    vel_row: i64,
    vel_col: i64,
}

impl Robot {
    fn new(row: i64, col: i64, vel_row: i64, vel_col: i64) -> Self {
        Self {
            row,
            col,
            vel_row,
            vel_col,
        }
    }

    fn mv(&mut self) {
        self.row += self.vel_row;
        self.col += self.vel_col;
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (row, col, vel_row, vel_col) = parse_line(line);
            Robot::new(row, col, vel_row, vel_col)
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
