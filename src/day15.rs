use std::{collections::HashMap, iter::repeat};

use itertools::Itertools;

use crate::selfprint::SelfPrint;

fn hash(string: &str) -> usize {
    let mut value = 0;
    for c in string.chars() {
        value = (value + u64::from(c) as usize) * 17 % 256
    }
    value
}

pub fn part1(input: String) {
    input.split(',').map(hash).sum::<usize>().print();
}

pub fn part2(input: String) {
    let mut boxes = repeat((Vec::<&str>::new(), HashMap::<&str, usize>::new()))
        .take(256)
        .collect_vec();
    for step in input.split(',') {
        if let Some((label, focal_length)) = step.split_once('=') {
            let index = hash(label);
            let (vec, map) = boxes.get_mut(index).expect("Box not found");
            if !vec.contains(&label) {
                vec.push(label);
            }
            map.insert(label, focal_length.parse().unwrap_or_default());
        } else {
            let label = &step[..step.len() - 1];
            let index = hash(label);
            let (vec, map) = boxes.get_mut(index).expect("Box not found");
            if let Some(lens_index) = vec.iter().position(|s| *s == label) {
                vec.remove(lens_index);
            }
            map.remove(label);
        }
    }
    boxes
        .iter()
        .enumerate()
        .map(|(index, (vec, map))| {
            map.iter()
                .map(|(label, focal_length)| {
                    *focal_length
                        * (index + 1)
                        * (vec
                            .iter()
                            .position(|s| s == label)
                            .expect("Label not found in vec")
                            + 1)
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        .print();
}
