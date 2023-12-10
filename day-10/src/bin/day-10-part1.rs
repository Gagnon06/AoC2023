#![feature(test)]
#![feature(iter_map_windows)]

use std::collections::HashMap;

extern crate test;

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    North,
    East,
    West,
    South,
}

type PipeCoords = (usize, usize);

#[derive(Debug)]
struct Pipe {
    neighbors: (PipeCoords, PipeCoords),
}

impl Pipe {
    fn new(coords: PipeCoords, raw_pipe: char) -> Option<Self> {
        if let Some(directions) = Self::directions(raw_pipe) {
            let neighbor0 = Self::coords(coords, directions.0);
            let neighbor1 = Self::coords(coords, directions.1);

            if neighbor0.is_none() || neighbor1.is_none() {
                return None;
            }

            return Some(Self {
                neighbors: (neighbor0.unwrap(), neighbor1.unwrap()),
            });
        }
        None
    }

    fn find_next_coords(&self, from: PipeCoords) -> Option<PipeCoords> {
        if self.neighbors.0 == from {
            return Some(self.neighbors.1);
        } else if self.neighbors.1 == from {
            return Some(self.neighbors.0);
        }
        None
    }

    fn directions(raw_pipe: char) -> Option<(Direction, Direction)> {
        match raw_pipe {
            '|' => Some((Direction::North, Direction::South)),
            '-' => Some((Direction::East, Direction::West)),
            'L' => Some((Direction::North, Direction::East)),
            'J' => Some((Direction::North, Direction::West)),
            '7' => Some((Direction::South, Direction::West)),
            'F' => Some((Direction::South, Direction::East)),
            _ => None,
        }
    }

    fn coords(from: PipeCoords, direction: Direction) -> Option<PipeCoords> {
        match direction {
            Direction::North => {
                if from.1 > 0 {
                    return Some((from.0, from.1 - 1));
                }
                None
            }
            Direction::East => Some((from.0 + 1, from.1)),
            Direction::West => {
                if from.0 > 0 {
                    return Some((from.0 - 1, from.1));
                }
                None
            }
            Direction::South => Some((from.0, from.1 + 1)),
        }
    }
}

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut graph: HashMap<PipeCoords, Pipe> = HashMap::new();
    let mut start_coords = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, raw_pipe) in line.chars().enumerate() {
            if let Some(pipe) = Pipe::new((x, y), raw_pipe) {
                graph.insert((x, y), pipe);
            } else if raw_pipe == 'S' {
                start_coords = (x, y);
            }
        }
    }

    for direction in [
        Direction::North,
        Direction::East,
        Direction::West,
        Direction::South,
    ] {
        if let Some(mut coords) = Pipe::coords(start_coords, direction) {
            let mut steps = 1;
            let mut previous_coords = start_coords;
            loop {
                if let Some(pipe) = graph.get(&coords) {
                    if let Some(next_coords) = pipe.find_next_coords(previous_coords) {
                        if next_coords == start_coords {
                            return ((steps / 2) + 1).to_string();
                        }
                        previous_coords = coords;
                        coords = next_coords;
                        steps += 1;
                        continue;
                    }
                }
                break;
            }
        }
    }

    "0".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = part1(
            "\
            -L|F7\n\
            7S-7|\n\
            L|7||\n\
            -L-J|\n\
            L|-JF",
        );
        assert_eq!(result, "4");
    }

    #[test]
    fn test_part1_example2() {
        let result = part1(
            "\
            7-F7-\n\
            .FJ|7\n\
            SJLL7\n\
            |F--J\n\
            LJ.LJ",
        );
        assert_eq!(result, "8");
    }

    #[test]
    fn test_part1_real_data() {
        let input = include_str!("../../input1.txt");
        let result = part1(input);
        assert_eq!(result, "7107");
    }
}

#[bench]
fn bench_part1(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part1(input);
    });
}
