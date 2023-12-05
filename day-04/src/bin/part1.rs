

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>
}

impl From<&str> for Card {
    fn from(line: &str) -> Self {
        let mut split_line = line.split(": ");
        let id = split_line.next().unwrap()
            .split(" ")
            .last().unwrap()
            .parse::<u32>().unwrap();

        let mut numbers = split_line.next().unwrap().split(" | ");
        let winning_numbers = numbers.next().unwrap()
            .split(" ")
            .filter_map(|n| {
                if !n.trim().is_empty() {
                    return Some(n.trim());
                }
                None
            })
            .map(|n| n.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let my_numbers = numbers.next().unwrap()
            .split(" ")
            .filter_map(|n| {
                if !n.trim().is_empty() {
                    return Some(n.trim());
                }
                None
            })
            .map(|n| n.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        

        Card {
            id,
            winning_numbers,
            my_numbers
        }
    }
}

impl Card {
    fn count_points(&self) -> u32 {
        let mut points = 0;
        for &num in self.my_numbers.iter() {
            if let Some(_) = self.winning_numbers.iter().find(|&&x| x == num) {
                if points == 0 {
                    points = 1;
                }
                else {
                    points *= 2;
                }
            }
        }
        points
    }
}

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let points: u32 = input.lines()
        .map(|line| {
            Card::from(line).count_points()
        })
        .sum();
    points.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "\
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(result, "13");
    }
}
