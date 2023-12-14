use std::collections::BTreeSet;

use crate::{point::Point, selfprint::SelfPrint};

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn tilt(
    direction: Direction,
    rounded: &BTreeSet<Point<usize>>,
    cube: &BTreeSet<Point<usize>>,
    max_x: usize,
    max_y: usize,
) -> BTreeSet<Point<usize>> {
    let mut rounded = rounded.clone();
    loop {
        let mut new_rounded = BTreeSet::new();

        for p in &rounded {
            if let Some(next) = match direction {
                Direction::UP => {
                    if p.y > 0 {
                        Some(p.up())
                    } else {
                        None
                    }
                }
                Direction::DOWN => {
                    if p.y < max_y - 1 {
                        Some(p.down())
                    } else {
                        None
                    }
                }
                Direction::LEFT => {
                    if p.x > 0 {
                        Some(p.left())
                    } else {
                        None
                    }
                }
                Direction::RIGHT => {
                    if p.x < max_x - 1 {
                        Some(p.right())
                    } else {
                        None
                    }
                }
            } {
                if rounded.contains(&next) || cube.contains(&next) {
                    new_rounded.insert(p.clone());
                } else {
                    new_rounded.insert(next);
                }
            } else {
                new_rounded.insert(p.clone());
            }
        }

        if new_rounded == rounded {
            break;
        }
        rounded = new_rounded;
    }
    rounded
}

pub fn part1(input: String) {
    let mut rounded = BTreeSet::new();
    let mut cube = BTreeSet::new();
    let mut max_x = usize::MAX;
    for (y, line) in input.lines().enumerate() {
        if max_x == usize::MAX {
            max_x = line.chars().count();
        }
        for (x, char) in line.chars().enumerate() {
            let p = Point { x, y };
            if char == 'O' {
                rounded.insert(p);
            } else if char == '#' {
                cube.insert(p);
            }
        }
    }

    let max_y = input.lines().count();
    rounded = tilt(Direction::UP, &rounded, &cube, max_x, max_y);
    rounded.iter().map(|p| max_y - p.y).sum::<usize>().print();
}

pub fn part2(input: String) {
    let mut rounded = BTreeSet::new();
    let mut cube = BTreeSet::new();
    let mut max_x = usize::MAX;
    for (y, line) in input.lines().enumerate() {
        if max_x == usize::MAX {
            max_x = line.chars().count();
        }
        for (x, char) in line.chars().enumerate() {
            let p = Point { x, y };
            if char == 'O' {
                rounded.insert(p);
            } else if char == '#' {
                cube.insert(p);
            }
        }
    }
    let max_y = input.lines().count();
    let mut seen = BTreeSet::new();
    let mut first_cycle = usize::MAX;
    let mut cycle_count = usize::MAX;
    let mut loads = Vec::new();
    for i in 0.. {
        rounded = tilt(Direction::UP, &rounded, &cube, max_x, max_y);
        rounded = tilt(Direction::LEFT, &rounded, &cube, max_x, max_y);
        rounded = tilt(Direction::DOWN, &rounded, &cube, max_x, max_y);
        rounded = tilt(Direction::RIGHT, &rounded, &cube, max_x, max_y);
        if seen.contains(&rounded) {
            if first_cycle == usize::MAX {
                first_cycle = i;
                seen.clear();
            } else if cycle_count == usize::MAX {
                cycle_count = i - first_cycle;
                seen.clear();
                break;
            }
        }
        seen.insert(rounded.clone());
        if first_cycle != usize::MAX {
            loads.push(rounded.iter().map(|p| max_y - p.y).sum::<usize>());
        }
    }
    loads
        .get((1000000000 - first_cycle - 1) % cycle_count)
        .expect("No load found")
        .print();
}
