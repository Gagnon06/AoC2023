#![feature(test)]
#![feature(iter_map_windows)]

extern crate test;

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
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
                value = seq.first().unwrap() - value;
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
    fn test_part2() {
        let result = part2(
            "\
            0 3 6 9 12 15\n\
            1 3 6 10 15 21\n\
            10 13 16 21 30 45",
        );
        assert_eq!(result, "2");
    }

    #[test]
    fn test_part2_real_data() {
        let input = include_str!("../../input1.txt");
        let result = part2(input);
        assert_eq!(result, "1208");
    }
}

#[bench]
fn bench_part2(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part2(input);
    });
}
