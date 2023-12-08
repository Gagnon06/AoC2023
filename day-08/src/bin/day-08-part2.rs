#![feature(test)]

extern crate test;

use std::collections::HashMap;

use num::integer::lcm;
use regex::Regex;

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let line_regex = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();

    let mut input_iter = input.lines();
    let left_right: Vec<char> = input_iter.next().unwrap()
        .chars()
        .collect();
    let _ = input_iter.next().unwrap();

    let raw_graph: String = input_iter.collect();

    let graph:  HashMap<&str, (&str, &str)> = line_regex.captures_iter(&raw_graph)
        .map(|caps|  caps.extract())
        .map(|(_, [key, left, right])| {
            (key, (left, right))
        })
        .collect();

    let start_keys: Vec<&str> = graph.keys()
        .filter_map(|&key| {
            (key.get(2..).unwrap() == "A").then_some(key)
        })
        .collect();

    let counts: Vec<usize> = start_keys.iter()
        .map(|&start_key| {
            let mut next_key = start_key;
            for step_count in 0usize.. {
                match left_right[step_count % left_right.len()] {
                    'L' => {
                        next_key = graph.get(next_key).unwrap().0;
                    }
                    'R' => {
                        next_key = graph.get(next_key).unwrap().1;
                    }
                    _ => panic!("Oops invalid left/right!")
                }
        
                if next_key.get(2..).unwrap() == "Z" {
                    return step_count + 1
                }
            }
            return 0;
        })
        .collect();

    // From all the path we extract the Least Common Multiple
    // This only works because all paths are cyclic and they contain only one element ending with A (start)
    // and one element ending with Z (end)
    counts.into_iter()
        .reduce(|a, b| lcm(a, b)).unwrap()
        .to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        let end_regex = Regex::new(r"[A-Z]{2}Z").unwrap();
        let haystack = vec!["AAZ", "XXD", "GCZ", "WWX", "ZZZ"];
        let end_count = haystack.iter().filter_map(|ele| end_regex.is_match(ele).then_some(ele)).count();
        assert_eq!(end_count, 3);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(
            "\
            LR\n\
            \n\
            11A = (11B, XXX)\n\
            11B = (XXX, 11Z)\n\
            11Z = (11B, XXX)\n\
            22A = (22B, XXX)\n\
            22B = (22C, 22C)\n\
            22C = (22Z, 22Z)\n\
            22Z = (22B, 22B)\n\
            XXX = (XXX, XXX)");
        assert_eq!(result, "6");
    }

    #[test]
    fn test_part2_real_data() {
        let input = include_str!("../../input1.txt");
        let result = part2(input);
        assert_eq!(result, "18215611419223");
    }

}

#[bench]
fn bench_part2(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part2(input);
    });
}
