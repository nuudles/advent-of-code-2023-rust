use itertools::Itertools;

use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

fn next_value(numbers: &Vec<i64>) -> i64 {
    let differences = numbers
        .iter()
        .tuple_windows()
        .map(|(a, b)| *b - *a)
        .collect_vec();
    let last_number = numbers.iter().last().copied().unwrap_or_default();
    if differences.iter().all(|v| v == &0) {
        last_number
    } else {
        last_number + next_value(&differences)
    }
}

fn previous_value(numbers: &Vec<i64>) -> i64 {
    let differences = numbers
        .iter()
        .tuple_windows()
        .map(|(a, b)| *b - *a)
        .collect_vec();
    let first_number = numbers.iter().next().copied().unwrap_or_default();
    if differences.iter().all(|v| v == &0) {
        first_number
    } else {
        first_number - previous_value(&differences)
    }
}

pub fn part1(input: String) {
    input
        .lines()
        .map(|line| {
            let numbers = parse_nums(line).collect_vec();
            next_value(&numbers)
        })
        .sum::<i64>()
        .print();
}

pub fn part2(input: String) {
    input
        .lines()
        .map(|line| {
            let numbers = parse_nums(line).collect_vec();
            previous_value(&numbers)
        })
        .sum::<i64>()
        .print();
}
