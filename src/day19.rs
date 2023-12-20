use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

#[derive(Debug)]
enum Expression<'a> {
    Condition(usize, bool, u64, &'a str), // index, is_greater, target, destination
    Destination(&'a str),
}

fn parse_workflow(line: &str) -> Vec<Expression> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"((\w+)([<>])(\d+):(\w+))|(\w+)").expect("Invalid Regex");
    }

    RE.captures_iter(line)
        .map(|c| {
            if let Some(destination) = c.get(6) {
                return Expression::Destination(destination.as_str());
            } else {
                let (_, _, index, symbol, target, destination) = c
                    .iter()
                    .filter_map(|v| v)
                    .collect_tuple()
                    .expect("Invalid capture");
                return Expression::Condition(
                    ["x", "m", "a", "s"]
                        .iter()
                        .position(|x| x == &index.as_str())
                        .expect("Index not found"),
                    symbol.as_str() == ">",
                    target.as_str().parse().unwrap_or_default(),
                    destination.as_str(),
                );
            }
        })
        .collect()
}

fn is_accepted(values: &Vec<u64>, from: &str, workflows: &HashMap<&str, Vec<Expression>>) -> bool {
    if from == "A" {
        return true;
    } else if from == "R" {
        return false;
    }

    let workflow = workflows.get(from).expect("Workflow not found");
    for expression in workflow {
        match expression {
            Expression::Condition(index, is_greater, target, destination) => {
                let value = values.get(*index).expect("Invalid index found");
                if *is_greater && value > target {
                    return is_accepted(values, destination, workflows);
                } else if !*is_greater && value < target {
                    return is_accepted(values, destination, workflows);
                }
            }
            Expression::Destination(destination) => {
                return is_accepted(values, *destination, workflows);
            }
        }
    }
    true
}

pub fn part1(input: String) {
    let (workflow_lines, part_lines) = input.split_once("\n\n").expect("No separation found!");
    let workflows = workflow_lines
        .lines()
        .fold(HashMap::new(), |mut map, line| {
            let (name, line) = line.split_once('{').expect("No { found?!");
            map.insert(name, parse_workflow(line));
            map
        });

    part_lines
        .lines()
        .map(|line| {
            let values = parse_nums(line).collect_vec();
            if is_accepted(&values, "in", &workflows) {
                values.iter().sum::<u64>()
            } else {
                0
            }
        })
        .sum::<u64>()
        .print();

    // Of course I tried to brute force Part 2 :trollol:
    // iproduct!(1..=4000, 1..=4000, 1..=4000, 1..=4000)
    //     .filter(|(x, m, a, s)| is_accepted(&vec![*x, *m, *a, *s], "in", &workflows))
    //     .count()
    //     .print();
}

fn acceptable_count(
    ranges: &[HashSet<u64>; 4],
    from: &str,
    workflows: &HashMap<&str, Vec<Expression>>,
) -> usize {
    if from == "A" {
        return ranges.iter().map(|r| r.len()).product::<usize>();
    } else if from == "R" {
        return 0;
    }

    let mut count = 0;

    let mut current_ranges = ranges.clone();

    let workflow = workflows.get(from).expect("No workflow found");
    for expression in workflow {
        match expression {
            Expression::Condition(index, is_greater, target, destination) => {
                let matching_range = current_ranges[*index]
                    .iter()
                    .filter(|v| {
                        if *is_greater {
                            v > &target
                        } else {
                            v < &target
                        }
                    })
                    .copied()
                    .collect::<HashSet<_>>();

                let mut matching_ranges = current_ranges.clone();
                matching_ranges[*index] = matching_range.clone();
                count += acceptable_count(&matching_ranges, *destination, workflows);

                current_ranges[*index] = current_ranges[*index]
                    .difference(&matching_range)
                    .copied()
                    .collect();
            }
            Expression::Destination(destination) => {
                count += acceptable_count(&current_ranges, *destination, workflows);
            }
        }
    }

    count
}

pub fn part2(input: String) {
    let (workflow_lines, _) = input.split_once("\n\n").expect("No separation found!");
    let workflows = workflow_lines
        .lines()
        .fold(HashMap::new(), |mut map, line| {
            let (name, line) = line.split_once('{').expect("No { found?!");
            map.insert(name, parse_workflow(line));
            map
        });

    let all_possible = (1..=4000).collect::<HashSet<u64>>();
    acceptable_count(
        &[
            all_possible.clone(),
            all_possible.clone(),
            all_possible.clone(),
            all_possible,
        ],
        "in",
        &workflows,
    )
    .print();
}
