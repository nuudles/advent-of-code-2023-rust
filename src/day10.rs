use std::collections::{HashMap, HashSet};

use crate::point::Point;

pub fn part1(input: String) {
    let mut map = HashMap::new();
    let mut start: Point<i64> = Point { x: -1, y: -1 };
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            map.insert(
                Point {
                    x: x as i64,
                    y: y as i64,
                },
                char,
            );
            if char == 'S' {
                start = Point {
                    x: x as i64,
                    y: y as i64,
                }
            }
        }
    }
    let mut seen = HashSet::from([start]);
    let mut next = [
        ((0, -1), HashSet::from(['|', '7', 'F'])),
        ((0, 1), HashSet::from(['|', 'L', 'J'])),
        ((-1, 0), HashSet::from(['-', 'L', 'F'])),
        ((1, 0), HashSet::from(['-', '7', 'J'])),
    ]
    .iter()
    .filter_map(|(delta, valid)| {
        if valid.contains(
            map.get(&Point {
                x: start.x + delta.0,
                y: start.y + delta.1,
            })
            .unwrap_or(&'.'),
        ) {
            Some((start, *delta))
        } else {
            None
        }
    })
    .collect::<HashSet<_>>();

    // Replace S with actual pipe
    if next.iter().map(|t| t.1).collect::<HashSet<_>>() == HashSet::from([(0, 1), (0, -1)]) {
        map.insert(start, '|');
    } else if next.iter().map(|t| t.1).collect::<HashSet<_>>() == HashSet::from([(1, 0), (-1, 0)]) {
        map.insert(start, '-');
    } else if next.iter().map(|t| t.1).collect::<HashSet<_>>() == HashSet::from([(1, 0), (0, -1)]) {
        map.insert(start, 'L');
    } else if next.iter().map(|t| t.1).collect::<HashSet<_>>() == HashSet::from([(-1, 0), (0, -1)])
    {
        map.insert(start, 'J');
    } else if next.iter().map(|t| t.1).collect::<HashSet<_>>() == HashSet::from([(-1, 0), (0, 1)]) {
        map.insert(start, '7');
    } else if next.iter().map(|t| t.1).collect::<HashSet<_>>() == HashSet::from([(1, 0), (0, 1)]) {
        map.insert(start, 'F');
    }

    // Part 1
    let mut count = 0;
    loop {
        next = next
            .iter()
            .map(|(point, delta)| {
                let neighbor = Point {
                    x: point.x + delta.0,
                    y: point.y + delta.1,
                };
                match map.get(&neighbor).unwrap_or(&'.') {
                    '|' => {
                        if delta.1 == -1 {
                            (neighbor, (0, -1))
                        } else {
                            (neighbor, (0, 1))
                        }
                    }
                    '-' => {
                        if delta.0 == -1 {
                            (neighbor, (-1, 0))
                        } else {
                            (neighbor, (1, 0))
                        }
                    }
                    'L' => {
                        if delta.1 == 1 {
                            (neighbor, (1, 0))
                        } else {
                            (neighbor, (0, -1))
                        }
                    }
                    'J' => {
                        if delta.1 == 1 {
                            (neighbor, (-1, 0))
                        } else {
                            (neighbor, (0, -1))
                        }
                    }
                    '7' => {
                        if delta.1 == -1 {
                            (neighbor, (-1, 0))
                        } else {
                            (neighbor, (0, 1))
                        }
                    }
                    'F' => {
                        if delta.1 == -1 {
                            (neighbor, (1, 0))
                        } else {
                            (neighbor, (0, 1))
                        }
                    }
                    _ => panic!("Shouldn't have been able to get here?!"),
                }
            })
            .collect();
        if next.iter().any(|(p, _)| seen.contains(p)) {
            next.iter().for_each(|p| {
                seen.insert(p.0);
            });
            break;
        }
        next.iter().for_each(|p| {
            seen.insert(p.0);
        });
        count += 1;
    }
    println!("Part 1: {}", count);

    // Part 2
    let mut inside = HashSet::new();
    let mut outside = HashSet::new();
    for y in 0..input.lines().count() {
        for x in 0..input.lines().next().unwrap().chars().count() {
            let mut point = Point {
                x: x as i64,
                y: y as i64,
            };
            if seen.contains(&point) {
                continue;
            }
            let mut intersections = 0;
            let mut last_interesting = ' ';
            while let Some(&c) = map.get(&point) {
                if inside.contains(&point) && intersections == 0 {
                    intersections = 1;
                    break;
                } else if outside.contains(&point) && intersections == 0 {
                    intersections = 2;
                    break;
                }
                if seen.contains(&point) {
                    match c {
                        '-' => intersections += 1,
                        'J' => {
                            if last_interesting == 'F' {
                                intersections += 1;
                                last_interesting = ' ';
                            } else {
                                last_interesting = 'J';
                            }
                        }
                        'F' => {
                            if last_interesting == 'J' {
                                intersections += 1;
                                last_interesting = ' ';
                            } else if last_interesting == 'L' {
                                last_interesting = ' ';
                            } else {
                                last_interesting = 'F';
                            }
                        }
                        '7' => {
                            if last_interesting == 'L' {
                                intersections += 1;
                                last_interesting = ' ';
                            } else if last_interesting == 'J' {
                                last_interesting = ' ';
                            } else {
                                last_interesting = '7';
                            }
                        }
                        'L' => {
                            if last_interesting == '7' {
                                intersections += 1;
                                last_interesting = ' ';
                            } else {
                                last_interesting = 'L';
                            }
                        }
                        _ => (),
                    }
                }
                point = point.up();
            }
            if intersections % 2 == 1 {
                inside.insert(Point {
                    x: x as i64,
                    y: y as i64,
                });
            } else {
                outside.insert(Point {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }
    println!("Part 2: {}", inside.len());
}
