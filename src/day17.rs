use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use pathfinding::directed::astar::astar;

use crate::{point::Point, selfprint::SelfPrint};

pub fn part1(input: String) {
    let mut map = HashMap::new();
    let y_max = input.lines().count();
    let mut x_max = usize::MAX;
    for (y, line) in input.lines().enumerate() {
        if x_max == usize::MAX {
            x_max = line.chars().count();
        }
        for (x, char) in line.chars().enumerate() {
            map.insert(Point { x, y }, char.to_digit(10).unwrap_or_default());
        }
    }

    let end = Point {
        x: x_max - 1,
        y: y_max - 1,
    }
    .print();
    let left = Point { x: -1isize, y: 0 };
    let right = Point { x: 1isize, y: 0 };
    let up = Point { x: 0isize, y: -1 };
    let down = Point { x: 0isize, y: 1 };
    astar(
        &(Point { x: 0usize, y: 0 }, Point { x: 0isize, y: 0 }, 0),
        |(position, delta, straight_count)| {
            let mut next_directions = HashSet::new();
            if delta == &left || delta == &right {
                next_directions.insert(*delta);
                next_directions.insert(up);
                next_directions.insert(down);
            } else if delta == &up || delta == &down {
                next_directions.insert(*delta);
                next_directions.insert(left);
                next_directions.insert(right);
            } else {
                next_directions.insert(right);
                next_directions.insert(down);
            }
            next_directions
                .iter()
                .filter_map(|d| {
                    let neighbor = Point {
                        x: position.x.saturating_add_signed(d.x),
                        y: position.y.saturating_add_signed(d.y),
                    };
                    if &neighbor == position {
                        return None;
                    }
                    if let Some(heat_cost) = &map.get(&neighbor) {
                        let is_straight = delta == d;
                        if is_straight && straight_count > &2 {
                            return None;
                        }
                        return Some((
                            (
                                neighbor,
                                *d,
                                if is_straight { straight_count + 1 } else { 1 },
                            ),
                            **heat_cost,
                        ));
                    }
                    None
                })
                .collect_vec()
        },
        |(position, _, _)| position.manhattan_distance(&end) as u32,
        |(position, _, _)| position == &end,
    )
    .expect("No path found!")
    .1
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
            map.insert(Point { x, y }, char.to_digit(10).unwrap_or_default());
        }
    }

    let end = Point {
        x: x_max - 1,
        y: y_max - 1,
    };
    let left = Point { x: -1isize, y: 0 };
    let right = Point { x: 1isize, y: 0 };
    let up = Point { x: 0isize, y: -1 };
    let down = Point { x: 0isize, y: 1 };
    let standing_still = Point { x: 0isize, y: 0 };
    astar(
        &(Point { x: 0usize, y: 0 }, Point { x: 0isize, y: 0 }, 0),
        |(position, delta, straight_count)| {
            let mut next_directions = HashSet::new();
            if delta == &left || delta == &right {
                next_directions.insert(*delta);
                next_directions.insert(up);
                next_directions.insert(down);
            } else if delta == &up || delta == &down {
                next_directions.insert(*delta);
                next_directions.insert(left);
                next_directions.insert(right);
            } else {
                next_directions.insert(right);
                next_directions.insert(down);
            }
            next_directions
                .iter()
                .filter_map(|d| {
                    let neighbor = Point {
                        x: position.x.saturating_add_signed(d.x),
                        y: position.y.saturating_add_signed(d.y),
                    };
                    if &neighbor == position {
                        return None;
                    }
                    if let Some(heat_cost) = &map.get(&neighbor) {
                        let is_straight = delta == d;
                        if is_straight && straight_count > &9 {
                            return None;
                        } else if !is_straight && straight_count < &4 && delta != &standing_still {
                            return None;
                        }
                        return Some((
                            (
                                neighbor,
                                *d,
                                if is_straight { straight_count + 1 } else { 1 },
                            ),
                            **heat_cost,
                        ));
                    }
                    None
                })
                .collect_vec()
        },
        |(position, _, _)| position.manhattan_distance(&end) as u32,
        |(position, _, straight_count)| position == &end && straight_count >= &4,
    )
    .expect("No path found!")
    .1
    .print();
}
