use std::error::Error;
use std::io::{self, Read};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

struct Password(Vec<u8>);

impl Password {
    fn new(n: u32) -> Password {
        let digits: Vec<u8> = n
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as u8)
            .collect();

        Password(digits)
    }

    fn is_valid(&self) -> bool {
        self.is_increasing() && self.has_double()
    }

    fn is_valid2(&self) -> bool {
        self.is_increasing() && self.has_balanced_doubles()
    }

    fn is_increasing(&self) -> bool {
        for pair in self.0.windows(2) {
            if pair[0] > pair[1] {
                return false;
            }
        }

        true
    }

    fn has_double(&self) -> bool {
        for pair in self.0.windows(2) {
            if pair[0] == pair[1] {
                return true;
            }
        }

        false
    }

    fn has_balanced_doubles(&self) -> bool {
        let mut stack = [0; 10];

        for n in &self.0 {
            stack[*n as usize] += 1;
        }

        if stack.contains(&2) {
            return true;
        }

        false
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let range: Vec<u32> = input
        .trim()
        .split('-')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let now = std::time::Instant::now();
    println!("Part 1: {:?}", part_1(range[0], range[1]));
    println!("Completed in {}ms", now.elapsed().as_millis());

    let now = std::time::Instant::now();
    println!("Part 2: {:?}", part_2(range[0], range[1]));
    println!("Completed in {}ms", now.elapsed().as_millis());

    Ok(())
}

fn part_1(start: u32, end: u32) -> u32 {
    let mut matching = 0;

    for n in start..=end {
        if Password::new(n).is_valid() {
            matching += 1;
        }
    }
    matching
}

fn part_2(start: u32, end: u32) -> u32 {
    let mut matching = 0;

    for n in start..=end {
        if Password::new(n).is_valid2() {
            matching += 1;
        }
    }
    matching
}

#[test]
fn test_part_1() {
    assert_eq!(Password::new(111111).is_valid(), true);
    assert_eq!(Password::new(223450).is_valid(), false);
    assert_eq!(Password::new(123789).is_valid(), false);
}

#[test]
fn test_part_2() {
    assert_eq!(Password::new(111122).is_valid2(), true);
    assert_eq!(Password::new(112233).is_valid2(), true);
    assert_eq!(Password::new(123444).is_valid2(), false);
}
