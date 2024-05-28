use std::collections::HashSet;
use std::fs::read_to_string;

enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn next(self, direction: Direction) -> Coord {
        match direction {
            Direction::North => Coord {
                x: self.x,
                y: self.y + 1,
            },
            Direction::East => Coord {
                x: self.x + 1,
                y: self.y,
            },
            Direction::West => Coord {
                x: self.x - 1,
                y: self.y,
            },
            Direction::South => Coord {
                x: self.x,
                y: self.y - 1,
            },
        }
    }
}

const ORIGIN: Coord = Coord { x: 0, y: 0 };

fn advance(curr: &Coord, count: &mut HashSet<Coord>, input: char) -> Coord {
    let next = match input {
        '^' => {
            let next = curr.next(Direction::North);
            count.insert(next);
            next
        }
        '>' => {
            let next = curr.next(Direction::East);
            count.insert(next);
            next
        }
        'v' => {
            let next = curr.next(Direction::South);
            count.insert(next);
            next
        }
        '<' => {
            let next = curr.next(Direction::West);
            count.insert(next);
            next
        }
        _ => panic!("Unexpected direction!"),
    };

    next
}

fn deliver<F>(data: &String, santa: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let mut santa_curr = ORIGIN;
    let mut robot_curr = ORIGIN;
    let mut count: HashSet<Coord> = HashSet::new();
    count.insert(ORIGIN);
    for (i, c) in data.chars().enumerate() {
        if santa(i) {
            santa_curr = advance(&santa_curr, &mut count, c);
        } else {
            robot_curr = advance(&robot_curr, &mut count, c);
        }
    }

    count.len()
}

fn main() -> anyhow::Result<()> {
    let data = read_to_string("y2015/src/bin/day3-2015/input.txt")?;
    let part1 = deliver(&data, |_| true);
    let part2 = deliver(&data, |i| i % 2 == 0);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
