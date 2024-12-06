use std::time::Instant;

use common::Report;
use regex::Regex;

const INPUT: &str = include_str!("input.txt");

fn first(input: &str) -> u32 {
    let re = Regex::new(r"mul\(\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();
    let numbers: Vec<(u32, u32)> = re
        .captures_iter(input)
        .map(|caps| {
            let (_, [first, second]) = caps.extract();
            (
                first.parse::<u32>().unwrap(),
                second.parse::<u32>().unwrap(),
            )
        })
        .collect();

    let mut result = 0;
    for (first_number, second_number) in numbers.iter() {
        result += first_number * second_number;
    }
    result
}

fn second(input: &str) -> u32 {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\(\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();
    let mut enabled = true;
    let numbers: Vec<(u32, u32)> = re
        .captures_iter(input)
        .map(|caps| {
            if let Some(cmd) = caps.get(0) {
                match cmd.as_str() {
                    "do()" => enabled = true,
                    "don't()" => enabled = false,
                    _ => {
                        if enabled {
                            if let (Some(first), Some(second)) = (caps.get(1), caps.get(2)) {
                                let first_number = first.as_str().parse::<u32>().unwrap();
                                let second_number = second.as_str().parse::<u32>().unwrap();
                                return (first_number, second_number);
                            }
                        }
                    }
                }
            }

            (0, 0)
        })
        .collect();

    let mut result = 0;
    for (first_number, second_number) in numbers.iter() {
        result += first_number * second_number;
    }
    result
}

pub fn solve(input: &str) -> Report<u32, u32> {
    Report {
        exercise: 3,
        first: first(input),
        second: second(input),
    }
}

pub fn main() {
    let now = Instant::now();
    let result = solve(INPUT);
    let elapsed = now.elapsed();
    println!("{}, elapsed: {:.2?}", result, elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let report = solve(input);
        assert_eq!(report.first, 161);
        assert_eq!(report.second, 161);
    }

    #[test]
    fn example2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let report = solve(input);
        assert_eq!(report.first, 161);
        assert_eq!(report.second, 48);
    }

    #[test]
    fn challenge() {
        let report = solve(INPUT);
        assert_eq!(report.first, 161085926);
        assert_eq!(report.second, 82045421);
    }
}
