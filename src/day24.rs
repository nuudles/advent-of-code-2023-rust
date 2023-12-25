use std::collections::HashSet;

use itertools::Itertools;

use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

fn two_d_intersection(
    a: &(i64, i64, i64, i64, i64, i64),
    b: &(i64, i64, i64, i64, i64, i64),
) -> Option<(f64, f64)> {
    let (x1, y1, _, dx1, dy1, _) = *a;
    let (x2, y2, _, dx2, dy2, _) = *b;
    /*
    a = 1
    b = -(dx / dy)
    c = x1 - (dx / dy) * y1
    */

    let (a1, b1, c1) = (
        1f64,
        -dx1 as f64 / dy1 as f64,
        x1 as f64 - (dx1 as f64 / dy1 as f64) * y1 as f64,
    );
    let (a2, b2, c2) = (
        1f64,
        -dx2 as f64 / dy2 as f64,
        x2 as f64 - (dx2 as f64 / dy2 as f64) * y2 as f64,
    );
    if a1 * b2 - a2 * b1 == 0f64 {
        // Parallel lines
        None
    } else {
        let (x, y) = (
            (c1 * b2 - b1 * c2) / (a1 * b2 - b1 * a2),
            (a1 * c2 - c1 * a1) / (a1 * b2 - b1 * a2),
        );
        // x = x1 + dx * t
        // t = (x - x1) / dx
        let t1 = (x - x1 as f64) / dx1 as f64;
        let t2 = (x - x2 as f64) / dx2 as f64;
        if t1 < 0f64 || t2 < 0f64 {
            None
        } else {
            Some((x, y))
        }
    }
}

pub fn part1(input: String) {
    let hailstones: HashSet<(i64, i64, i64, i64, i64, i64)> = input
        .lines()
        .map(|line| parse_nums(line).collect_tuple().expect("Parsing error"))
        .collect();
    // let (min, max) = (7f64, 27f64);
    let (min, max) = (200000000000000f64, 400000000000000f64);
    hailstones
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| {
            if let Some((x, y)) = two_d_intersection(*a, *b) {
                if x >= min && x <= max && y >= min && y <= max {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        })
        .count()
        .print();
}

pub fn part2(input: String) {
    let hailstones: HashSet<(i64, i64, i64, i64, i64, i64)> = input
        .lines()
        .map(|line| parse_nums(line).collect_tuple().expect("Parsing error"))
        .collect();

    for (
        (x1, y1, z1, dx1, dy1, dz1),
        (x2, y2, z2, dx2, dy2, dz2),
        (x3, y3, z3, dx3, dy3, dz3),
        (x4, y4, z4, dx4, dy4, dz4),
    ) in hailstones.iter().tuple_combinations().take(1)
    {
        /*
        x1 + dx1 * t1 = rx + rdx * t1
        y1 + dy1 * t1 = ry + rdy * t1
        x2 + dx2 * t2 = rx + rdx * t2
        y2 + dy2 * t2 = ry + rdy * t2
         */
        println!(
            "{} {} {} * a = x + t * a",
            x1,
            if dx1 > &0 { '+' } else { '-' },
            dx1.abs()
        );
        println!(
            "{} {} {} * a = y + u * a",
            y1,
            if dy1 > &0 { '+' } else { '-' },
            dy1.abs()
        );
        println!(
            "{} {} {} * a = z + v * a",
            z1,
            if dz1 > &0 { '+' } else { '-' },
            dz1.abs()
        );
        println!(
            "{} {} {} * b = x + t * b",
            x2,
            if dx2 > &0 { '+' } else { '-' },
            dx2.abs()
        );
        println!(
            "{} {} {} * b = y + u * b",
            y2,
            if dy2 > &0 { '+' } else { '-' },
            dy2.abs()
        );
        println!(
            "{} {} {} * b = z + v * b",
            z2,
            if dz2 > &0 { '+' } else { '-' },
            dz2.abs()
        );
        println!(
            "{} {} {} * c = x + t * c",
            x3,
            if dx3 > &0 { '+' } else { '-' },
            dx3.abs()
        );
        println!(
            "{} {} {} * c = y + u * c",
            y3,
            if dy3 > &0 { '+' } else { '-' },
            dy3.abs()
        );
        println!(
            "{} {} {} * c = z + v * c",
            z3,
            if dz3 > &0 { '+' } else { '-' },
            dz3.abs()
        );
        println!(
            "{} {} {} * d = x + t * d",
            x4,
            if dx4 > &0 { '+' } else { '-' },
            dx4.abs()
        );
        println!(
            "{} {} {} * d = y + u * d",
            y4,
            if dy4 > &0 { '+' } else { '-' },
            dy4.abs()
        );
        println!(
            "{} {} {} * d = z + v * d",
            z4,
            if dz4 > &0 { '+' } else { '-' },
            dz4.abs()
        );
    }
    /*
    I used the print statements above to print the systems of equations, then I used an online systems
    of equations solver to solve for a, b, c, d, x, y, z, t, u, and v:
    https://quickmath.com/webMathematica3/quickmath/equations/solve/advanced.jsp

    Then, I added x + y + z to get the solution
     */
}
