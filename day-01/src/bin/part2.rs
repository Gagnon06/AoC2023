const POSSIBLE_DIGITS: [(&'static str, u32); 18] = [
    ("one", 1), ("1", 1),
    ("two", 2), ("2", 2),
    ("three", 3), ("3", 3), 
    ("four", 4), ("4", 4), 
    ("five", 5), ("5", 5), 
    ("six", 6), ("6", 6), 
    ("seven", 7), ("7", 7), 
    ("eight", 8), ("8", 8), 
    ("nine", 9), ("9", 9)
];

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let total: u32 = input.lines()
        .map(|line| process_line(line))
        .sum();

    total.to_string()
}

fn process_line(line: &str) -> u32 {
    dbg!(line);

    let mut possible_first_digits: Vec<(usize, u32)> = POSSIBLE_DIGITS
        .iter()
        .filter_map(|digit| match line.find(digit.0) {
            Some(pos) => Some((pos, digit.1)),
            None => None
        })
        .collect();
    let mut possible_second_digits: Vec<(usize, u32)> = POSSIBLE_DIGITS
        .iter()
        .filter_map(|digit| match line.rfind(digit.0) {
            Some(pos) => Some((pos, digit.1)),
            None => None
        })
        .collect();

    possible_first_digits.sort_by(|a, b| a.0.cmp(&b.0));
    possible_second_digits.sort_by(|a, b| a.0.cmp(&b.0));
    possible_second_digits.reverse();

    possible_first_digits[0].1 * 10 + possible_second_digits[0].1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            "two1nine\n\
                    eightwothree\n\
                    abcone2threexyz\n\
                    xtwone3four\n\
                    4nineeightseven2\n\
                    zoneight234\n\
                    7pqrstsixteen\n");
        assert_eq!(result, "281");
    }
}
