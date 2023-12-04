use std::collections::{HashMap, HashSet};

use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

fn match_count(line: &str) -> usize {
    let (left, right) = line.split_once("|").expect("Could not find | character");
    let winning = parse_nums(left)
        .skip(1) // skip the game ID
        .collect::<HashSet<u64>>();
    parse_nums(right).filter(|n| winning.contains(n)).count()
}

pub fn part1(input: String) {
    input
        .lines()
        .map(|line| {
            let match_count = match_count(line);
            if match_count > 1 {
                2 << (match_count - 2)
            } else {
                match_count
            }
        })
        .sum::<usize>()
        .print();
}

pub fn part2(input: String) {
    let mut multipliers = HashMap::<usize, usize>::new();
    input
        .lines()
        .enumerate()
        .fold(0usize, |result, (index, line)| {
            let multiplier = multipliers.get(&index).unwrap_or(&0).clone() + 1;
            multipliers.remove(&index);
            let match_count = match_count(line);
            for i in (index + 1)..(index + match_count + 1) {
                multipliers.insert(i, *multipliers.get(&i).unwrap_or(&0) + multiplier);
            }
            result + match_count * multiplier + 1
        })
        .print();
}
