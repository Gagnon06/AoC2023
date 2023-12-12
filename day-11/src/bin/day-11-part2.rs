#![feature(test)]

extern crate test;

use std::ops::Range;

use ndarray::prelude::*;

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part2(input, 1000000);
    dbg!(output);
}

fn range_between_galaxies(galaxy1: usize, galaxy2: usize) -> Range<usize> {
    if galaxy1 < galaxy2 {
        galaxy1..galaxy2
    } else {
        galaxy2..galaxy1
    }
}

fn part2(input: &str, expansion_factor: usize) -> String {
    let galaxies: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, ele)| (ele == '#').then_some((x, y)))
        })
        .collect();

    let mut universe: Array2<char> = Array::default((0, input.lines().count()));
    for line in input.lines() {
        universe
            .push_row(aview1(line.chars().collect::<Vec<char>>().as_slice()))
            .unwrap();
    }

    let x_expansions: Vec<_> = universe
        .axis_iter(Axis(1))
        .map(|row| {
            if !row.to_vec().contains(&'#') {
                expansion_factor
            } else {
                1
            }
        })
        .collect();

    let y_expansions: Vec<_> = universe
        .axis_iter(Axis(0))
        .map(|col| {
            if !col.to_vec().contains(&'#') {
                expansion_factor
            } else {
                1
            }
        })
        .collect();

    let distances: Vec<_> = galaxies
        .iter()
        .enumerate()
        .map(|(idx, &galaxy1)| {
            galaxies
                .iter()
                .skip(idx + 1)
                .map(|&galaxy2| {
                    x_expansions[range_between_galaxies(galaxy1.0, galaxy2.0)]
                        .iter()
                        .sum::<usize>()
                        + y_expansions[range_between_galaxies(galaxy1.1, galaxy2.1)]
                            .iter()
                            .sum::<usize>()
                })
                .collect::<Vec<_>>()
        })
        .collect();

    distances.iter().flatten().sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_example1() {
        let result = part2(
            "\
            ...#......\n\
            .......#..\n\
            #.........\n\
            ..........\n\
            ......#...\n\
            .#........\n\
            .........#\n\
            ..........\n\
            .......#..\n\
            #...#.....",
            10,
        );
        assert_eq!(result, "1030");
    }

    #[test]
    fn test_part2_example2() {
        let result = part2(
            "\
            ...#......\n\
            .......#..\n\
            #.........\n\
            ..........\n\
            ......#...\n\
            .#........\n\
            .........#\n\
            ..........\n\
            .......#..\n\
            #...#.....",
            100,
        );
        assert_eq!(result, "8410");
    }

    #[test]
    fn test_part2_real_data() {
        let input = include_str!("../../input1.txt");
        let result = part2(input, 1000000);
        assert_eq!(result, "791134099634");
    }
}

#[bench]
fn bench_part2(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part2(input, 1000000);
    });
}
