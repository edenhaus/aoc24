use std::collections::HashMap;

use super::common::Report;
use std::time::Instant;


const INPUT: &'static str = include_str!("input.txt");

pub fn solve(input: &str)-> Report<u32, u32> {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut numbers: Vec<Vec<u32>> = Vec::new();
    input.lines().for_each(|line| {
        let trimmed_line = line.trim();
        if trimmed_line.contains("|") {
            //rules
            let parts = trimmed_line.split("|").map(|n| n.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();
            let first = parts[0];
            let second = parts[1];
            rules.entry(first).or_insert(Vec::new()).push(second);
        } else if trimmed_line.contains(",") {
            //numbers
            let numbers_line: Vec<u32> = trimmed_line.split(",").map(|n| n.parse::<u32>().unwrap()).collect();
            numbers.push(numbers_line);
        }
    });

    let mut first_result:u32 = 0;
    let mut second_result:u32 = 0;

    for line in numbers {
        let mut new_line: Vec<u32> = Vec::new();

        for &number in &line {
            let mut inserted= false;

            if let Some(before) = rules.get(&number) {
                if let Some(insert_at) = new_line.iter().position(|&x| before.contains(&x)) {
                    new_line.insert(insert_at, number);
                    inserted = true;
                }
            }

            if !inserted {
                new_line.push(number);
            }
        }

        if line == new_line{
            first_result += line[line.len()/2];
        } else {
            second_result += new_line[new_line.len()/2];
        }
    }

    Report{exercise:5, first:first_result, second:second_result}
}


pub fn run() {
    let now = Instant::now();
    println!("{}",solve(&INPUT));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let report = solve(input);
        assert_eq!(report.first, 143);
        assert_eq!(report.second, 123);
    }



    #[test]
    fn challenge() {
        let report = solve(&INPUT);
        assert_eq!(report.first, 6041);
        assert_eq!(report.second, 4884);
    }
}