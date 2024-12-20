use common::Report;
use std::{collections::HashMap, time::Instant};

const INPUT: &str = include_str!("input.txt");

fn abs(x: i64) -> i64 {
    if x > 0 {
        x
    } else {
        -x
    }
}

pub fn solve(input: &str) -> Report<i64, i64> {
    let words: Vec<&str> = input.split_whitespace().collect();
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();
    for (number, value) in words.iter().enumerate() {
        let value: i64 = value.parse().unwrap();
        if number % 2 == 0 {
            left.push(value);
        } else {
            right.push(value);
        }
    }
    left.sort();
    right.sort();

    let mut sum: i64 = 0;
    for number in 0..(left.len()) {
        sum += abs(left[number] - right[number]);
    }

    let mut similarity_score: i64 = 0;
    let mut map_count: HashMap<&i64, i64> = HashMap::new();
    for value in right.iter() {
        if !map_count.contains_key(value) {
            map_count.insert(value, 1);
        } else {
            let count: i64 = map_count[value];
            map_count.insert(value, count + 1);
        }
    }

    for value in left.iter() {
        if map_count.contains_key(value) {
            similarity_score += value * map_count[value];
        }
    }
    Report {
        exercise: 1,
        first: sum,
        second: similarity_score,
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
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let report = solve(input);
        assert_eq!(report.first, 11);
        assert_eq!(report.second, 31);
    }

    #[test]
    fn challenge() {
        let report = solve(INPUT);
        assert_eq!(report.first, 1151792);
        assert_eq!(report.second, 21790168);
    }
}
