#![feature(test)]
#![feature(iter_map_windows)]

extern crate test;

use core::ops::Range;
use std::collections::HashMap;

type ConversionMap = HashMap<Range<usize>, Range<usize>>;

#[derive(Debug)]
struct Almanac {
    seeds_ranges: Vec<Range<usize>>,
    maps: Vec<ConversionMap>,
}

impl Almanac {
    fn extract_map(s: &str) -> ConversionMap {
        let mut split_s = s.split('\n');
        let _ = split_s.next(); // Skip first

        split_s
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

        type SeedsRangePart = Vec<(usize, usize)>;
        let (seeds_range_start, seeds_range_len): (SeedsRangePart, SeedsRangePart) =
            split_input
                .next()
                .unwrap()
                .split(": ")
                .last()
                .unwrap()
                .split(' ')
                .map(|s| s.parse::<usize>().unwrap())
                .enumerate()
                .partition(|(idx, _)| idx % 2 == 0);

        let seeds_ranges = seeds_range_start
            .iter()
            .map(|(_idx, start)| *start)
            .zip(seeds_range_len.iter().map(|(_idx, len)| *len))
            .map(|(start, len)| start..start + len)
            .collect();

        let maps = split_input
            .map(Almanac::extract_map)
            .collect::<Vec<ConversionMap>>();

        Self { seeds_ranges, maps }
    }
}

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let almanac = Almanac::from(input);

    // Brute force, takes time but works
    for location in 0usize.. {
        println!("{:?}", location);
        let mut next_dst = location;
        for map in almanac.maps.iter().rev() {
            for (source, dest) in map.iter() {
                if dest.contains(&next_dst) {
                    next_dst = source.start + (next_dst - dest.start);
                    break;
                }
            }
        }

        for range in almanac.seeds_ranges.iter() {
            if range.contains(&next_dst) {
                return location.to_string();
            }
        }
    }

    "0".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
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
        assert_eq!(result, "46");
    }

    //#[test]
    //fn test_part2_real_data() {
    //    let input = include_str!("../../input1.txt");
    //    let result = part2(input);
    //    assert_eq!(result, "78775051");
    //}
}

//#[bench]
//fn bench_part2(b: &mut test::Bencher) {
//    b.iter(|| {
//        let input = include_str!("../../input1.txt");
//       let _ = part2(input);
//    });
//}
