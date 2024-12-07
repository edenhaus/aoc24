use common::Report;
use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

fn calculate(target: u64, calculated: u64, numbers: &[u64], allow_concatenation: bool) -> bool {
    match target.cmp(&calculated) {
        std::cmp::Ordering::Equal => {
            if numbers.is_empty() {
                return true;
            }
        }
        std::cmp::Ordering::Less => return false,
        std::cmp::Ordering::Greater => {
            if numbers.is_empty() {
                return false;
            }
        }
    }

    let num = numbers[0];
    if calculate(target, calculated + num, &numbers[1..], allow_concatenation) {
        return true;
    }
    if calculate(target, calculated * num, &numbers[1..], allow_concatenation) {
        return true;
    }
    if allow_concatenation {
        let new_calculated = (calculated.to_string() + &num.to_string())
            .parse::<u64>()
            .unwrap();
        if calculate(target, new_calculated, &numbers[1..], allow_concatenation) {
            return true;
        }
    }

    false
}

fn first(values: Vec<(u64, Vec<u64>)>) -> (u64, Vec<(u64, Vec<u64>)>) {
    let mut sum: u64 = 0;
    let mut for_second: Vec<(u64, Vec<u64>)> = Vec::new();
    for (target, numbers) in values {
        if calculate(target, 0, &numbers, false) {
            sum += target;
        } else {
            for_second.push((target, numbers));
        }
    }
    (sum, for_second)
}

fn second(values: Vec<(u64, Vec<u64>)>) -> u64 {
    let mut sum: u64 = 0;
    for (target, numbers) in values {
        if calculate(target, 0, &numbers, true) {
            sum += target;
        }
    }
    sum
}

pub fn solve(input: &str) -> Report<u64, u64> {
    let values = input
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            let target = parts.next().unwrap().trim().parse::<u64>().unwrap();
            let numbers = parts
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            (target, numbers)
        })
        .collect::<Vec<(u64, Vec<u64>)>>();

    let (first, for_second) = first(values);
    Report {
        exercise: 6,
        first,
        second: first + second(for_second),
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
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let report = solve(input);
        assert_eq!(report.first, 3749);
        assert_eq!(report.second, 11387);
    }

    #[test]
    fn challenge() {
        let report = solve(INPUT);
        assert_eq!(report.first, 12940396350192);
        assert_eq!(report.second, 106016735664498);
    }
}
