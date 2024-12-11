use common::Report;
use rustc_hash::FxHashMap;
use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Eq, PartialEq, Hash)]
struct Stone {
    num: u64,
    blinks: u64,
}

fn round(cache: &mut FxHashMap<Stone, u64>, stone_num: u64, blinks: u64) -> u64 {
    let key = Stone { num:stone_num, blinks };
    if let Some(&value) = cache.get(&key) {
        return value;
    }

    let mut next = 1;

    if blinks == 1 {
        if stone_num != 0 && (stone_num.ilog10() + 1) % 2 == 0 {
            return 2;
        }
    } else if stone_num == 0 {
        next = round(cache, 1, blinks - 1);
    } else {
        let digits = stone_num.ilog10() + 1;
        if digits % 2 == 0 {
            let power = 10_u64.pow(digits / 2);
            next =
                round(cache, stone_num / power, blinks - 1) + round(cache, stone_num % power, blinks - 1);
        } else {
            next = round(cache, stone_num * 2024, blinks - 1);
        }
    }

    cache.insert(key, next);
    next
}

pub fn solve(input: &str) -> Report<u64, u64> {
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let mut cache: FxHashMap<Stone, u64> = FxHashMap::default();
    let first = stones
        .iter()
        .map(|&stone| round(&mut cache, stone, 25))
        .sum();
    let second = stones
        .iter()
        .map(|&stone| round(&mut cache, stone, 75))
        .sum();

    Report {
        exercise: 11,
        first,
        second,
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
        let input = "125 17";
        let report = solve(input);
        assert_eq!(report.first, 55312);
        assert_eq!(report.second, 65601038650482);
    }

    #[test]
    fn challenge() {
        let report = solve(INPUT);
        assert_eq!(report.first, 220722);
        assert_eq!(report.second, 261952051690787);
    }
}
