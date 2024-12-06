use std::time::Instant;

use common::Report;

const INPUT: &str = include_str!("input.txt");

fn first(input: &str) -> u32 {
    let characters: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut result: u32 = 0;

    let target = ['X', 'M', 'A', 'S'];
    let target_minus_1 = target.len() - 1;

    for i in 0..characters.len() {
        for j in 0..characters[i].len() {
            if characters[i][j] != target[0] {
                continue;
            }

            // check right
            if j + target_minus_1 < characters[i].len() {
                let mut found = true;
                for (t_idx, t_value) in target.iter().enumerate() {
                    if characters[i][j + t_idx] != *t_value {
                        found = false;
                        break;
                    }
                }
                if found {
                    result += 1;
                }
            }

            //check left
            if j >= target_minus_1 {
                let mut found = true;
                for (t_idx, t_value) in target.iter().enumerate() {
                    if characters[i][j - t_idx] != *t_value {
                        found = false;
                        break;
                    }
                }
                if found {
                    result += 1;
                }
            }

            // check down
            if i + target_minus_1 < characters.len() {
                let mut found = true;
                for (t_idx, t_value) in target.iter().enumerate() {
                    if characters[i + t_idx][j] != *t_value {
                        found = false;
                        break;
                    }
                }
                if found {
                    result += 1;
                }
            }

            // check up
            if i >= target_minus_1 {
                let mut found = true;
                for (t_idx, t_value) in target.iter().enumerate() {
                    if characters[i - t_idx][j] != *t_value {
                        found = false;
                        break;
                    }
                }
                if found {
                    result += 1;
                }
            }

            // check diagonal down right
            if i + target_minus_1 < characters.len() && j + target_minus_1 < characters[i].len() {
                let mut found = true;
                for (t_idx, t_value) in target.iter().enumerate() {
                    if characters[i + t_idx][j + t_idx] != *t_value {
                        found = false;
                        break;
                    }
                }
                if found {
                    result += 1;
                }
            }

            // check diagonal down left
            if i + target_minus_1 < characters.len() && j >= target_minus_1 {
                let mut found = true;
                for (t_idx, t_value) in target.iter().enumerate() {
                    if characters[i + t_idx][j - t_idx] != *t_value {
                        found = false;
                        break;
                    }
                }
                if found {
                    result += 1;
                }
            }

            // check diagonal up right
            if i >= target_minus_1 && j + target_minus_1 < characters[i].len() {
                let mut found = true;
                for (t_idx, t_value) in target.iter().enumerate() {
                    if characters[i - t_idx][j + t_idx] != *t_value {
                        found = false;
                        break;
                    }
                }
                if found {
                    result += 1;
                }
            }

            // check diagonal up left
            if i >= target_minus_1 && j >= target_minus_1 {
                let mut found = true;
                for (t_idx, t_value) in target.iter().enumerate() {
                    if characters[i - t_idx][j - t_idx] != *t_value {
                        found = false;
                        break;
                    }
                }
                if found {
                    result += 1;
                }
            }
        }
    }

    result
}

fn second(input: &str) -> u32 {
    let characters: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut result: u32 = 0;

    for i in 0..characters.len() {
        for j in 0..characters[i].len() {
            if characters[i][j] != 'A' {
                continue;
            }

            if i + 1 >= characters.len() || j + 1 >= characters[i].len() || j < 1 || i < 1 {
                continue;
            }

            if ((characters[i - 1][j - 1] == 'M' && characters[i + 1][j + 1] == 'S')
                || (characters[i - 1][j - 1] == 'S' && characters[i + 1][j + 1] == 'M'))
                && ((characters[i - 1][j + 1] == 'M' && characters[i + 1][j - 1] == 'S')
                    || (characters[i - 1][j + 1] == 'S' && characters[i + 1][j - 1] == 'M'))
            {
                result += 1;
            }
        }
    }

    result
}

pub fn solve(input: &str) -> Report<u32, u32> {
    Report {
        exercise: 4,
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
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let report = solve(input);
        assert_eq!(report.first, 18);
        assert_eq!(report.second, 9);
    }

    #[test]
    fn challenge() {
        let report = solve(INPUT);
        assert_eq!(report.first, 2358);
        assert_eq!(report.second, 1737);
    }
}
