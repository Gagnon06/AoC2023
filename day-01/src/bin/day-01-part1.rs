#![feature(test)]

extern crate test;


const BASE: u32 = 10;

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let total: u32 = input.lines()
        .map(|line| process_line(line))
        .sum();

    total.to_string()
}

fn process_line(line: &str) -> u32 {
    dbg!(line);

    let digits = line.chars()
        .filter_map(|c| match c.to_digit(BASE) {
            Some(digit) => Some(digit),
            None => None,
        })
        .collect::<Vec<u32>>();

    digits.first().unwrap() * BASE + digits.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "1abc2\n\
                    pqr3stu8vwx\n\
                    a1b2c3d4e5f\n\
                    treb7uchet\n");
        assert_eq!(result, "142");
    }
}

#[bench]
fn bench_part1(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part1(input);
    });
}
