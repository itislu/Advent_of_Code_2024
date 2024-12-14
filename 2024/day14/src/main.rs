use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    exercise2(&input);
}

fn exercise1(input: &str) -> usize {
    const ITERATIONS: usize = 100;
    let mut world = World::new(input);

    for _ in 0..ITERATIONS {
        world.mv_robots();
    }
    world.count_quadrant(Quadrant::TopLeft)
        * world.count_quadrant(Quadrant::TopRight)
        * world.count_quadrant(Quadrant::BottomLeft)
        * world.count_quadrant(Quadrant::BottomRight)
}

/*
The `OFFSET` I got by noticing a kind of rectangle in the output after 2 iterations.
The `FREQUENCY` this rectangle then occured was after every 101 iterations.
I just did this 100 times, until it was very apparent that it is iteration #6668 with the christmas tree.
*/
fn exercise2(input: &str) {
    const OFFSET: usize = 2;
    const FREQUENCY: usize = 101;
    const FIRST_OCCURANCE: usize = 66;
    let mut world = World::new(input);

    for _ in 0..OFFSET {
        world.mv_robots();
    }
    for i in 0..=FIRST_OCCURANCE {
        println!("{}:\n{}\n", i * FREQUENCY + OFFSET, world);
        for _ in 0..FREQUENCY {
            world.mv_robots();
        }
    }
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

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut world: Vec<Vec<char>> = vec![vec!['.'; self.cols as usize]; self.rows as usize];

        for robot in &self.robots {
            let c: &mut char = &mut world[robot.row as usize][robot.col as usize];
            *c = match *c {
                '.' => '1',
                _ => (*c as u8 + 1) as char,
            }
        }
        for row in world {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
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
            let (col, row, vel_col, vel_row) = parse_line(line);
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
