
const INPUT: &'static str = include_str!("input.txt");

fn difference(x: u32,y:u32) -> u32 {
    if x > y {
        x-y
    } else {
        y-x
    }
}

fn check_line(numbers: &Vec<u32>) -> bool {
    let mut mode = 0;
    // 0: no checked; 1: increasing; 2: decreasing
    for idx in 1..(numbers.len()) {
        let current = numbers[idx];
        let previous = numbers[idx - 1];
        if current > previous {
            if mode == 0 {
                mode = 1;
            } else if mode == 2 {
                // line not safe
                return false;
            }
        } else if current < previous {
            if mode == 0 {
                mode = 2;
            } else if mode == 1 {
                // line not safe
                return false;
            }
        }

        let difference = difference(current, previous);
        if difference < 1 {
            // line not safe
            return false;
        } else if difference > 3 {
            // line not safe
            return false;
        }
    }
    true
}

pub fn solve(input: &str)-> (u32, u32) {
    let lines: Vec<&str> = input.lines().collect();
    let mut safe_lines_first:u32 = 0;
    let mut safe_lines_second:u32 = 0;

    for line in lines.iter() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let numbers = words.iter().map(|&x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        
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

    (safe_lines_first, safe_lines_first+safe_lines_second)
}


pub fn run() {
    let result = solve(&INPUT);
    println!("Result ex2: first: {:?} second: {:?}", result.0, result.1);
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
        let (first, second) = solve(input);
        assert_eq!(first, 2);
        assert_eq!(second, 4);
    }

    #[test]
    fn challenge() {
        let (first, second) = solve(&INPUT);
        assert_eq!(first, 549);
        assert_eq!(second, 589);
    }
}