use super::common::Report;
use std::time::Instant;


const INPUT: &'static str = include_str!("input.txt");

#[derive(PartialEq)]
enum Type {
    NotSeen,
    Seen,
    Obstacle,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn solve(input: &str)-> Report<u32, u32> {
    let mut map: Vec<Vec<Type>> = Vec::new();
    let mut y = 0;
    let mut x = 0;
    let mut direction = Direction::Up;
    input.lines().for_each(|line| {
        let mut new_line: Vec<Type> = Vec::new();
        line.chars().for_each(|c| {
            match c {
                '^' => direction = Direction::Up,
                'v' => direction = Direction::Down,
                '<' => direction = Direction::Left,
                '>' => direction = Direction::Right,
                '#' => new_line.push(Type::Obstacle),
                _ => new_line.push(Type::NotSeen),
            }
            if c == '^' || c == 'v' || c == '<' || c == '>' {
                x = new_line.len();
                y = map.len();
            }
        });
        map.push(new_line);
    });

    let mut count_seen:u32 = 0;
    while y != 0 && y != map.len()-1 && x != 0 && x != map[0].len()-1 {
        match map[y][x] {
            Type::NotSeen => {
                count_seen += 1;
                map[y][x] = Type::Seen;
            },
            Type::Seen => {},
            Type::Obstacle => {
                panic!("Obstacle at {},{}", x, y);
            },
        }
        let next = match direction {
            Direction::Up => (y-1, x),
            Direction::Down => (y+1, x),
            Direction::Left => (y, x-1),
            Direction::Right => (y, x+1),
        };
        if map[next.0][next.1] == Type::Obstacle {
            match direction {
                Direction::Up => direction = Direction::Right,
                Direction::Down => direction = Direction::Left,
                Direction::Left => direction = Direction::Up,
                Direction::Right => direction = Direction::Down,
            }
        } else {
            y = next.0;
            x = next.1;
        }
    }

    Report{exercise:6, first:count_seen+1, second:0}
}


pub fn run() {
    let now = Instant::now();
    let result = solve(&INPUT);
    let elapsed = now.elapsed();
    println!("{}, elapsed: {:.2?}",result,elapsed);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let report = solve(input);
        assert_eq!(report.first, 41);
        assert_eq!(report.second, 0);
    }



    #[test]
    fn challenge() {
        let report = solve(&INPUT);
        assert_eq!(report.first, 5208);
        assert_eq!(report.second, 0);
    }
}