#![feature(test)]
#![feature(iter_map_windows)]

extern crate test;

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let history: Vec<i64> = line
                .split(' ')
                .map(|val| val.parse::<i64>().unwrap())
                .collect();

            let mut sequences = vec![history];
            loop {
                let mut sequence: Vec<i64> = sequences
                    .first()
                    .unwrap()
                    .iter()
                    .map_windows(|[&a, &b]| b - a)
                    .collect();

                if sequence.iter().all(|&a| a == 0) {
                    sequence.push(0);
                    sequences.insert(0, sequence);
                    break;
                }
                sequences.insert(0, sequence);
            }

            let mut value = 0;
            for seq in sequences.iter().skip(1) {
                value += seq.last().unwrap();
            }

            value
        })
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "\
            0 3 6 9 12 15\n\
            1 3 6 10 15 21\n\
            10 13 16 21 30 45",
        );
        assert_eq!(result, "114");
    }

    #[test]
    fn test_part1_real_data() {
        let input = include_str!("../../input1.txt");
        let result = part1(input);
        assert_eq!(result, "2174807968");
    }
}

#[bench]
fn bench_part1(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part1(input);
    });
}
