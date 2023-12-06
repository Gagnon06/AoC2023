#![feature(test)]

extern crate test;


fn main() {
    let input = include_str!("../../input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    assert!(input.lines().count() == 2);

    let mut lines = input.lines();

    let time = lines.next().unwrap()
        .replace(" ", "")
        .split(":")
        .last().unwrap()
        .parse::<u64>().unwrap();

    let record_distance = lines.next().unwrap()
        .replace(" ", "")
        .split(":")
        .last().unwrap()
        .parse::<u64>().unwrap();

    let winning_distances: Vec<u64> = (0..=time)
        .filter_map(|hold_time| {
            let distance = hold_time * (time - hold_time);
            if distance > record_distance {
                return Some(distance);
            }
            None
        })
        .collect();

    winning_distances.len().to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            "\
            Time:      7  15   30\n\
            Distance:  9  40  200");
        assert_eq!(result, "71503");
    }

    #[test]
    fn test_part2_real_data() {
        let input = include_str!("../../input1.txt");
        let result = part2(input);
        assert_eq!(result, "30125202");
    }

}

#[bench]
fn bench_part2(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part2(input);
    });
}
