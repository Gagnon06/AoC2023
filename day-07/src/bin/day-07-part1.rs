#![feature(test)]

extern crate test;

use std::collections::HashMap;

fn get_card_value(card: &char) -> u32 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("oups, invalid card"),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    bid: usize,
    cards: Vec<char>,
    hand_type: HandType,
}

impl Hand {
    fn new(line: &str) -> Self {
        let mut iter = line.split(" ");
        let cards: Vec<char> = iter.next().unwrap().chars().collect();
        let bid = iter.next().unwrap().parse().unwrap();

        let mut cards_map = HashMap::new();

        for card in cards.iter() {
            cards_map
                .entry(*card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        Self {
            bid,
            cards,
            hand_type: Self::get_hand_type(cards_map),
        }
    }

    fn get_hand_type(cards_map: HashMap<char, usize>) -> HandType {
        let card_counts: Vec<usize> = cards_map.iter().map(|(_card, &count)| count).collect();

        if card_counts.len() == 1 {
            return HandType::FiveOfAKind;
        } else if card_counts.len() == 2 {
            if card_counts.contains(&4) {
                return HandType::FourOfAKind;
            } else if card_counts.contains(&3) {
                return HandType::FullHouse;
            }
        } else if cards_map.len() == 3 {
            if card_counts.contains(&3) {
                return HandType::ThreeOfAKind;
            } else if card_counts.contains(&2) {
                return HandType::TwoPair;
            }
        } else if cards_map.len() == 4 {
            return HandType::OnePair;
        }

        HandType::HighCard
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.hand_type == other.hand_type;
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type == other.hand_type {
            for (card1, card2) in self.cards.iter().zip(other.cards.iter()) {
                if card1 == card2 {
                    continue;
                }
                return get_card_value(card1).partial_cmp(&get_card_value(card2));
            }
        }
        return self.hand_type.partial_cmp(&other.hand_type);
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut hands: Vec<Hand> = input.lines().map(|line| Hand::new(line)).collect();

    hands.sort();

    let winnings: Vec<usize> = hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx + 1usize))
        .collect();

    winnings.iter().sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "\
            32T3K 765\n\
            T55J5 684\n\
            KK677 28\n\
            KTJJT 220\n\
            QQQJA 483",
        );
        assert_eq!(result, "6440");
    }

    #[test]
    fn test_part1_real_data() {
        let input = include_str!("../../input1.txt");
        let result = part1(input);
        assert_eq!(result, "248569531");
    }
}

#[bench]
fn bench_part1(b: &mut test::Bencher) {
    b.iter(|| {
        let input = include_str!("../../input1.txt");
        let _ = part1(input);
    });
}
