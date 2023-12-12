#![feature(test)]

extern crate test;

use ndarray::prelude::*;

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let y_len = input.lines().count();
    let mut expanded_universe: Array2<char> = Array::default((0, y_len));
    for line in input.lines() {
        expanded_universe
            .push_row(aview1(line.chars().collect::<Vec<char>>().as_slice()))
            .unwrap();
        if !line.contains('#') {
            expanded_universe
                .push_row(aview1(vec!['.'; line.len()].as_slice()))
                .unwrap();
        }
    }

    let expanded_universe_trans = expanded_universe.reversed_axes();

    let mut expanded_universe = vec![];
    expanded_universe_trans.axis_iter(Axis(0)).for_each(|a| {
        expanded_universe.push(a);
        if !a.to_vec().contains(&'#') {
            expanded_universe.push(a);
        }
    });

    let mut expanded_universe = Array::from_iter(expanded_universe.iter().flatten())
        .into_shape((
            expanded_universe.len(),
            expanded_universe.iter().last().unwrap().len(),
        ))
        .unwrap();
    expanded_universe = expanded_universe.reversed_axes();

    let galaxies: Vec<(usize, usize)> = expanded_universe
        .indexed_iter()
        .filter_map(|((x, y), &&a)| (a == '#').then_some((x, y)))
        .collect();

    let distances: Vec<_> = galaxies
        .iter()
        .enumerate()
        .map(|(idx, &galaxy1)| {
            galaxies
                .iter()
                .skip(idx + 1)
                .map(|&galaxy2| {
                    let mut distance = 0;
                    if galaxy1.0 <= galaxy2.0 {
                        distance += galaxy2.0 - galaxy1.0;
                    } else {
                        distance += galaxy1.0 - galaxy2.0;
                    }
                    if galaxy1.1 <= galaxy2.1 {
                        distance += galaxy2.1 - galaxy1.1;
                    } else {
                        distance += galaxy1.1 - galaxy2.1;
                    }
                    distance
                })
                .collect::<Vec<_>>()
        })
        .collect();

    println!("{distances:?}");

    distances.iter().flatten().sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = part1(
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
        );
        assert_eq!(result, "374");
    }

    #[test]
    fn test_part1_real_data() {
        let input = include_str!("../../input1.txt");
        let result = part1(input);
        assert_eq!(result, "9681886");
    }
}

#[bench]
fn bench_part1(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part1(input);
    });
}
