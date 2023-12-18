use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;

use crate::point::Point;

pub fn part1(input: String) {
    let mut trench = HashMap::new();
    let regex = Regex::new(r"([UDLR]) (\d+) \((\#[a-f0-9]{6})\)").expect("Invalid Regex");
    let mut point = Point { x: 0i64, y: 0i64 };
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (i64::MAX, i64::MIN, i64::MAX, i64::MIN);
    for capture in regex.captures_iter(&input) {
        let (_, direction, count, color) = capture
            .iter()
            .filter_map(|c| c)
            .collect_tuple()
            .expect("Invalid capture");
        for _ in 0..count.as_str().parse::<usize>().unwrap_or_default() {
            point = match direction.as_str() {
                "U" => point.up(),
                "D" => point.down(),
                "L" => point.left(),
                "R" => point.right(),
                _ => panic!("Invalid direction"),
            };
            trench.insert(point, color);
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
        }
    }
    let mut filled = HashSet::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let mut p = Point { x, y };
            let mut left_or_right = 0; // left = 1, right = -1
            let mut intersections = 0;
            if trench.contains_key(&p) {
                continue;
            }
            while p.y >= min_y {
                if trench.contains_key(&p) {
                    let left = p.left();
                    let right = p.right();
                    if left_or_right == 0 {
                        if trench.contains_key(&left) && trench.contains_key(&right) {
                            intersections += 1;
                        } else if trench.contains_key(&left) {
                            left_or_right = 1;
                        } else if trench.contains_key(&right) {
                            left_or_right = -1;
                        } else {
                            intersections += 1;
                        }
                    } else if left_or_right == 1 {
                        if trench.contains_key(&left) {
                            left_or_right = 0;
                        } else if trench.contains_key(&right) {
                            left_or_right = 0;
                            intersections += 1;
                        }
                    } else if left_or_right == -1 {
                        if trench.contains_key(&right) {
                            left_or_right = 0;
                        } else if trench.contains_key(&left) {
                            left_or_right = 0;
                            intersections += 1;
                        }
                    }
                }
                p = p.up();
            }
            if intersections % 2 == 1 {
                filled.insert(Point { x, y });
            }
        }
    }
    println!("{}", filled.len() + trench.len());
}

pub fn part2(input: String) {
    let regex = Regex::new(r"\#([a-f0-9]{6})").expect("Invalid Regex");
    let mut point = Point { x: 0i64, y: 0i64 };
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (i64::MAX, i64::MIN, i64::MAX, i64::MIN);

    let mut x_ranges = HashSet::new();
    let mut y_ranges = HashSet::new();

    for capture in regex.captures_iter(&input) {
        let (_, hex) = capture
            .iter()
            .filter_map(|c| c)
            .collect_tuple()
            .expect("Invalid capture");
        let count = i64::from_str_radix(&hex.as_str()[0..5], 16).unwrap_or_default();
        match hex.as_str().chars().last().unwrap_or_default() {
            '2' => {
                x_ranges.insert(((point.x - count)..point.x + 1, point.y));
                point.x -= count;
            }
            '0' => {
                x_ranges.insert((point.x..point.x + count + 1, point.y));
                point.x += count;
            }
            '3' => {
                y_ranges.insert(((point.y - count)..point.y + 1, point.x));
                point.y -= count;
            }
            '1' => {
                y_ranges.insert((point.y..point.y + count + 1, point.x));
                point.y += count;
            }
            _ => panic!("Invalid direction"),
        }
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);
    }
    let mut count = 0;
    for x in min_x..=max_x {
        // println!("{x} {:?} {:?}", x_ranges.len(), y_ranges.len());
        let mut intersecting_x_ranges = x_ranges
            .iter()
            .filter(|(r, _)| r.contains(&x))
            .sorted_by_key(|t| t.1);
        let mut intersecting_y_ranges = y_ranges
            .iter()
            .filter(|t| t.1 == x)
            .sorted_by_key(|(r, _)| r.start);
        let mut next_x_range = intersecting_x_ranges.next();
        let mut next_y_range = intersecting_y_ranges.next();
        let mut is_inside = false;
        while let Some(x_range) = next_x_range {
            let next_y_run = next_y_range.map(|t| t.0.start).unwrap_or(i64::MAX);
            count += 1;
            if x_range.1 == next_y_run {
                let is_left = x_range.0.start == x;
                let y_range = next_y_range.expect("There should have been a y range");
                count += y_range.0.end - y_range.0.start - 2;
                next_x_range = intersecting_x_ranges.next();
                if let Some(x_range) = next_x_range {
                    if is_left == (x_range.0.start == x) {
                        is_inside = !is_inside;
                    }
                }
                next_y_range = intersecting_y_ranges.next();
            } else {
                is_inside = !is_inside;
                let start = x_range.1;
                next_x_range = intersecting_x_ranges.next();
                if let Some(x_range) = next_x_range {
                    if is_inside {
                        count += x_range.1 - start - 1;
                    }
                }
            }
        }
    }
    println!("{}", count);
}
