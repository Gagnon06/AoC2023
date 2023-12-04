use regex::Regex;

struct Game {
    _id: u32,
    red: Vec<u32>,
    green: Vec<u32>,
    blue: Vec<u32>
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let re: Regex = Regex::new(r"Game (?<id>[0-9]+)").unwrap();
        let Some(caps) = re.captures(line) else {
            panic!("oops");
        };

        let hands: Vec<Hand> = line.split(": ")
            .last().unwrap()
            .split("; ")
            .map(|hand_str| {
                Hand::from(hand_str)
            })
            .collect();

        Self {
            _id: caps["id"].parse().unwrap(),
            red: hands.iter().filter_map(|hand| Some(hand.red)).collect(),
            green: hands.iter().filter_map(|hand| Some(hand.green)).collect(),
            blue: hands.iter().filter_map(|hand| Some(hand.blue)).collect()
        }
    }
}

struct Hand {
    red: u32,
    green: u32,
    blue: u32
}

impl From<&str> for Hand {
    fn from(hand_str: &str) -> Self {
        let pairs: Vec<(u32, &str)> = hand_str.split(", ")
            .map(|color_str| {
                let pair: Vec<&str> = color_str.split(" ").collect();
                return (pair[0].parse::<u32>().unwrap(), pair[1])
            })
            .collect();

        let red: Vec<u32> = pairs.iter().filter_map(|pair| {
            if pair.1 == "red" {
                return Some(pair.0);
            }
            None
        }).collect();

        let green: Vec<u32> = pairs.iter().filter_map(|pair| {
            if pair.1 == "green" {
                return Some(pair.0);
            }
            None
        }).collect();

        let blue: Vec<u32> = pairs.iter().filter_map(|pair| {
            if pair.1 == "blue" {
                return Some(pair.0);
            }
            None
        }).collect();

        Self {
            red: *red.first().unwrap_or(&0),
            green: *green.first().unwrap_or(&0),
            blue: *blue.first().unwrap_or(&0)
        }
    }
}

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let total: u32 = input
        .lines()
        .map(|line| Game::from(line))
        .filter_map(|game| {
            Some(*game.red.iter().max().unwrap() * 
            *game.green.iter().max().unwrap() * 
            *game.blue.iter().max().unwrap())
        })
        .sum();

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, "2286");
    }
}
