use std::error::Error;
use std::io::{self, Read};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let program: Vec<usize> = input
        .trim()
        .split(',')
        .map(|n| match n.parse::<usize>() {
            Ok(n) => n,
            Err(_) => panic!("failed to parse `{}` as an integer", n),
        })
        .collect();

    let answer = preprocess_and_run(program.clone(), 12, 2);
    println!("Part 1: {:?}", answer[0]);

    let now = std::time::Instant::now();
    let answer = part2(program);
    println!("Part 2: {} in {}", answer, now.elapsed().as_millis());

    Ok(())
}

fn preprocess_and_run(mut program: Vec<usize>, a: usize, b: usize) -> Vec<usize> {
    program[1] = a;
    program[2] = b;
    run(program)
}

fn part2(mut program: Vec<usize>) -> usize {
    let mut result = 0;
    for x in 0..=99 {
        for y in 0..=99 {
            if preprocess_and_run(program.clone(), x, y)[0] == 19690720 {
                result = 100 * x + y;
            }
        }
    }
    result
}

pub fn run(mut program: Vec<usize>) -> Vec<usize> {
    let mut pc = 0;

    macro_rules! read_reg {
        () => {{
            let val = program[pc] as usize;
            pc += 1;
            val
        }};
    }

    loop {
        let op = program[pc];
        pc += 1;

        match op {
            1 => {
                let a = read_reg!();
                let b = read_reg!();
                let c = read_reg!();
                program[c] = program[a] + program[b];
            }
            2 => {
                let a = read_reg!();
                let b = read_reg!();
                let c = read_reg!();
                program[c] = program[a] * program[b];
            }
            99 => {
                break;
            }
            unknown => {
                panic!("Unknown opcode: {}", unknown);
            }
        }
    }

    program.to_vec()
}

#[test]
fn test_part1() {
    assert_eq!(run(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
    assert_eq!(run(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
    assert_eq!(run(vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
    assert_eq!(
        run(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
        vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
    );
}
