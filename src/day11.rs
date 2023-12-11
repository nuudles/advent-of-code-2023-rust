use std::collections::HashSet;

use itertools::Itertools;

use crate::{point::Point, selfprint::SelfPrint};

pub fn part1(input: String) {
    let mut galaxies = HashSet::new();
    let mut empty_rows = (0..input.lines().count()).collect::<HashSet<_>>();
    let mut empty_columns = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        if empty_columns.is_empty() {
            empty_columns = (0..line.chars().count()).collect();
        }
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.insert(Point { x: x, y: y });
                empty_rows.remove(&y);
                empty_columns.remove(&x);
            }
        }
    }
    // Part 1
    galaxies
        .iter()
        .map(|point| Point {
            x: point.x + empty_columns.iter().filter(|&&x| x < point.x).count(),
            y: point.y + empty_rows.iter().filter(|&&y| y < point.y).count(),
        })
        .tuple_combinations()
        .map(|(a, b)| a.manhattan_distance(&b))
        .sum::<usize>()
        .print();

    // Part 2
    galaxies
        .iter()
        .map(|point| Point {
            x: point.x + empty_columns.iter().filter(|&&x| x < point.x).count() * (1000000 - 1),
            y: point.y + empty_rows.iter().filter(|&&y| y < point.y).count() * (1000000 - 1),
        })
        .tuple_combinations()
        .map(|(a, b)| a.manhattan_distance(&b))
        .sum::<usize>()
        .print();
}
