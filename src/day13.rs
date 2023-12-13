use std::collections::HashSet;

use itertools::iproduct;

use crate::point::Point;

fn value(set: &HashSet<Point<usize>>, max_x: usize, max_y: usize, old_value: usize) -> usize {
    if let Some(column) = (1..max_x).find(|&x| {
        if old_value == x {
            return false;
        }

        let size = x.min(max_x - x);
        let left_range = (x - size)..x;
        let right_range = x..(x + size);
        let left = (&set)
            .iter()
            .filter(|p| left_range.contains(&p.x))
            .copied()
            .collect::<HashSet<_>>();
        let right = (&set)
            .iter()
            .filter(|p| right_range.contains(&p.x))
            .map(|p| Point {
                x: x - (p.x - x) - 1,
                y: p.y,
            })
            .collect::<HashSet<_>>();

        left == right
    }) {
        return column;
    } else if let Some(row) = (1..max_y).find(|&y| {
        if old_value == y * 100 {
            return false;
        }

        let size = y.min(max_y - y);
        let top_range = (y - size)..y;
        let bottom_range = y..(y + size);
        let top = (&set)
            .iter()
            .filter(|p| top_range.contains(&p.y))
            .copied()
            .collect::<HashSet<_>>();
        let bottom = (&set)
            .iter()
            .filter(|p| bottom_range.contains(&p.y))
            .map(|p| Point {
                x: p.x,
                y: y - (p.y - y) - 1,
            })
            .collect::<HashSet<_>>();

        top == bottom
    }) {
        return row * 100;
    }
    0
}

pub fn part1(input: String) {
    let mut total = 0;
    for section in input.split("\n\n") {
        let mut set = HashSet::new();
        let max_y = section.lines().count();
        let mut max_x = usize::MIN;
        for (y, line) in section.lines().enumerate() {
            if max_x == usize::MIN {
                max_x = line.chars().count();
            }
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    set.insert(Point { x, y });
                }
            }
        }
        total += value(&set, max_x, max_y, usize::MAX);
    }
    println!("{}", total);
}

pub fn part2(input: String) {
    let mut total = 0;
    for section in input.split("\n\n") {
        let mut set = HashSet::new();
        let max_y = section.lines().count();
        let mut max_x = usize::MIN;
        for (y, line) in section.lines().enumerate() {
            if max_x == usize::MIN {
                max_x = line.chars().count();
            }
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    set.insert(Point { x, y });
                }
            }
        }
        let old_value = value(&set, max_x, max_y, usize::MAX);
        for (y, x) in iproduct!(0..max_y, 0..max_x) {
            let mut new_set = set.clone();
            let p = Point { x, y };
            if !new_set.remove(&p) {
                new_set.insert(p);
            }
            let value = value(&new_set, max_x, max_y, old_value);
            if value > 0 && value != old_value {
                total += value;
                break;
            }
        }
    }
    println!("{}", total);
}
