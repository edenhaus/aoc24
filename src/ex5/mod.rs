use std::collections::HashMap;

use super::common::Report;


const INPUT: &'static str = include_str!("input.txt");

fn first(rules: &HashMap<u32, Vec<u32>>, numbers: Vec<Vec<u32>>)-> (u32, Vec<Vec<u32>>){
    let mut result:u32 = 0;
    let mut incorrect: Vec<Vec<u32>> = Vec::new();

    for line in numbers.iter() {
        let mut checked: Vec<u32> = Vec::new();
        let mut valid = true;
        for number in line.iter() {
            if let Some(before) = rules.get(number) {
                if checked.iter().any(|n| before.contains(n)) {
                    valid = false;
                    incorrect.push(line.clone());
                    break;
                }
            };
            checked.push(*number);
        }

        if valid{
            result += line[line.len()/2];
        }
    }

    (result, incorrect)
}

fn second(rules: &HashMap<u32, Vec<u32>>, numbers: Vec<Vec<u32>>)-> u32{
    let mut result:u32 = 0;

    for line in numbers.iter() {
        let mut new_line: Vec<u32> = Vec::new();
        for number in line.iter() {
            let mut inserted= false;

            if let Some(before) = rules.get(number) {
                let mut insert_at:i32 = -1;

                for idx in 0..new_line.len() {
                    let element = new_line[idx];
                    if before.contains(&element) {
                        insert_at = idx as i32;
                        break;
                    }
                }
                
                if insert_at != -1 {
                    new_line.insert(insert_at as usize, *number);
                    inserted = true;
                }
            }

            if !inserted {
                new_line.push(*number);
            }
        }

        result += new_line[new_line.len()/2];
    }

   

    result
}


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
            if !rules.contains_key(&first) {
                rules.insert(first, vec![second]);
            }else {
                rules.get_mut(&first).unwrap().push(second);
            }

        } else if trimmed_line.contains(",") {
            //numbers
            let numbers_line: Vec<u32> = trimmed_line.split(",").map(|n| n.parse::<u32>().unwrap()).collect();
            numbers.push(numbers_line);
        }
    });

    let (result_first, incorrect) = first(&rules, numbers);
    Report{exercise:5, first:result_first, second:second(&rules, incorrect)}
}


pub fn run() {
    println!("{}",solve(&INPUT));
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