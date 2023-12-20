use std::collections::{BTreeMap, BTreeSet};

use gcd::Gcd;
use itertools::Itertools;

use crate::selfprint::SelfPrint;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Module<'a> {
    Broadcaster,
    FlipFlop(bool),                        // is_on
    Conjunction(BTreeMap<&'a str, Pulse>), // memory
    Output,
}

pub fn part1(input: String) {
    let mut mappings = BTreeMap::<&str, BTreeSet<&str>>::new();
    let mut modules = input.lines().fold(BTreeMap::new(), |mut map, line| {
        let (left, right) = line.split_once(" -> ").expect("Arrow not found");
        let destinations = right.split(", ").collect_vec();
        if left == "broadcaster" {
            map.insert(left, (Module::Broadcaster, destinations.clone()));
            for destination in destinations {
                mappings.entry(destination).or_default().insert(left);
            }
        } else {
            map.insert(
                &left[1..],
                (
                    match &left[0..1] {
                        "%" => Module::FlipFlop(false),
                        "&" => Module::Conjunction(BTreeMap::new()),
                        _ => panic!("Unknown module type!"),
                    },
                    destinations.clone(),
                ),
            );
            for destination in destinations {
                mappings.entry(destination).or_default().insert(&left[1..]);
            }
        }
        map
    });

    for (name, (module, _)) in modules.iter_mut() {
        match module {
            Module::Conjunction(memory) => {
                for source in mappings.get(*name).expect("Mapping not found") {
                    memory.insert(*source, Pulse::Low);
                }
            }
            _ => (),
        }
    }

    modules.insert("output", (Module::Output, vec![]));
    modules.insert("rx", (Module::Output, vec![]));

    let rx_dependencies = mappings.get("hf").expect("Dependency not found");
    let mut last_times = BTreeMap::new();
    let mut cycle_counts = BTreeMap::new();

    let (mut low_count, mut high_count) = (0u64, 0u64);
    for i in 0u64..10000 {
        let mut pulses: Vec<(Pulse, &str, &str)> = vec![(Pulse::Low, "button", "broadcaster")];
        while !pulses.is_empty() {
            let mut new_pulses = Vec::new();
            for (pulse, source, target) in pulses {
                let (module, destinations) = modules
                    .get_mut(target)
                    .expect(format!("Module not found: {}", target).as_str());
                match module {
                    Module::Broadcaster => {
                        for destination in destinations {
                            new_pulses.push((pulse, target, *destination));
                        }
                    }
                    Module::FlipFlop(is_on) => {
                        if pulse == Pulse::Low {
                            if *is_on {
                                for destination in destinations {
                                    new_pulses.push((Pulse::Low, target, *destination));
                                }
                                *is_on = false;
                            } else {
                                for destination in destinations {
                                    new_pulses.push((Pulse::High, target, *destination));
                                }
                                *is_on = true;
                            }
                        }
                    }
                    Module::Conjunction(memory) => {
                        memory.insert(source, pulse);
                        for destination in destinations {
                            new_pulses.push((
                                if memory.values().all(|p| p == &Pulse::High) {
                                    Pulse::Low
                                } else {
                                    Pulse::High
                                },
                                target,
                                destination,
                            ));
                        }
                    }
                    Module::Output => (),
                }

                if pulse == Pulse::Low {
                    low_count += 1;
                } else {
                    high_count += 1;
                }
            }

            pulses = new_pulses;
        }

        for key in rx_dependencies {
            let source = mappings
                .get(*key)
                .expect("Source not found")
                .first()
                .expect("First source not found");
            let sources = mappings.get(*source).expect("Sources not found");
            if let Some((Module::Conjunction(memory), _)) = modules.get(*source) {
                for x in sources {
                    if cycle_counts.contains_key(x) {
                        continue;
                    }
                    if memory.get(x).unwrap() == &Pulse::High {
                        if let Some(last_time) = last_times.get(*x) {
                            if i - last_time > 1 {
                                cycle_counts.insert(*x, i - last_time - 1);
                            }
                        }
                        last_times.insert(*x, i);
                    }
                }
            }
        }
    }
    println!("Part 1: {}", low_count * high_count);

    print!("Part 2: ");
    rx_dependencies
        .iter()
        .fold(1, |result, dep| {
            let source = mappings
                .get(*dep)
                .expect("Source not found")
                .first()
                .expect("First source not found");
            let sources = mappings.get(*source).expect("Sources not found");
            let sum = sources
                .iter()
                .map(|s| cycle_counts.get(*s).copied().unwrap_or_default())
                .sum::<u64>();
            lcm(result, sum)
        })
        .print();
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / a.gcd(b)
}
