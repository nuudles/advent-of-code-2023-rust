use std::collections::HashSet;

use itertools::Itertools;

use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

pub fn part1(input: String) {
    let mut mappings: Vec<Vec<(usize, usize, usize)>> = vec![];

    for section in input.split("\n\n").skip(1) {
        let mut map = vec![];
        for line in section.lines().skip(1) {
            let numbers = parse_nums::<usize>(line);
            let (dest_start, src_start, count) =
                numbers.collect_tuple().expect("Could not find ranges");
            map.push((dest_start, src_start, count));
        }
        mappings.push(map);
    }

    let mut targets: Vec<usize> =
        parse_nums(input.lines().next().expect("No first line found")).collect_vec();
    for map in mappings {
        targets = targets
            .iter()
            .map(|x| {
                if let Some(mapped) = map
                    .iter()
                    .find(|(_, src, count)| x >= src && x < &(src + count))
                {
                    mapped.0 + x - mapped.1
                } else {
                    *x
                }
            })
            .collect();
    }
    targets.iter().min().expect("No minimum found").print();
}

pub fn part2(input: String) {
    let mut targets: HashSet<(usize, usize)> = HashSet::new();
    for (start, count) in parse_nums(input.lines().next().expect("First line not found")).tuples() {
        targets.insert((start, count));
    }
    for section in input.split("\n\n").skip(1) {
        let mut next_targets = HashSet::new();
        for line in section.lines().skip(1) {
            let numbers = parse_nums::<usize>(line);
            let (dest_min, src_min, count) =
                numbers.collect_tuple().expect("Could not find ranges");
            let src_max = src_min + count - 1;
            for (target, target_count) in targets.clone() {
                let target_min = target;
                let target_max = target + target_count - 1;
                if target_max < src_min {
                    continue;
                } else if target_min > src_max {
                    continue;
                } else if target_min >= src_min && target_max <= src_max {
                    // src: (5, 5) 5-9, target: (6, 2) 6-7
                    targets.remove(&(target, target_count));
                    next_targets.insert((dest_min + (target_min - src_min), target_count));
                } else if target_min <= src_min && target_max >= src_max {
                    // src: (15, 1) 15-15, target: (10, 10) 10-19
                    targets.remove(&(target, target_count));
                    if src_min > target_min {
                        targets.insert((target_min, src_min - target_min - 1));
                    }
                    if target_max > src_max {
                        targets.insert((src_max + 1, target_max - src_max));
                    }
                    next_targets.insert((dest_min, src_max - src_min + 1));
                } else if target_min < src_min {
                    // src: (5, 5) 5-9, target: (4, 2) 4-5
                    targets.remove(&(target, target_count));
                    targets.insert((target_min, src_min - target_min));
                    next_targets.insert((dest_min, target_max - src_min + 1));
                } else {
                    // src: (5, 5) 5-9, target: (9, 2) 9-10
                    targets.remove(&(target, target_count));
                    targets.insert((src_max + 1, target_max - src_max));
                    next_targets
                        .insert((dest_min + target_min - src_min, src_max - target_min + 1));
                }
            }
        }
        targets = next_targets.union(&targets).copied().collect();
    }
    targets
        .iter()
        .map(|t| t.0)
        .min()
        .expect("No minimum found")
        .print();
}
