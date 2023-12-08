#![feature(test)]

extern crate test;

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    assert!(input.lines().count() == 2);

    let mut lines = input.lines();

    let times: Vec<u32> = lines
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .split(" ")
        .filter_map(|s| {
            if !s.is_empty() {
                return Some(s.parse::<u32>().unwrap());
            }
            None
        })
        .collect();

    let distances: Vec<u32> = lines
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .split(" ")
        .filter_map(|s| {
            if !s.is_empty() {
                return Some(s.parse::<u32>().unwrap());
            }
            None
        })
        .collect();

    assert!(times.len() == distances.len());

    let mut accumulator = 1;

    for (idx, &time) in times.iter().enumerate() {
        let winning_distances: Vec<u32> = (0..=time)
            .filter_map(|hold_time| {
                let distance = hold_time * (time - hold_time);
                if distance > distances[idx] {
                    return Some(distance);
                }
                None
            })
            .collect();

        accumulator *= winning_distances.len();
    }

    accumulator.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "\
            Time:      7  15   30\n\
            Distance:  9  40  200",
        );
        assert_eq!(result, "288");
    }

    #[test]
    fn test_part1_real_data() {
        let input = include_str!("../../input1.txt");
        let result = part1(input);
        assert_eq!(result, "2344708");
    }
}

#[bench]
fn bench_part1(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part1(input);
    });
}
