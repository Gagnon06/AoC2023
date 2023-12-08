#![feature(test)]

extern crate test;

use regex::Regex;

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let re = Regex::new(r"[0-9]+").unwrap();

    let mut total: u32 = 0;
    let mut line_idx = 0usize;
    for line in input.lines() {
        let mut col_idx = 0usize;
        while col_idx < line.chars().count() {
            dbg!(col_idx);
            let col = line.chars().nth(col_idx).unwrap();
            if col != '.' && !is_symbol(col) {
                let mat = re.find(&line[col_idx..line.len()]).unwrap();
                dbg!(mat);
                if let Ok(number) = mat.as_str().parse::<u32>() {
                    dbg!(number);
                    if is_part_number(input, number, line_idx, col_idx) {
                        println!("Is part number!");
                        total += number;
                    }
                }
                col_idx += mat.end() - mat.start();
                continue;
            }
            col_idx += 1;
        }
        line_idx += 1;
    }

    total.to_string()
}

fn is_symbol(c: char) -> bool {
    match c {
        '0' => false,
        '1' => false,
        '2' => false,
        '3' => false,
        '4' => false,
        '5' => false,
        '6' => false,
        '7' => false,
        '8' => false,
        '9' => false,
        '.' => false,
        _ => true,
    }
}

fn check_line(line: Option<&str>, col_idx: usize) -> bool {
    if let Some(line) = line {
        if col_idx > 0 {
            if let Some(ch) = line.chars().nth(col_idx - 1) {
                if is_symbol(ch) {
                    return true;
                }
            }
        }
        if let Some(ch) = line.chars().nth(col_idx) {
            if is_symbol(ch) {
                return true;
            }
        }
        if col_idx < line.len() - 1 {
            if let Some(ch) = line.chars().nth(col_idx + 1) {
                if is_symbol(ch) {
                    return true;
                }
            }
        }
    }
    return false;
}

fn check_neighbors(input: &str, line_idx: usize, col_idx: usize) -> bool {
    if line_idx > 0 && check_line(input.lines().nth(line_idx - 1), col_idx) {
        return true;
    }
    if line_idx < input.lines().count() - 1 && check_line(input.lines().nth(line_idx + 1), col_idx)
    {
        return true;
    }
    return check_line(input.lines().nth(line_idx), col_idx);
}

fn is_part_number(input: &str, number: u32, line_idx: usize, col_idx: usize) -> bool {
    if check_neighbors(input, line_idx, col_idx) {
        return true;
    }

    if number >= 10 {
        if check_neighbors(input, line_idx, col_idx + 1) {
            return true;
        }
    }
    if number >= 100 {
        if check_neighbors(input, line_idx, col_idx + 2) {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
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
        assert_eq!(result, "4361");
    }

    #[test]
    fn test_part1_real_data() {
        let input = include_str!("../../input1.txt");
        let result = part1(input);
        assert_eq!(result, "539713");
    }
}

#[bench]
fn bench_part1(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part1(input);
    });
}
