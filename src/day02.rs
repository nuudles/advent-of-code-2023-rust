use itertools::Itertools;

use crate::selfprint::SelfPrint;

fn parse_game(line: &str) -> Vec<(u64, u64, u64)> {
    let (_, game) = line.split_once(": ").expect("Could not find :");
    game.split("; ")
        .map(|round| {
            let mut rgb = (0, 0, 0);
            for part in round.split(", ") {
                let (count, color) = part
                    .split_once(" ")
                    .expect("Could not find a space in the part");
                match color {
                    "red" => rgb.0 = count.parse().unwrap_or_default(),
                    "green" => rgb.1 = count.parse().unwrap_or_default(),
                    "blue" => rgb.2 = count.parse().unwrap_or_default(),
                    _ => (),
                }
            }
            rgb
        })
        .collect_vec()
}

pub fn part1(input: String) {
    input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let game = parse_game(line);
            if game.iter().any(|(r, g, b)| *r > 12 || *g > 13 || *b > 14) {
                0
            } else {
                index + 1
            }
        })
        .sum::<usize>()
        .print();
}

pub fn part2(input: String) {
    input
        .lines()
        .map(|line| {
            let game = parse_game(line);
            let mut min_rgb = (0, 0, 0);

            for (r, g, b) in game {
                min_rgb.0 = min_rgb.0.max(r);
                min_rgb.1 = min_rgb.1.max(g);
                min_rgb.2 = min_rgb.2.max(b);
            }

            min_rgb.0 * min_rgb.1 * min_rgb.2
        })
        .sum::<u64>()
        .print();
}
