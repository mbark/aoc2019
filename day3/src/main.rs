use std::{collections::{HashMap, HashSet}, fs};

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let trimmed = content.trim();

    let wires: Vec<&str> = trimmed.split("\n").collect();
    let wire1 = wires[0].split(",").collect();
    let wire2 = wires[1].split(",").collect();

    println!("first {}", first(&wire1, &wire2));
    println!("second {}", second(&wire1, &wire2));
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point(i32, i32);

impl Direction {
    fn apply(&self, p: &Point) -> Point {
        match self {
            Direction::Up => Point(p.0, p.1 - 1),
            Direction::Down => Point(p.0, p.1 + 1),
            Direction::Left => Point(p.0 - 1, p.1),
            Direction::Right => Point(p.0 + 1, p.1),
        }
    }
}

fn first(wire1: &Vec<&str>, wire2: &Vec<&str>) -> i32 {
    let draw_wire = |wire: &Vec<&str>| -> HashSet<Point> {
        let mut prev = Point(0, 0);
        let mut points: HashSet<Point> = HashSet::new();

        for op in wire {
            let dir = match op.chars().nth(0).unwrap() {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("unrecognized direction"),
            };

            let len: usize = op[1..].parse::<usize>().unwrap();

            for _i in 0..len {
                let p = dir.apply(&prev);
                prev = Point(p.0, p.1);
                points.insert(p);
            }
        }

        return points;
    };

    draw_wire(wire1)
        .intersection(&draw_wire(wire2))
        .map(|p| p.0.abs() + p.1.abs())
        .min()
        .unwrap()
}

fn second(wire1: &Vec<&str>, wire2: &Vec<&str>) -> i32 {
    let draw_wire = |wire: &Vec<&str>| -> HashMap<Point, usize> {
        let mut prev = Point(0, 0);
        let mut points: HashMap<Point, usize> = HashMap::new();
        let mut length = 0;

        for op in wire {
            let dir = match op.chars().nth(0).unwrap() {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("unrecognized direction"),
            };

            let len: usize = op[1..].parse::<usize>().unwrap();

            for _i in 0..len {
                length += 1;
                let p = dir.apply(&prev);
                prev = Point(p.0, p.1);
                points.insert(p, length);
            }
        }

        return points;
    };

    let points1 = draw_wire(wire1);
    let points2 = draw_wire(wire2);

    points1
        .keys()
        .filter(|p| points2.contains_key(p))
        .map(|p| points1.get(p).unwrap() + points2.get(p).unwrap())
        .min()
        .unwrap() as i32
}
