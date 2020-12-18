// Copyright (c) 2020 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::util::lines;
use itertools::Itertools;
use std::collections::HashSet;

struct Cube {
    alive: HashSet<(i64, i64, i64)>,
    x_lo: i64,
    x_hi: i64,
    y_lo: i64,
    y_hi: i64,
    z_lo: i64,
    z_hi: i64,
}

impl Cube {
    pub fn new(x_lo: i64, x_hi: i64, y_lo: i64, y_hi: i64, z_lo: i64, z_hi: i64) -> Cube {
        Cube {
            alive: HashSet::new(),
            x_lo,
            x_hi,
            y_lo,
            y_hi,
            z_lo,
            z_hi,
        }
    }

    pub fn load(strings: &[String]) -> Cube {
        let mut alive = HashSet::new();
        for (x, row) in strings.iter().enumerate() {
            for (y, c) in row.chars().enumerate() {
                if c == '#' {
                    alive.insert((x as i64, y as i64, 0));
                }
            }
        }

        Cube {
            alive,
            x_lo: 0,
            x_hi: strings.len() as i64,
            y_lo: 0,
            y_hi: strings[0].len() as i64,
            z_lo: 0,
            z_hi: 1,
        }
    }
}

macro_rules! product {
    ($e:expr) => {
        $e
    };
    ($e:expr, $($es:expr),+) => {
        $e.cartesian_product(product!($($es),+))
    };
}

fn neighbors(p: (i64, i64, i64)) -> [(i64, i64, i64); 26] {
    [
        // (p.0 + 0, p.1 + 0, p.2 + 0),
        (p.0 + 0, p.1 + 0, p.2 - 1),
        (p.0 + 0, p.1 + 0, p.2 + 1),
        (p.0 + 0, p.1 + 1, p.2 + 0),
        (p.0 + 0, p.1 + 1, p.2 - 1),
        (p.0 + 0, p.1 + 1, p.2 + 1),
        (p.0 + 0, p.1 - 1, p.2 + 0),
        (p.0 + 0, p.1 - 1, p.2 - 1),
        (p.0 + 0, p.1 - 1, p.2 + 1),
        (p.0 + 1, p.1 + 0, p.2 + 0),
        (p.0 + 1, p.1 + 0, p.2 - 1),
        (p.0 + 1, p.1 + 0, p.2 + 1),
        (p.0 + 1, p.1 + 1, p.2 + 0),
        (p.0 + 1, p.1 + 1, p.2 - 1),
        (p.0 + 1, p.1 + 1, p.2 + 1),
        (p.0 + 1, p.1 - 1, p.2 + 0),
        (p.0 + 1, p.1 - 1, p.2 - 1),
        (p.0 + 1, p.1 - 1, p.2 + 1),
        (p.0 - 1, p.1 + 0, p.2 + 0),
        (p.0 - 1, p.1 + 0, p.2 - 1),
        (p.0 - 1, p.1 + 0, p.2 + 1),
        (p.0 - 1, p.1 + 1, p.2 + 0),
        (p.0 - 1, p.1 + 1, p.2 - 1),
        (p.0 - 1, p.1 + 1, p.2 + 1),
        (p.0 - 1, p.1 - 1, p.2 + 0),
        (p.0 - 1, p.1 - 1, p.2 - 1),
        (p.0 - 1, p.1 - 1, p.2 + 1),
    ]
}

pub fn part1() -> Option<i64> {
    // - If a cube is *active* and *exactly `2` or `3`* of its neighbors are also active, the cube remains *active*. Otherwise, the cube becomes *inactive*.
    // - If a cube is *inactive* but *exactly `3`* of its neighbors are active, the cube becomes *active*. Otherwise, the cube remains *inactive*.
    let mut cube = Cube::load(&lines());

    for _ in 0..6 {
        let mut new_cube = Cube::new(
            cube.x_lo - 1,
            cube.x_hi + 1,
            cube.y_lo - 1,
            cube.y_hi + 1,
            cube.z_lo - 1,
            cube.z_hi + 1,
        );

        for x in new_cube.x_lo..new_cube.x_hi {
            for y in new_cube.y_lo..new_cube.y_hi {
                for z in new_cube.z_lo..new_cube.z_hi {
                    let alive_peers = neighbors((x, y, z))
                        .iter()
                        .filter(|&p| cube.alive.contains(p))
                        .count();
                    match (cube.alive.contains(&(x, y, z)), alive_peers) {
                        (true, 2) | (true, 3) | (false, 3) => {
                            new_cube.alive.insert((x, y, z));
                        }
                        _ => (),
                    }
                }
            }
        }

        cube = new_cube;
    }
    Some(cube.alive.len() as i64)
}

pub fn part2() -> Option<i64> {
    let n_cycle = 6;
    let strings = lines();
    let range = [
        strings.len() + (n_cycle + 1) * 2,
        strings[0].len() + (n_cycle + 1) * 2,
        1 + (n_cycle + 1) * 2,
        1 + (n_cycle + 1) * 2,
    ];
    let mut alive = vec![vec![vec![vec![false; range[3]]; range[2]]; range[1]]; range[0]];

    for (x, row) in strings.iter().enumerate() {
        for (y, c) in row.chars().enumerate() {
            if c == '#' {
                alive[x + n_cycle + 1][y + n_cycle + 1][n_cycle + 1][n_cycle + 1] = true;
            }
        }
    }

    for _ in 0..n_cycle {
        let mut alive_counts = vec![vec![vec![vec![0; range[3]]; range[2]]; range[1]]; range[0]];
        for (p1, (p2, (p3, p4))) in product!(
            (1..range[0] - 1),
            (1..range[1] - 1),
            (1..range[2] - 1),
            (1..range[3] - 1)
        )
        .filter(|&(p1, (p2, (p3, p4)))| alive[p1][p2][p3][p4])
        {
            for (n1, (n2, (n3, n4))) in product!(
                (p1 - 1..p1 + 2),
                (p2 - 1..p2 + 2),
                (p3 - 1..p3 + 2),
                (p4 - 1..p4 + 2)
            )
            .filter(|&(n1, (n2, (n3, n4)))| !(n1 == p1 && n2 == p2 && n3 == p3 && n4 == p4))
            {
                alive_counts[n1][n2][n3][n4] += 1;
            }
        }

        for (p1, (p2, (p3, p4))) in product!(
            (1..range[0] - 1),
            (1..range[1] - 1),
            (1..range[2] - 1),
            (1..range[3] - 1)
        ) {
            alive[p1][p2][p3][p4] = match (alive[p1][p2][p3][p4], alive_counts[p1][p2][p3][p4]) {
                (true, 2) | (true, 3) | (false, 3) => true,
                _ => false,
            };
        }
    }

    Some(
        alive
            .iter()
            .flatten()
            .flatten()
            .flatten()
            .filter(|&&x| x)
            .count() as i64,
    )
}
