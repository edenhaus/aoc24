use common::Report;
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
    time::Instant,
};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct Delta {
    x: i32,
    y: i32,
}

impl Sub for &Point {
    type Output = Delta;

    fn sub(self, other: &Point) -> Delta {
        Delta {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add<&Delta> for &Point {
    type Output = Point;

    fn add(self, other: &Delta) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Field {
    x: usize,
    y: usize,
    antinode: HashSet<Point>,
}

impl Field {
    fn new(x: usize, y: usize) -> Field {
        Field {
            x,
            y,
            antinode: HashSet::new(),
        }
    }

    fn add_antinode(&mut self, point: Point) -> bool {
        if point.x >= self.x as _ || point.y >= self.y as _ || point.x < 0 || point.y < 0 {
            return false;
        }

        self.antinode.insert(point);
        true
    }
}

pub fn solve(input: &str) -> Report<usize, usize> {
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|&(_, c)| c != '.')
            .for_each(|(x, c)| {
                antennas.entry(c).or_default().push(Point {
                    x: x as _,
                    y: y as _,
                });
            });
    });

    let mut field = Field::new(input.lines().count(), input.lines().next().unwrap().len());
    let mut field_part2 = Field::new(field.x, field.y);
    antennas.iter().for_each(|(_, points)| {
        points.iter().for_each(|point| {
            points.iter().filter(|&p| p != point).for_each(|p| {
                let delta = &(point - p);
                let mut antinode = point.clone();

                field.add_antinode(point + delta);
                while field_part2.add_antinode(antinode) {
                    antinode = &antinode + delta;
                }
            });
        });
    });

    Report {
        exercise: 8,
        first: field.antinode.len(),
        second: field_part2.antinode.len(),
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
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let report = solve(input);
        assert_eq!(report.first, 14);
        assert_eq!(report.second, 34);
    }

    #[test]
    fn challenge() {
        let report = solve(INPUT);
        assert_eq!(report.first, 261);
        assert_eq!(report.second, 898);
    }
}
