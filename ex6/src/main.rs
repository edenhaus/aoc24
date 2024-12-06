use common::Report;
use std::{collections::HashSet, fmt, hash::Hash, time::Instant};


const INPUT: &'static str = include_str!("input.txt");

#[derive(PartialEq, Clone)]
enum Type {
    NotSeen,
    SeenUpDown,
    SeenLeftRight,
    Obstacle,
    SeenRightTurn,
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::NotSeen => write!(f, "."),
            Type::SeenLeftRight => write!(f, "-"),
            Type::SeenUpDown => write!(f, "|"),
            Type::Obstacle => write!(f, "#"),
            Type::SeenRightTurn => write!(f, "+")
        }
    }
}


#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn next_position(y: usize, x: usize, direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (y-1, x),
        Direction::Down => (y+1, x),
        Direction::Left => (y, x-1),
        Direction::Right => (y, x+1),
    }
}

fn check_loop(map: &Vec<Vec<Type>>,  y: usize, x: usize,direction: &Direction) -> bool {
    let mut direction = direction.clone();
    let mut x = x;
    let mut y = y;
    let mut turns: Vec<(usize, usize, Direction)> = Vec::new();
    let mut map: Vec<Vec<Type>> = map.clone();
    map[y][x] = Type::Obstacle;
    turns.push((y, x, direction));
    // previous position
    (y,x)= match direction {
        Direction::Up => (y+1, x),
        Direction::Down => (y-1, x),
        Direction::Left => (y, x+1),
        Direction::Right => (y, x-1),    
    };
    let last_y = map.len()-1;
    let last_x = map[0].len()-1;

    while y != 0 && y != last_y && x != 0 && x != last_x {
        if turns.contains(&(y, x, direction)) {
            return true;
        }
        match map[y][x] {
            Type::NotSeen => {
                if direction == Direction::Up || direction == Direction::Down {
                    map[y][x] = Type::SeenUpDown;
                } else {
                    map[y][x] = Type::SeenLeftRight;
                }
            },
            Type::Obstacle => {
                return false;
            },
            _ =>{}
        }
        let next = next_position(y, x, &direction);
        if map[next.0][next.1] == Type::Obstacle {
            turns.push((y, x, direction));
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
    false
}

pub fn solve(input: &str)-> Report<u32, u32> {
    let mut map: Vec<Vec<Type>> = Vec::new();
    let mut y = 0;
    let mut x = 0;
    let mut direction = Direction::Up;
    input.lines().for_each(|line| {
        let mut new_line: Vec<Type> = Vec::new();
        line.chars().for_each(|c| {
            if c == '^' || c == 'v' || c == '<' || c == '>' {
                x = new_line.len();
                y = map.len();
            }
            match c {
                '^' => {direction = Direction::Up; new_line.push(Type::SeenUpDown);},
                'v' => {direction = Direction::Down; new_line.push(Type::SeenUpDown);},
                '<' => {direction = Direction::Left; new_line.push(Type::SeenLeftRight);},
                '>' => {direction = Direction::Right; new_line.push(Type::SeenLeftRight);},
                '#' => new_line.push(Type::Obstacle),
                _ => new_line.push(Type::NotSeen),
            }
        });
        map.push(new_line);
    });

    let mut first:u32 = 1;
    let mut second: HashSet<(usize, usize)> = HashSet::new();
    (y,x) = next_position(y, x, &direction);
    while y != 0 && y != map.len()-1 && x != 0 && x != map[0].len()-1 {
        match map[y][x] {
            Type::NotSeen => {
                first += 1;
                if direction == Direction::Up || direction == Direction::Down {
                    map[y][x] = Type::SeenUpDown;
                } else {
                    map[y][x] = Type::SeenLeftRight;
                }
                if first > 0 && check_loop(&map, y, x, &direction){
                    second.insert((y, x));
                }
            },
            Type::SeenLeftRight => {},
            Type::SeenUpDown => {},
            Type::SeenRightTurn => {},
            Type::Obstacle => {
                panic!("Obstacle at {},{}", x, y);
            },
        }
        let next = next_position(y, x, &direction);
        if map[next.0][next.1] == Type::Obstacle {
            match direction {
                Direction::Up => direction = Direction::Right,
                Direction::Down => direction = Direction::Left,
                Direction::Left => direction = Direction::Up,
                Direction::Right => direction = Direction::Down,
            }
            map[y][x] = Type::SeenRightTurn;
        } else {
            y = next.0;
            x = next.1;
        }
    }

    //check putting obstacle at the end
    if check_loop(&map, y, x, &direction){
        second.insert((y, x));
    }

    Report{exercise:6, first:first+1, second:second.len() as u32}
}


pub fn main() {
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
        assert_eq!(report.second, 6);
    }



    #[test]
    fn challenge() {
        let report = solve(&INPUT);
        assert_eq!(report.first, 5208);
        assert_eq!(report.second, 1972);
    }
}