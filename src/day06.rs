use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

pub fn part1(input: String) {
    let mut lines = input.lines();
    let times = parse_nums::<u64>(lines.next().expect("First line not found"));
    let distances = parse_nums::<u64>(lines.next().expect("Second line not found"));
    times
        .zip(distances)
        .map(|(time, record)| {
            let mut count = 0;
            for i in 1..time {
                if i * (time - i) > record {
                    count += 1;
                }
            }
            count
        })
        .product::<u64>()
        .print();
}

pub fn part2(input: String) {
    let mut lines = input.lines();
    let time: u64 = lines
        .next()
        .expect("First line not found")
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap_or_default();
    let distance: u64 = lines
        .next()
        .expect("Second line not found")
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap_or_default();
    let mut count = 0;
    for i in 1..time {
        if i * (time - i) > distance {
            count += 1;
        }
    }
    println!("{}", count);
}
