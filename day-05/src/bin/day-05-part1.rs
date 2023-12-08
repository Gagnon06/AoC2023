#![feature(test)]

extern crate test;

use core::ops::Range;
use std::collections::HashMap;

type ConversionMap = HashMap<Range<usize>, Range<usize>>;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<ConversionMap>,
}

impl Almanac {
    fn extract_map(s: &str) -> ConversionMap {
        s.split('\n')
            .skip(1)
            .filter_map(|line| {
                let numbers = line
                    .split(' ')
                    .filter_map(|number| {
                        if number.is_empty() {
                            return None;
                        }
                        Some(number.parse::<usize>().unwrap())
                    })
                    .collect::<Vec<usize>>();
                if numbers.len() != 3 {
                    println!("Oops, numbers.len() != 3, : {:?}", numbers);
                    return None;
                }

                let dest_range = numbers[0]..numbers[0] + numbers[2];
                let source_range = numbers[1]..numbers[1] + numbers[2];
                Some((source_range, dest_range))
            })
            .collect::<ConversionMap>()
    }
}

impl From<&str> for Almanac {
    fn from(input: &str) -> Self {
        let mut split_input = input.split("\n\n");

        let seeds = split_input
            .next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let maps = split_input
            .map(Almanac::extract_map)
            .collect::<Vec<ConversionMap>>();

        Self { seeds, maps }
    }
}

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let almanac = Almanac::from(input);

    let locations = almanac.seeds.iter().map(|seed| {
        let mut next_source = *seed;
        for map in almanac.maps.iter() {
            for (source, dest) in map.iter() {
                if source.contains(&next_source) {
                    next_source = dest.start + (next_source - source.start);
                    break;
                }
            }
        }
        next_source
    });

    locations.min().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "\
            seeds: 79 14 55 13\n\
            \n\
            seed-to-soil map:\n\
            50 98 2\n\
            52 50 48\n\
            \n\
            soil-to-fertilizer map:\n\
            0 15 37\n\
            37 52 2\n\
            39 0 15\n\
            \n\
            fertilizer-to-water map:\n\
            49 53 8\n\
            0 11 42\n\
            42 0 7\n\
            57 7 4\n\
            \n\
            water-to-light map:\n\
            88 18 7\n\
            18 25 70\n\
            \n\
            light-to-temperature map:\n\
            45 77 23\n\
            81 45 19\n\
            68 64 13\n\
            \n\
            temperature-to-humidity map:\n\
            0 69 1\n\
            1 0 69\n\
            \n\
            humidity-to-location map:\n\
            60 56 37\n\
            56 93 4",
        );
        assert_eq!(result, "35");
    }

    #[test]
    fn test_part1_real_data() {
        let input = include_str!("../../input1.txt");
        let result = part1(input);
        assert_eq!(result, "227653707");
    }
}

#[bench]
fn bench_part1(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part1(input);
    });
}
