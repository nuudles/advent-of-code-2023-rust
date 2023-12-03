use std::{collections::HashSet, ops::Range};

use itertools::Itertools;

use crate::{point::Point, selfprint::SelfPrint};

fn parse(
    input: &str,
) -> (
    HashSet<Point<usize>>,
    Vec<(Range<usize>, Range<usize>, u32)>,
) {
    let mut symbols = HashSet::new();
    let mut numbers = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut start_index = usize::MAX;
        let mut number = 0;
        for (x, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if start_index == usize::MAX {
                    start_index = x;
                }
                number = number * 10 + c.to_digit(10).unwrap_or_default();
            } else {
                if c != '.' {
                    symbols.insert(Point { x: x, y: y });
                }
                if number > 0 {
                    numbers.push((
                        start_index.saturating_sub(1)..x + 1,
                        y.saturating_sub(1)..y + 2,
                        number,
                    ));
                    number = 0;
                    start_index = usize::MAX;
                }
            }
        }
        if number > 0 {
            numbers.push((
                start_index.saturating_sub(1)..line.len() + 1,
                y.saturating_sub(1)..y + 2,
                number,
            ));
        }
    }
    (symbols, numbers)
}

pub fn part1(input: String) {
    let (symbols, numbers) = parse(&input);
    numbers
        .iter()
        .map(|(x_range, y_range, number)| {
            if symbols
                .iter()
                .any(|point| x_range.contains(&point.x) && y_range.contains(&point.y))
            {
                *number
            } else {
                0
            }
        })
        .sum::<u32>()
        .print();
}

pub fn part2(input: String) {
    let (symbols, numbers) = parse(&input);
    symbols
        .iter()
        .map(|point| {
            let adjacent = numbers
                .iter()
                .filter(|(x_range, y_range, _)| {
                    x_range.contains(&point.x) && y_range.contains(&point.y)
                })
                .collect_vec();
            if adjacent.len() == 2 {
                adjacent.get(0).unwrap().2 * adjacent.get(1).unwrap().2
            } else {
                0
            }
        })
        .sum::<u32>()
        .print();
}
