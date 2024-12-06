use std::{cmp::Ordering, time::Instant};

use common::Report;

const INPUT: &str = include_str!("input.txt");

fn difference(x: u32, y: u32) -> u32 {
    if x > y {
        x - y
    } else {
        y - x
    }
}

fn check_line(numbers: &[u32]) -> bool {
    enum Mode {
        Increasing,
        Decreasing,
        Unknown,
    }

    let mut mode = Mode::Unknown;
    for idx in 1..(numbers.len()) {
        let current = numbers[idx];
        let previous = numbers[idx - 1];

        match current.cmp(&previous) {
            Ordering::Less => {
                // Decreasing
                match mode {
                    Mode::Unknown => mode = Mode::Decreasing,
                    Mode::Increasing => return false,
                    _ => {}
                }
            }
            Ordering::Greater => {
                // Increasing
                match mode {
                    Mode::Unknown => mode = Mode::Increasing,
                    Mode::Decreasing => return false,
                    _ => {}
                }
            }
            _ => {}
        }

        let difference = difference(current, previous);
        if !(1..=3).contains(&difference) {
            return false;
        }
    }
    true
}

pub fn solve(input: &str) -> Report<u32, u32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut safe_lines_first: u32 = 0;
    let mut safe_lines_second: u32 = 0;

    for line in lines.iter() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let numbers = words
            .iter()
            .map(|&x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if check_line(&numbers) {
            safe_lines_first += 1;
        } else {
            for remove_idx in 0..numbers.len() {
                let mut new_numbers = numbers.clone();
                new_numbers.remove(remove_idx);
                if check_line(&new_numbers) {
                    safe_lines_second += 1;
                    break;
                }
            }
        }
    }

    Report {
        exercise: 2,
        first: safe_lines_first,
        second: safe_lines_first + safe_lines_second,
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
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let report = solve(input);
        assert_eq!(report.first, 2);
        assert_eq!(report.second, 4);
    }

    #[test]
    fn challenge() {
        let report = solve(INPUT);
        assert_eq!(report.first, 549);
        assert_eq!(report.second, 589);
    }
}
