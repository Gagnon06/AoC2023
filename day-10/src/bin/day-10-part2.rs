#![feature(test)]
#![feature(iter_map_windows)]

use std::collections::HashMap;

extern crate test;

use geo::Contains;
use geo::{point, Coord, LineString, Polygon};

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
    let output = part2(input);
    dbg!(output);
}

fn find_path(graph: &HashMap<PipeCoords, Pipe>, start_coords: PipeCoords) -> Vec<PipeCoords> {
    for direction in [
        Direction::North,
        Direction::East,
        Direction::West,
        Direction::South,
    ] {
        let mut path: Vec<PipeCoords> = vec![start_coords];
        if let Some(mut coords) = Pipe::coords(start_coords, direction) {
            let mut previous_coords = start_coords;
            loop {
                if let Some(pipe) = graph.get(&coords) {
                    if let Some(next_coords) = pipe.find_next_coords(previous_coords) {
                        if next_coords == start_coords {
                            return path;
                        }
                        path.push(coords);
                        previous_coords = coords;
                        coords = next_coords;
                        continue;
                    }
                }
                break;
            }
        }
    }
    vec![]
}

fn part2(input: &str) -> String {
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

    let path = find_path(&graph, start_coords);

    let fpath: Vec<Coord> = path
        .iter()
        .map(|(x, y)| (*x as f64, *y as f64).into())
        .collect();

    let polygon = Polygon::new(LineString(fpath), vec![]);

    let mut point_count = 0;
    for (y, line) in input.lines().enumerate() {
        for x in 0..line.chars().count() {
            if polygon.contains(&point!(x: x as f64, y: y as f64)) {
                point_count += 1;
            }
        }
    }

    point_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_example1() {
        let result = part2(
            "\
            .F----7F7F7F7F-7....\n\
            .|F--7||||||||FJ....\n\
            .||.FJ||||||||L7....\n\
            FJL7L7LJLJ||LJ.L-7..\n\
            L--J.L7...LJS7F-7L7.\n\
            ....F-J..F7FJ|L7L7L7\n\
            ....L7.F7||L7|.L7L7|\n\
            .....|FJLJ|FJ|F7|.LJ\n\
            ....FJL-7.||.||||...\n\
            ....L---J.LJ.LJLJ...",
        );
        assert_eq!(result, "8");
    }

    #[test]
    fn test_part2_example2() {
        let result = part2(
            "\
            FF7FSF7F7F7F7F7F---7\n\
            L|LJ||||||||||||F--J\n\
            FL-7LJLJ||||||LJL-77\n\
            F--JF--7||LJLJ7F7FJ-\n\
            L---JF-JLJ.||-FJLJJ7\n\
            |F|F-JF---7F7-L7L|7|\n\
            |FFJF7L7F-JF7|JL---7\n\
            7-L-JL7||F7|L7F-7F7|\n\
            L.L7LFJ|||||FJL7||LJ\n\
            L7JLJL-JLJLJL--JLJ.L",
        );
        assert_eq!(result, "10");
    }

    #[test]
    fn test_part2_real_data() {
        let input = include_str!("../../input1.txt");
        let result = part2(input);
        assert_eq!(result, "281");
    }
}

#[bench]
fn bench_part2(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part2(input);
    });
}
