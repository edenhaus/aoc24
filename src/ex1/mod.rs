use std::collections::HashMap;

const INPUT: &'static str = include_str!("input.txt");

fn abs(x: i64) -> i64 {
    if x > 0 {
        x
    } else {
        -x
    }
}

pub fn solve(input: &str)-> (i64, i64) {
    let words: Vec<&str> = input.split_whitespace().collect();
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();
    for number in 0..(words.len()) {
        let value:i64 = words[number].parse().unwrap();
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
            let count:i64 = map_count[value];
            map_count.insert(value, count + 1);
        }
    }

    for value in left.iter() {
        if map_count.contains_key(value) {
            similarity_score += value*map_count[value];
        }
    }
    (sum, similarity_score)
}


pub fn run() {
    let (sum, similarity) = solve(&INPUT);
    println!("Result ex1: sum: {:?} similarity: {:?}", sum, similarity);
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
        let (sum, similarity) = solve(input);
        assert_eq!(sum, 11);
        assert_eq!(similarity, 31);
    }

    #[test]
    fn challenge() {
        let (sum, similarity) = solve(&INPUT);
        assert_eq!(sum, 1151792);
        assert_eq!(similarity, 21790168);
    }
}