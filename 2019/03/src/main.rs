use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();

    let now = std::time::Instant::now();

    let wire_a = Wire::new(lines.next().unwrap());
    let wire_b = Wire::new(lines.next().unwrap());

    let mut intersections = wire_a.intersection(&wire_b);
    intersections.sort_by(|a, b| a.distance().cmp(&b.distance()));

    println!("Part 1: {:?}", intersections[0].distance());
    println!("Part 2: {:?}", wire_a.fewest_steps(&wire_b));

    println!("Completed in {}ms", now.elapsed().as_millis());

    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

#[derive(Debug)]
struct Directions(Vec<Direction>);

#[derive(Clone, Copy, Debug, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Wire {
    points: HashSet<Point>,
    steps: HashMap<Point, u32>,
}

impl From<&str> for Directions {
    fn from(s: &str) -> Directions {
        let inner = s.split(',').map(Direction::from).collect();
        Directions(inner)
    }
}

impl From<&str> for Direction {
    fn from(s: &str) -> Direction {
        let n: u32 = s[1..].parse().unwrap();

        match &s[..1] {
            "U" => Direction::Up(n),
            "D" => Direction::Down(n),
            "L" => Direction::Left(n),
            "R" => Direction::Right(n),
            _ => panic!("Invalid input"),
        }
    }
}

impl Point {
    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Wire {
    fn new(input: &str) -> Wire {
        let directions = Directions::from(input);
        let mut points = HashSet::new();
        let mut steps: HashMap<Point, u32> = HashMap::new();
        let mut i = 0;
        let mut x = 0;
        let mut y = 0;

        let point = Point { x, y };
        points.insert(point);
        steps.insert(point, i);

        for dir in directions.0 {
            match dir {
                Direction::Up(n) => {
                    for _ in 0..n {
                        i += 1;
                        y += 1;
                        let point = Point { x, y };
                        points.insert(point);
                        steps.insert(point, i);
                    }
                }
                Direction::Down(n) => {
                    for _ in 0..n {
                        i += 1;
                        y -= 1;
                        let point = Point { x, y };
                        points.insert(point);
                        steps.insert(point, i);
                    }
                }
                Direction::Left(n) => {
                    for _ in 0..n {
                        i += 1;
                        x -= 1;
                        let point = Point { x, y };
                        points.insert(point);
                        steps.insert(point, i);
                    }
                }
                Direction::Right(n) => {
                    for _ in 0..n {
                        i += 1;
                        x += 1;
                        let point = Point { x, y };
                        points.insert(point);
                        steps.insert(point, i);
                    }
                }
            }
        }

        Wire { points, steps }
    }

    fn intersection(&self, other: &Wire) -> Vec<Point> {
        self.points
            .intersection(&other.points)
            .into_iter()
            .filter(|p| p.distance() != 0)
            .map(|p| p.clone())
            .collect::<Vec<Point>>()
    }

    fn steps(&self, point: &Point) -> &u32 {
        self.steps.get(point).unwrap()
    }

    fn fewest_steps(&self, other: &Wire) -> u32 {
        let intersections = self.intersection(&other);

        let mut steps = Vec::new();
        for point in &intersections {
            let a = self.steps(&point);
            let b = other.steps(&point);
            steps.push(a + b);
        }
        steps.sort();
        steps[0]
    }
}

#[test]
fn test_parsing_direction() {
    assert_eq!(Direction::from("U83"), Direction::Up(83));
    assert_eq!(Direction::from("D71"), Direction::Down(71));
    assert_eq!(Direction::from("L12"), Direction::Left(12));
    assert_eq!(Direction::from("R75"), Direction::Right(75));
}

#[test]
fn test_part_1() {
    let mut input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83\n".lines();
    let wire_a = Wire::new(input.next().unwrap());
    let wire_b = Wire::new(input.next().unwrap());

    let mut intersections = wire_a.intersection(&wire_b);
    intersections.sort_by(|a, b| a.distance().cmp(&b.distance()));

    assert_eq!(intersections[0].distance(), 159);

    input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7\n"
        .lines();
    let wire_a = Wire::new(input.next().unwrap());
    let wire_b = Wire::new(input.next().unwrap());

    let mut intersections = wire_a.intersection(&wire_b);
    intersections.sort_by(|a, b| a.distance().cmp(&b.distance()));

    assert_eq!(intersections[0].distance(), 135);
}

#[test]
fn test_part_2() {
    let mut input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83\n".lines();
    let wire_a = Wire::new(input.next().unwrap());
    let wire_b = Wire::new(input.next().unwrap());

    let steps = wire_a.fewest_steps(&wire_b);

    assert_eq!(steps, 610);

    input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7\n"
        .lines();
    let wire_a = Wire::new(input.next().unwrap());
    let wire_b = Wire::new(input.next().unwrap());

    let steps = wire_a.fewest_steps(&wire_b);

    assert_eq!(steps, 410);
}
