use std::cmp::Ordering;

use counter::Counter;
use itertools::Itertools;

use crate::selfprint::SelfPrint;

#[derive(Debug)]
enum Hand {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    fn from(string: &str, with_jokers: bool) -> Self {
        let counter = string.chars().collect::<Counter<_>>();
        let joker_count = if with_jokers {
            counter
                .iter()
                .find(|c| c.0 == &'J')
                .map(|c| *c.1)
                .unwrap_or_default()
        } else {
            0
        };
        if counter.len() == 1 {
            return Hand::FiveOfAKind;
        } else if counter.len() == 2 {
            if counter.values().any(|v| v == &4) {
                if joker_count > 0 {
                    return Hand::FiveOfAKind;
                } else {
                    return Hand::FourOfAKind;
                }
            } else {
                if joker_count > 0 {
                    return Hand::FiveOfAKind;
                } else {
                    return Hand::FullHouse;
                }
            }
        } else if counter.len() == 3 {
            if counter.values().any(|v| v == &3) {
                if joker_count > 0 {
                    return Hand::FourOfAKind;
                } else {
                    return Hand::ThreeOfAKind;
                }
            } else {
                if joker_count == 2 {
                    return Hand::FourOfAKind;
                } else if joker_count == 1 {
                    return Hand::FullHouse;
                } else {
                    return Hand::TwoPair;
                }
            }
        } else if counter.len() == 4 {
            if joker_count > 0 {
                return Hand::ThreeOfAKind;
            } else {
                return Hand::OnePair;
            }
        } else {
            if joker_count > 0 {
                return Hand::OnePair;
            } else {
                return Hand::HighCard;
            }
        }
    }

    fn ranking(&self) -> usize {
        match self {
            Hand::FiveOfAKind => 0,
            Hand::FourOfAKind => 1,
            Hand::FullHouse => 2,
            Hand::ThreeOfAKind => 3,
            Hand::TwoPair => 4,
            Hand::OnePair => 5,
            Hand::HighCard => 6,
        }
    }
}

fn compare_hands(a_string: &str, b_string: &str, with_jokers: bool) -> Ordering {
    let a = Hand::from(a_string, with_jokers);
    let b = Hand::from(b_string, with_jokers);
    // println!("{:?} {:?}, {:?} {:?}", a_string, a, b_string, b);
    if a.ranking() < b.ranking() {
        Ordering::Greater
    } else if a.ranking() > b.ranking() {
        Ordering::Less
    } else {
        let ranks = if with_jokers {
            vec![
                'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
            ]
        } else {
            vec![
                'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
            ]
        };
        for (a, b) in a_string.chars().zip(b_string.chars()) {
            let a_rank = ranks.iter().position(|c| c == &a).unwrap_or_default();
            let b_rank = ranks.iter().position(|c| c == &b).unwrap_or_default();
            if a_rank < b_rank {
                return Ordering::Greater;
            } else if a_rank > b_rank {
                return Ordering::Less;
            }
        }
        return Ordering::Equal;
    }
}

pub fn part1(input: String) {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(" ").expect("No space found");
            (cards, bid.parse::<usize>().unwrap_or_default())
        })
        .sorted_by(|a, b| compare_hands(a.0, b.0, false))
        .enumerate()
        .map(|(index, t)| t.1 * (index + 1))
        .sum::<usize>()
        .print();
}

pub fn part2(input: String) {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(" ").expect("No space found");
            (cards, bid.parse::<usize>().unwrap_or_default())
        })
        .sorted_by(|a, b| compare_hands(a.0, b.0, true))
        .enumerate()
        .map(|(index, t)| t.1 * (index + 1))
        .sum::<usize>()
        .print();
}
