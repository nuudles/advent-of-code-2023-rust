use std::collections::{BTreeMap, BTreeSet};

use itertools::{iproduct, Itertools};
use pathfinding::directed::dijkstra::dijkstra;

use crate::{point::Point, selfprint::SelfPrint};

fn longest_path(
    from: Point<i64>,
    to: Point<i64>,
    map: &BTreeMap<Point<i64>, char>,
    seen: BTreeSet<Point<i64>>,
) -> usize {
    if from == to {
        return 0;
    }

    let mut longest = usize::MIN;
    for neighbor in from.neighbors() {
        if seen.contains(&neighbor) {
            continue;
        }
        if let Some(&c) = map.get(&neighbor) {
            match c {
                '#' => continue,
                '>' | 'v' | '<' | '^' => {
                    let next = match c {
                        '>' => neighbor.right(),
                        'v' => neighbor.down(),
                        '<' => neighbor.left(),
                        '^' => neighbor.up(),
                        _ => panic!("Should be impossible to get here"),
                    };
                    if next == from {
                        // We tried to up a slippery slope
                        continue;
                    }
                    let mut seen = seen.clone();
                    seen.insert(neighbor);
                    seen.insert(next);
                    longest = longest.max(longest_path(next, to, map, seen) + 2);
                }
                _ => {
                    let mut seen = seen.clone();
                    seen.insert(neighbor);
                    longest = longest.max(longest_path(neighbor, to, map, seen) + 1);
                }
            }
        }
    }
    longest
}

pub fn part1(input: String) {
    let map = input
        .lines()
        .enumerate()
        .fold(BTreeMap::new(), |mut map, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                map.insert(
                    Point {
                        x: x as i64,
                        y: y as i64,
                    },
                    c,
                );
            });
            map
        });
    let y_max = input.lines().count() as i64;
    let x_max = input.lines().next().expect("First line not found").len() as i64;
    let start = Point { x: 1, y: 0 };
    let end = Point {
        x: x_max - 2,
        y: y_max - 1,
    };
    longest_path(start, end, &map, BTreeSet::from([start])).print();
}

fn longest_path_by_distance_map(
    from: &Point<i64>,
    to: &Point<i64>,
    distances: &BTreeMap<BTreeSet<Point<i64>>, usize>,
    seen: BTreeSet<Point<i64>>,
) -> usize {
    if from == to {
        return 0;
    }

    let mut longest = 0;
    for (key, distance) in distances {
        if !key.contains(from) {
            continue;
        }
        let other = key
            .iter()
            .filter(|i| i != &from)
            .next()
            .expect("We're supposed to have a second point in the key");
        if seen.contains(other) {
            continue;
        }
        let mut seen = seen.clone();
        seen.insert(other.clone());

        let next = longest_path_by_distance_map(other, to, distances, seen);
        longest = longest.max(next + *distance);
    }

    longest
}

pub fn part2(input: String) {
    let map = input
        .lines()
        .enumerate()
        .fold(BTreeMap::new(), |mut map, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                map.insert(
                    Point {
                        x: x as i64,
                        y: y as i64,
                    },
                    c,
                );
            });
            map
        });
    let y_max = input.lines().count() as i64;
    let x_max = input.lines().next().expect("First line not found").len() as i64;
    let start = Point { x: 1, y: 0 };
    let end = Point {
        x: x_max - 2,
        y: y_max - 1,
    };

    let mut intersections = iproduct!(0..x_max, 0..y_max)
        .filter_map(|(x, y)| {
            let p = Point { x, y };
            if map.get(&p).unwrap_or(&'#') != &'#'
                && p.neighbors()
                    .iter()
                    .filter(|n| map.get(*n).unwrap_or(&'#') != &'#')
                    .count()
                    > 2
            {
                Some(p)
            } else {
                None
            }
        })
        .collect::<BTreeSet<_>>();

    intersections.insert(start);
    intersections.insert(end);

    // Find all of the connected intersections (including start and end) and their respective "distances"
    let distances =
        intersections
            .iter()
            .tuple_combinations()
            .fold(BTreeMap::new(), |mut distances, (a, b)| {
                if let Some(path) = dijkstra(
                    a,
                    |p| {
                        p.neighbors()
                            .iter()
                            .filter(|n| {
                                (!intersections.contains(&n) || n == &b)
                                    && map.get(*n).unwrap_or(&'#') != &'#'
                            })
                            .map(|n| (*n, 1usize))
                            .collect_vec()
                    },
                    |p| p == b,
                ) {
                    distances.insert(BTreeSet::from([*a, *b]), path.1);
                }
                distances
            });
    longest_path_by_distance_map(&start, &end, &distances, BTreeSet::from([start])).print();
}
