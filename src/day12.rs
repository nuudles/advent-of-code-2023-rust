use std::collections::VecDeque;

use cached::proc_macro::cached;
use itertools::Itertools;
use regex::Regex;

use crate::selfprint::SelfPrint;

fn count_ones(string: &str) -> Vec<usize> {
    string
        .split('0')
        .filter(|s| s.len() > 0)
        .map(|s| s.chars().count())
        .collect_vec()
}

pub fn part1(input: String) {
    let regex = Regex::new(r"\?+").expect("Invalid Regex");

    input
        .lines()
        .flat_map(|line| {
            let (record, count_string) = line.split_once(' ').expect("No space found");
            let counts = count_string
                .split(',')
                .filter_map(|c| c.parse::<usize>().ok())
                .collect_vec();
            regex
                .find_iter(record)
                .map(|m| {
                    let count = m.len();
                    let range = m.range();
                    (0..1 << count)
                        .map(move |n| (range.clone(), format!("{n:0>width$b}", width = count)))
                })
                .multi_cartesian_product()
                .filter_map(move |possibilities| {
                    let mut new_record = record.replace(".", "0").replace("#", "1");
                    for (range, replacement) in possibilities {
                        new_record.replace_range(range, &replacement);
                    }
                    if count_ones(&new_record) == counts {
                        Some(new_record)
                    } else {
                        None
                    }
                })
        })
        .count()
        .print();
}

#[cached]
fn possibilities(record: String, counts: VecDeque<usize>) -> usize {
    if record.is_empty() {
        if !counts.is_empty() {
            return 0;
        } else {
            return 1;
        }
    }

    let mut chars = record.chars();
    let c = chars.next().expect("Record should not be empty");
    if c == '.' {
        return possibilities(chars.skip_while(|c| c == &'.').collect(), counts);
    } else if c == '#' {
        let mut new_counts = counts.clone();
        if let Some(count) = new_counts.pop_front() {
            // We must only the count number of # or ? characters
            for _ in 1..count {
                if let Some(c) = chars.next() {
                    if c == '.' {
                        return 0;
                    }
                } else {
                    return 0;
                }
            }
            if let Some(c) = chars.next() {
                // After the count, we have to have a space, so it must be a . or ?
                if c == '#' {
                    return 0;
                } else {
                    return possibilities(chars.collect(), new_counts);
                }
            } else {
                if new_counts.is_empty() {
                    return 1;
                } else {
                    return 0;
                }
            }
        } else {
            return 0;
        }
    } else {
        let mut with_dot = record.clone();
        with_dot.replace_range(0..1, ".");
        let mut with_hash = record.clone();
        with_hash.replace_range(0..1, "#");
        // println!("Trying {} {}", with_dot, with_hash);
        possibilities(with_dot, counts.clone()) + possibilities(with_hash, counts.clone())
    }
}

pub fn part2(input: String) {
    input
        .lines()
        .map(|line| {
            let (record, count_string) = line.split_once(' ').expect("No space found");

            let mut new_record = record.to_string();
            let mut new_count_string = count_string.to_string();
            for _ in 0..4 {
                new_record.push('?');
                new_record.push_str(record);
                new_count_string.push(',');
                new_count_string.push_str(count_string);
            }

            possibilities(
                new_record,
                new_count_string
                    .split(',')
                    .filter_map(|c| c.parse::<usize>().ok())
                    .collect(),
            )
        })
        .sum::<usize>()
        .print();
}

/*
pub fn part2(input: String) {
    let regex = Regex::new(r"\?+").expect("Invalid Regex");

    input
        .lines()
        .map(|line| {
            let (a, b) = (0..2)
                .map(|i| {
                    if i == 1 {
                        return 1;
                    }

                    let mut total_count = 0;
                    let (original_record, original_count_string) =
                        line.split_once(' ').expect("No space found");
                    let mut record = original_record.to_string();
                    let mut count_string = original_count_string.to_string();
                    for _ in 0..i {
                        record.push('?');
                        record.push_str(original_record);
                        count_string.push(',');
                        count_string.push_str(original_count_string);
                    }
                    // println!("{}", record);
                    // println!("{}", count_string);
                    let counts = count_string
                        .split(',')
                        .filter_map(|c| c.parse::<usize>().ok())
                        .collect_vec();
                    let products = regex
                        .find_iter(&record)
                        .map(|m| {
                            let count = m.len();
                            let range = m.range();
                            (0..1 << count).map(move |n| {
                                (range.clone(), format!("{n:0>width$b}", width = count))
                            })
                        })
                        .multi_cartesian_product();
                    for possibilities in products {
                        let mut new_record = record.replace(".", "0").replace("#", "1");
                        for (range, replacement) in possibilities {
                            new_record.replace_range(range, &replacement);
                        }
                        if count_ones(&new_record) == counts {
                            total_count += 1;
                        }
                    }
                    total_count
                })
                .collect_tuple()
                .expect("Could not find 2");
            // let multiplier = b / a;
            // a * multiplier * multiplier * multiplier * multiplier
            a.print()
        })
        .sum::<usize>()
        .print();
}
*/

/*
fn counts(line: &str, repetitions: usize) -> usize {
    let (orecord, ocount_string) = line.split_once(' ').expect("No space found");

    let mut record = orecord.to_string();
    let mut count_string = ocount_string.to_string();
    for _ in 0..repetitions {
        record.push('?');
        record.push_str(orecord);
        count_string.push(',');
        count_string.push_str(ocount_string);
    }
    println!("{:?}", record);

    let counts = count_string
        .split(',')
        .filter_map(|c| c.parse::<usize>().ok())
        .collect_vec();

    let mut possibilities = HashSet::<Vec<usize>>::new();
    let record_length = record.len();
    let length = counts.iter().sum::<usize>();
    let space_count = counts.len() - 1;
    repeat(0..record_length - length - space_count + 2)
        .take(space_count + 1)
        .multi_cartesian_product()
        .filter(|v| {
            v.iter().sum::<usize>() <= record_length - length
                && v.iter().skip(1).take(space_count).all(|n| n > &0)
        })
        .for_each(|v| {
            let mut iter = v.iter();
            let mut vec = vec![];
            let mut x = 0;
            for c in &counts {
                let space = iter.next().copied().expect("Space not found");
                vec.push(space);
                x += space;

                vec.push(*c);
                x += *c;
            }
            let space = record_length - x;
            vec.push(space);
            possibilities.insert(vec);
        });

    println!("{:?}", possibilities.len());

    if possibilities.is_empty() {
        return 1;
    }

    possibilities
        .iter()
        .filter(|vec| {
            let mut x = 0;
            let mut is_dot = true;
            for &c in *vec {
                if c > 0 {
                    if record[x..x + c]
                        .chars()
                        .any(|char| char != '?' && char != if is_dot { '.' } else { '#' })
                    {
                        return false;
                    }
                    x += c;
                }
                is_dot = !is_dot;
            }

            // let mut is_zero = true;
            // for &c in *vec {
            //     for _ in 0..c {
            //         print!("{}", if is_zero { '.' } else { '#' });
            //     }
            //     is_zero = !is_zero;
            // }
            // println!();

            true
        })
        .count()
}

pub fn part2(input: String) {
    // let regex = Regex::new(r"\?+").expect("Invalid Regex");

    input
        .lines()
        .map(|line| {
            let (a, b) = (counts(line, 0), counts(line, 1));
            println!("{} {} {}", line, a, b);
            let multiplier = b / a;
            a * multiplier * multiplier * multiplier * multiplier
            // counts(line, 0)
        })
        .sum::<usize>()
        .print();
}
 */
