use std::collections::HashMap;

use regex::Regex;

use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    input
        .lines()
        .map(|line| {
            let mut numbers = line.chars().filter(|c| c.is_numeric());
            let first = numbers.next().unwrap_or_default();
            let last = numbers.last().unwrap_or(first);

            last.to_digit(10).unwrap_or_default() + first.to_digit(10).unwrap_or_default() * 10
        })
        .sum::<u32>()
        .print();
}

fn value(x: &str) -> Option<u64> {
    let map = HashMap::from([
        ("one", 1u64),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    if let Some(value) = map.get(x) {
        return Some(*value);
    } else if x != "" {
        return x.parse().ok();
    }
    None
}

pub fn part2(input: String) {
    let regex =
        Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").expect("Invalid regex");
    let reverse_regex =
        Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").expect("Invalid regex");
    input
        .lines()
        .map(|line| {
            let first = regex
                .find_iter(line)
                .next()
                .and_then(|v| value(v.as_str()))
                .unwrap_or_default();
            let last = reverse_regex
                .find_iter(line.chars().rev().collect::<String>().as_str())
                .next()
                .and_then(|v| value(v.as_str().chars().rev().collect::<String>().as_str()))
                .unwrap_or(first);
            first * 10 + last
        })
        .sum::<u64>()
        .print();
}
