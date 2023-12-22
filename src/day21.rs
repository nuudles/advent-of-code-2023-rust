use std::collections::{BTreeMap, BTreeSet};

use crate::point::Point;

pub fn part1(input: String) {
    let mut map = BTreeMap::new();

    let mut points = BTreeSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let p: Point<i64> = Point {
                x: x as i64,
                y: y as i64,
            };
            map.insert(p, char == '#');
            if char == 'S' {
                points.insert(p);
            }
        }
    }

    for _ in 0..64 {
        let mut next_points = BTreeSet::new();

        for point in points {
            for neighbor in point.neighbors() {
                if let Some(is_wall) = map.get(&neighbor) {
                    if is_wall == &false {
                        next_points.insert(neighbor);
                    }
                }
            }
        }

        points = next_points;
    }
    println!("{}", points.len());
}

pub fn part2(input: String) {
    let mut maps = BTreeMap::new();

    let mut points = BTreeSet::new();
    let y_max = input.lines().count() as i64;
    let mut x_max = i64::MAX;

    for (y, line) in input.lines().enumerate() {
        if x_max == i64::MAX {
            x_max = line.len() as i64;
        }
        // let mut wall_map = HashSet::new();
        for (x, char) in line.chars().enumerate() {
            let p = Point {
                x: x as i64,
                y: y as i64,
            };
            maps.insert(p, char == '#');
            if char == 'S' {
                points.insert(p);
            }
        }
        // wall_maps.push(wall_map);
    }

    for i in 1..1000 {
        let mut next_points = BTreeSet::new();

        for point in points {
            for neighbor in point.neighbors() {
                let (mut x, mut y) = (neighbor.x, neighbor.y);
                let x_multiplier = if x >= 0 { x / x_max } else { (x / x_max) - 1 };
                let y_multiplier = if y >= 0 { y / y_max } else { (y / y_max) - 1 };
                x -= x_multiplier * x_max;
                y -= y_multiplier * y_max;

                let is_wall = maps
                    .get(&Point {
                        x: x % x_max,
                        y: y % y_max,
                    })
                    .expect("Neighbor not found");
                if is_wall == &false {
                    next_points.insert(neighbor);
                }
            }
        }

        if (i - x_max / 2) % x_max == 0 {
            println!("{}: {}", i, next_points.len());
        }

        points = next_points;
    }

    // Wolfram Alpha finds this formula: 3810 - 15286 n + 15387 n^2
    // Calculate where n = (26501365 - 65) / 131 + 1 = 202301

    // let size = input.lines().count() as i64;
    // let num_steps = 26501365;

    // let going_down = wall_maps.iter().cycle().skip(start.y);
    // let going_up = wall_maps
    //     .iter()
    //     .rev()
    //     .cycle()
    //     .skip(size as usize - start.y - 1);

    // let mut count = 0;
    // for (n, wall_down, wall_up) in izip!((1..=num_steps + 1).rev(), going_down, going_up) {
    //     println!("{n}");
    //     // println!("Up {:?}", wall_up.iter().sorted().collect_vec());
    //     let is_start: bool = n == num_steps + 1;
    //     let mut x = start.x as i64 - n + 1;
    //     while x < 0 {
    //         x += size;
    //     }
    //     for _ in 0..n {
    //         if !wall_down.contains(&x) {
    //             count += 1;
    //         }
    //         if !is_start && !wall_up.contains(&x) {
    //             count += 1;
    //         }
    //         x += 2;
    //         if x >= size {
    //             x -= size;
    //         }
    //     }
    //     // break;
    // }
    // println!("{}", count);

    // let mut is_wall_map = BTreeMap::new();
    // let y_max = input.lines().count() as i64;
    // let mut x_max = i64::MAX;

    // let mut points = BTreeMap::new();

    // for (y, line) in input.lines().enumerate() {
    //     if x_max == i64::MAX {
    //         x_max = line.len() as i64;
    //     }
    //     for (x, char) in line.chars().enumerate() {
    //         let p: Point<i64> = Point {
    //             x: x as i64,
    //             y: y as i64,
    //         };
    //         is_wall_map.insert(p, char == '#');
    //         if char == 'S' {
    //             points.insert(BTreeSet::from([p]), BTreeSet::from([(0, 0)]));
    //         }
    //     }
    // }

    // // let mut last_last_delta = 0i64;
    // // let mut last_delta = 0i64;

    // // let mut counts = BTreeMap::new();

    // for _ in 0..10 {
    //     let mut next_points = BTreeMap::new();

    //     for (maps, windows) in points {
    //         let mut next_maps = BTreeMap::new();
    //         for point in maps {
    //             for neighbor in point.neighbors() {
    //                 let (x_window, x) = if neighbor.x < 0 {
    //                     (windows.0 - 1, neighbor.x + x_max)
    //                 } else if neighbor.x >= x_max {
    //                     (windows.0 + 1, neighbor.x - x_max)
    //                 } else {
    //                     (windows.0, neighbor.x)
    //                 };
    //                 let (y_window, y) = if neighbor.y < 0 {
    //                     (windows.1 - 1, neighbor.y + y_max)
    //                 } else if neighbor.y >= y_max {
    //                     (windows.1 + 1, neighbor.y - y_max)
    //                 } else {
    //                     (windows.1, neighbor.y)
    //                 };
    //                 let p = Point { x, y };
    //                 let is_wall = is_wall_map.get(&p).expect("Neighbor not found!");
    //                 if is_wall == &false {
    //                     next_maps
    //                         .entry((x_window, y_window))
    //                         .or_insert(BTreeSet::new())
    //                         .0
    //                         .insert(p);
    //                 }
    //             }
    //         }
    //         for (map, count) in next_maps.values() {
    //             *next_points.entry(map.clone()).or_default() += count;
    //         }
    //     }

    //     // println!("{:?}", next_points.iter().map(|(_, c)| c).collect_vec());
    //     println!("{:?}", next_points);

    //     points = next_points;
    // }

    // points
    //     .iter()
    //     .fold(0usize, |r, (m, c)| r + m.len() * c)
    //     .print();
}
