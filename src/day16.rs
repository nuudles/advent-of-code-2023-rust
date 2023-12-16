use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{point::Point, selfprint::SelfPrint};

fn energized_count(
    position: Point<usize>,
    delta: Point<isize>,
    map: &HashMap<Point<usize>, char>,
    x_max: usize,
    y_max: usize,
) -> usize {
    let mut beams = HashSet::new();

    // Account for the first position
    match map.get(&position) {
        Some('|') => {
            if delta.x == 0 {
                beams.insert((position, delta));
            } else {
                beams.insert((position, Point { x: 0, y: -1 }));
                beams.insert((position, Point { x: 0, y: 1 }));
            }
        }
        Some('-') => {
            if delta.y == 0 {
                beams.insert((position, delta));
            } else {
                beams.insert((position, Point { x: -1, y: 0 }));
                beams.insert((position, Point { x: 1, y: 0 }));
            }
        }
        Some('\\') => {
            // (0, 1) => (1, 0)
            // (0, -1) => (-1, 0)
            // (1, 0) => (0, 1)
            // (-1, 0) => (0, -1)
            beams.insert((
                position,
                Point {
                    x: delta.y,
                    y: delta.x,
                },
            ));
        }
        Some('/') => {
            // (0, 1) => (-1, 0)
            // (0, -1) => (1, 0)
            // (1, 0) => (0, -1)
            // (-1, 0) => (0, 1)
            beams.insert((
                position,
                Point {
                    x: -delta.y,
                    y: -delta.x,
                },
            ));
        }
        _ => {
            beams.insert((position, delta));
        }
    }
    let mut visited = beams.clone();
    loop {
        if beams.is_empty() {
            break;
        }
        let mut next_beams = HashSet::new();
        for (position, delta) in &beams {
            let next_position = Point {
                x: position.x.saturating_add_signed(delta.x),
                y: position.y.saturating_add_signed(delta.y),
            };
            if next_position == *position || next_position.x >= x_max || next_position.y >= y_max {
                continue;
            }
            match map.get(&next_position) {
                Some('|') => {
                    if delta.x == 0 {
                        next_beams.insert((next_position, *delta));
                    } else {
                        next_beams.insert((next_position, Point { x: 0, y: -1 }));
                        next_beams.insert((next_position, Point { x: 0, y: 1 }));
                    }
                }
                Some('-') => {
                    if delta.y == 0 {
                        next_beams.insert((next_position, *delta));
                    } else {
                        next_beams.insert((next_position, Point { x: -1, y: 0 }));
                        next_beams.insert((next_position, Point { x: 1, y: 0 }));
                    }
                }
                Some('\\') => {
                    // (0, 1) => (1, 0)
                    // (0, -1) => (-1, 0)
                    // (1, 0) => (0, 1)
                    // (-1, 0) => (0, -1)
                    next_beams.insert((
                        next_position,
                        Point {
                            x: delta.y,
                            y: delta.x,
                        },
                    ));
                }
                Some('/') => {
                    // (0, 1) => (-1, 0)
                    // (0, -1) => (1, 0)
                    // (1, 0) => (0, -1)
                    // (-1, 0) => (0, 1)
                    next_beams.insert((
                        next_position,
                        Point {
                            x: -delta.y,
                            y: -delta.x,
                        },
                    ));
                }
                _ => {
                    next_beams.insert((next_position, *delta));
                }
            }
        }
        beams = next_beams
            .iter()
            .filter(|n| !visited.contains(n))
            .copied()
            .collect();
        for b in &beams {
            visited.insert(b.clone());
        }
    }
    visited.iter().map(|v| v.0).sorted().dedup().count()
}

pub fn part1(input: String) {
    let mut map = HashMap::new();
    let y_max = input.lines().count();
    let mut x_max = usize::MAX;
    for (y, line) in input.lines().enumerate() {
        if x_max == usize::MAX {
            x_max = line.chars().count();
        }
        for (x, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            map.insert(Point { x, y }, char);
        }
    }

    energized_count(
        Point { x: 0, y: 0 },
        Point { x: 1, y: 0 },
        &map,
        x_max,
        y_max,
    )
    .print();
}

pub fn part2(input: String) {
    let mut map = HashMap::new();
    let y_max = input.lines().count();
    let mut x_max = usize::MAX;
    for (y, line) in input.lines().enumerate() {
        if x_max == usize::MAX {
            x_max = line.chars().count();
        }
        for (x, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            map.insert(Point { x, y }, char);
        }
    }

    // Iterate all the things!
    let mut max = usize::MIN;
    for y in 0..y_max {
        max = max.max(energized_count(
            Point { x: 0, y },
            Point { x: 1, y: 0 },
            &map,
            x_max,
            y_max,
        ));
        max = max.max(energized_count(
            Point { x: x_max - 1, y },
            Point { x: -1, y: 0 },
            &map,
            x_max,
            y_max,
        ));
    }
    for x in 0..x_max {
        max = max.max(energized_count(
            Point { x, y: 0 },
            Point { x: 0, y: 1 },
            &map,
            x_max,
            y_max,
        ));
        max = max.max(energized_count(
            Point { x, y: y_max - 1 },
            Point { x: 0, y: -1 },
            &map,
            x_max,
            y_max,
        ));
    }
    println!("{}", max);
}
