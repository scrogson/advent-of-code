use std::error::Error;
use std::io::{self, Read};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let answer = part1(&input)?;
    println!("Part 1: {}", answer);

    let answer = part2(&input)?;
    println!("Part 2: {}", answer);

    Ok(())
}

pub fn fuel_requirement(mass: i64) -> i64 {
    let fuel = (mass as f64 / 3.0).floor() as i64 - 2;

    if fuel <= 0 {
        return mass;
    }

    fuel
}

pub fn part1(input: &str) -> Result<i64> {
    let reqs: i64 = input
        .lines()
        .map(|line| {
            let mass: i64 = line.parse().unwrap();
            fuel_requirement(mass)
        })
        .sum();
    Ok(reqs)
}

pub fn part2(input: &str) -> Result<i64> {
    let reqs: i64 = input
        .lines()
        .map(|line| {
            let mut fuel: Vec<i64> = Vec::new();
            let mut mass: i64 = line.parse().unwrap();

            loop {
                let req = fuel_requirement(mass);

                if req == mass {
                    break;
                }

                fuel.push(req);
                mass = req;
            }

            fuel.into_iter().sum::<i64>()
        })
        .sum();
    Ok(reqs)
}

#[test]
fn test_fuel_requirement() {
    assert_eq!(fuel_requirement(5), 5);
    assert_eq!(fuel_requirement(12), 2);
    assert_eq!(fuel_requirement(14), 2);
    assert_eq!(fuel_requirement(1969), 654);
    assert_eq!(fuel_requirement(100756), 33583);
}

#[test]
fn test_part2() {
    assert_eq!(part2("100756").unwrap(), 50346);
}
