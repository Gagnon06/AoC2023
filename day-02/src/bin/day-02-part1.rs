#![feature(test)]

extern crate test;

use regex::Regex;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

struct Game {
    id: u32,
    red: Vec<u32>,
    green: Vec<u32>,
    blue: Vec<u32>,
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let re: Regex = Regex::new(r"Game (?<id>[0-9]+)").unwrap();
        let Some(caps) = re.captures(line) else {
            panic!("oops");
        };

        let hands: Vec<Hand> = line
            .split(": ")
            .last()
            .unwrap()
            .split("; ")
            .map(Hand::from)
            .collect();

        Self {
            id: caps["id"].parse().unwrap(),
            red: hands.iter().map(|hand| hand.red).collect(),
            green: hands.iter().map(|hand| hand.green).collect(),
            blue: hands.iter().map(|hand| hand.blue).collect(),
        }
    }
}

struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

impl From<&str> for Hand {
    fn from(hand_str: &str) -> Self {
        let pairs: Vec<(u32, &str)> = hand_str
            .split(", ")
            .map(|color_str| {
                let pair: Vec<&str> = color_str.split(' ').collect();
                (pair[0].parse::<u32>().unwrap(), pair[1])
            })
            .collect();

        let red: Vec<u32> = pairs
            .iter()
            .filter_map(|pair| {
                if pair.1 == "red" {
                    return Some(pair.0);
                }
                None
            })
            .collect();

        let green: Vec<u32> = pairs
            .iter()
            .filter_map(|pair| {
                if pair.1 == "green" {
                    return Some(pair.0);
                }
                None
            })
            .collect();

        let blue: Vec<u32> = pairs
            .iter()
            .filter_map(|pair| {
                if pair.1 == "blue" {
                    return Some(pair.0);
                }
                None
            })
            .collect();

        Self {
            red: *red.first().unwrap_or(&0),
            green: *green.first().unwrap_or(&0),
            blue: *blue.first().unwrap_or(&0),
        }
    }
}

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let total: u32 = input
        .lines()
        .filter_map(|line| {
            let game = Game::from(line);
            (*game.red.iter().max().unwrap() <= MAX_RED
                && *game.green.iter().max().unwrap() <= MAX_GREEN
                && *game.blue.iter().max().unwrap() <= MAX_BLUE)
                .then_some(game.id)
        })
        .sum();

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "8");
    }
}

#[bench]
fn bench_part1(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part1(input);
    });
}
