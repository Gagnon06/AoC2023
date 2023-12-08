#![feature(test)]

extern crate test;

use std::collections::HashMap;

use regex::Regex;

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();

    let mut input_iter = input.lines();
    let left_right: Vec<char> = input_iter.next().unwrap().chars().collect();
    let _ = input_iter.next().unwrap();

    let raw_graph: String = input_iter.collect();

    let graph: HashMap<&str, (&str, &str)> = re
        .captures_iter(&raw_graph)
        .map(|caps| caps.extract())
        .map(|(_, [key, left, right])| (key, (left, right)))
        .collect();

    let mut current_element_key = "AAA";

    for idx in 0usize.. {
        match left_right[idx % left_right.len()] {
            'L' => {
                current_element_key = graph.get(current_element_key).unwrap().0;
            }
            'R' => {
                current_element_key = graph.get(current_element_key).unwrap().1;
            }
            _ => panic!("Oops invalid left/right!"),
        }

        if current_element_key == "ZZZ" {
            return (idx + 1).to_string();
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
            RL\n\
            \n\
            AAA = (BBB, CCC)\n\
            BBB = (DDD, EEE)\n\
            CCC = (ZZZ, GGG)\n\
            DDD = (DDD, DDD)\n\
            EEE = (EEE, EEE)\n\
            GGG = (GGG, GGG)\n\
            ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, "2");
    }

    #[test]
    fn test_part1_example2() {
        let result = part1(
            "\
            LLR\n\
            \n\
            AAA = (BBB, BBB)\n\
            BBB = (AAA, ZZZ)\n\
            ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, "6");
    }

    #[test]
    fn test_part1_real_data() {
        let input = include_str!("../../input1.txt");
        let result = part1(input);
        assert_eq!(result, "12361");
    }
}

#[bench]
fn bench_part1(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part1(input);
    });
}
