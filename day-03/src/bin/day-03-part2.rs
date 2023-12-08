#![feature(test)]

extern crate test;

struct ParsedLine {
    numbers: Vec<(u32, Vec<NumberRange>)>,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone)]
struct NumberRange {
    start: usize,
    end: usize,
}

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let parsed_input = parse_input(input);

    let mut line_idx = 0usize;
    let gears_ratio_totals = input
        .lines()
        .map(|line| {
            let ratios = line
                .match_indices('*')
                .filter_map(|(idx, _)| extract_gear(&parsed_input, line_idx, idx))
                .collect::<Vec<u32>>();
            line_idx += 1;
            ratios
        })
        .reduce(|mut a, mut b| {
            a.append(&mut b);
            a
        })
        .unwrap();

    gears_ratio_totals.into_iter().sum::<u32>().to_string()
}

fn parse_input(input: &str) -> Vec<ParsedLine> {
    let re = regex::Regex::new(r"\b\d+\b").unwrap();
    input
        .lines()
        .map(|line| {
            let mut numbers = line
                .split(|c: char| !c.is_ascii_digit())
                .filter_map(|v| {
                    if !v.is_empty() {
                        let number = v.parse::<u32>().unwrap();
                        let mut ranges = vec![];
                        for mat in re.find_iter(line) {
                            if mat.as_str() == number.to_string().as_str() {
                                ranges.push(get_range(mat.start(), v.len()));
                            }
                        }
                        Some((number, ranges))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(u32, Vec<NumberRange>)>>();
            numbers.sort();
            numbers.dedup();
            ParsedLine { numbers }
        })
        .collect()
}

fn get_range(idx: usize, len: usize) -> NumberRange {
    let mut start = idx;
    start = start.saturating_sub(1);

    NumberRange {
        start,
        end: idx + len,
    }
}

fn extract_gear(input: &Vec<ParsedLine>, line_idx: usize, col_idx: usize) -> Option<u32> {
    let mut ratios = vec![];
    for line_idx in line_idx - 1..line_idx + 2 {
        if line_idx < input.len() {
            ratios.append(
                &mut input[line_idx]
                    .numbers
                    .clone()
                    .into_iter()
                    .filter_map(|(number, ranges)| {
                        for range in ranges {
                            if col_idx >= range.start && col_idx <= range.end {
                                return Some(number);
                            }
                        }
                        None
                    })
                    .collect::<Vec<u32>>(),
            );
        }
    }

    if ratios.len() < 2 {
        return None;
    }

    if ratios.len() > 2 {
        dbg!(ratios.clone());
        panic!("OUPS!")
    }
    //Some(0)
    Some(ratios.into_iter().reduce(|a, b| a * b).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            "467..114..\n\
            ...*......\n\
            ..35..633.\n\
            ......#...\n\
            617*......\n\
            .....+.58.\n\
            ..592.....\n\
            ......755.\n\
            ...$.*....\n\
            .664.598..",
        );
        assert_eq!(result, "467835");
    }

    #[test]
    fn test_part2_real_data() {
        let input = include_str!("../../input1.txt");
        let result = part2(input);
        assert_eq!(result, "84159075");
    }
}

#[bench]
fn bench_part2(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part2(input);
    });
}
