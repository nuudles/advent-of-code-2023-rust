use std::{collections::HashSet, ops::RangeInclusive};

use itertools::Itertools;

use crate::parse_nums::parse_nums;

fn range_intersects(r1: &RangeInclusive<u64>, r2: &RangeInclusive<u64>) -> bool {
    r2.contains(r1.start())
        || r2.contains(r1.end())
        || r1.contains(r2.start())
        || r1.contains(r2.end())
}

fn fall_them_down(
    bricks: &Vec<(
        RangeInclusive<u64>,
        RangeInclusive<u64>,
        RangeInclusive<u64>,
    )>,
) -> Vec<(
    RangeInclusive<u64>,
    RangeInclusive<u64>,
    RangeInclusive<u64>,
)> {
    let mut next_bricks = Vec::new();

    for brick in bricks {
        if brick.2.contains(&1) {
            next_bricks.push(brick.clone());
            continue;
        }
        let potential = (
            brick.0.clone(),
            brick.1.clone(),
            (brick.2.start() - 1)..=(brick.2.end() - 1),
        );
        if next_bricks.iter().any(|(x, y, z)| {
            range_intersects(z, &potential.2)
                && range_intersects(x, &potential.0)
                && range_intersects(y, &potential.1)
        }) {
            next_bricks.push(brick.clone());
        } else {
            next_bricks.push(potential);
        }
    }

    next_bricks
        .iter()
        .sorted_by_key(|(_, _, z)| *z.start())
        .cloned()
        .collect()
}

pub fn part1(input: String) {
    let mut bricks = input
        .lines()
        .map(|line| {
            let (x1, y1, z1, x2, y2, z2) = parse_nums::<u64>(line)
                .collect_tuple()
                .expect("Error parsing");
            (
                x1.min(x2)..=x1.max(x2),
                y1.min(y2)..=y1.max(y2),
                z1.min(z2)..=z1.max(z2),
            )
        })
        .sorted_by_key(|(_, _, z)| *z.start())
        .collect_vec();

    loop {
        let next_bricks = fall_them_down(&bricks);
        if bricks == next_bricks {
            break;
        }
        bricks = next_bricks;
    }

    let mut not_disintegratable = HashSet::new();

    // Check disintegratable bricks
    for brick in &bricks {
        let supporting = bricks
            .iter()
            .filter(|(x, y, z)| {
                z.contains(&(brick.2.start() - 1))
                    && range_intersects(x, &brick.0)
                    && range_intersects(y, &brick.1)
            })
            .collect_vec();
        if supporting.len() == 1 {
            not_disintegratable.insert(*supporting.first().unwrap());
        }
    }
    println!("Part 1: {}", bricks.len() - not_disintegratable.len());

    let mut count = 0;
    for to_disintegrate in not_disintegratable {
        let after_disintegrating = bricks
            .iter()
            .filter(|b| b != &to_disintegrate)
            .cloned()
            .collect_vec();
        let mut next_bricks = Vec::new();
        for brick in &after_disintegrating {
            if brick.2.contains(&1) {
                next_bricks.push(brick.clone());
                continue;
            }
            let potential = (
                brick.0.clone(),
                brick.1.clone(),
                (brick.2.start() - 1)..=(brick.2.end() - 1),
            );
            if next_bricks.iter().any(|(x, y, z)| {
                range_intersects(z, &potential.2)
                    && range_intersects(x, &potential.0)
                    && range_intersects(y, &potential.1)
            }) {
                next_bricks.push(brick.clone());
            } else {
                next_bricks.push(potential);
                count += 1;
            }
        }
    }
    println!("Part 2: {}", count);
}
