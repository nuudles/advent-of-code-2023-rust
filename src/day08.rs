use std::collections::{HashMap, HashSet};

use gcd::Gcd;
use itertools::Itertools;
use regex::Regex;

use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    let instructions = input
        .lines()
        .next()
        .expect("First line not found")
        .chars()
        .cycle();
    let regex = Regex::new(r"[A-Z\d]{3}").expect("Invalid Regex");
    let map = input
        .lines()
        .skip(2)
        .map(|line| {
            let (src, left, right) = regex
                .find_iter(line)
                .collect_tuple()
                .expect("Could not parse");
            (src.as_str(), (left.as_str(), right.as_str()))
        })
        .collect::<HashMap<_, _>>();
    let mut current = "AAA";
    let mut count = 0;
    for direction in instructions {
        let entry = map.get(current).expect("Could not found entry");
        if direction == 'L' {
            current = entry.0;
        } else {
            current = entry.1;
        }
        count += 1;
        if current == "ZZZ" {
            break;
        }
    }
    println!("{}", count);
}

pub fn part2(input: String) {
    let instructions = input
        .lines()
        .next()
        .expect("First line not found")
        .chars()
        .collect_vec();
    let regex = Regex::new(r"[A-Z\d]{3}").expect("Invalid Regex");
    let map = input
        .lines()
        .skip(2)
        .map(|line| {
            let (src, left, right) = regex
                .find_iter(line)
                .collect_tuple()
                .expect("Could not parse");
            (src.as_str(), (left.as_str(), right.as_str()))
        })
        .collect::<HashMap<_, _>>();
    let mut counts = Vec::<u64>::new();
    for key in map.keys().filter(|k| k.ends_with("A")).copied() {
        let mut count = 0;
        let mut current = key;
        for direction in instructions.iter().cycle() {
            let entry = map.get(current).expect("Could not found entry");
            if direction == &'L' {
                current = entry.0;
            } else {
                current = entry.1;
            }
            count += 1;
            if current.ends_with("Z") {
                break;
            }
        }
        counts.push(count);
    }

    counts.iter().fold(1, |result, a| lcm(result, *a)).print();
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / a.gcd(b)
}
