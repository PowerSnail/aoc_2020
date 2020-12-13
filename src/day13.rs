// Copyright (c) 2020 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use crate::util::lines;

pub fn part1() -> Option<i64> {
    let input = lines();
    let t0: i64 = input[0].parse().unwrap();
    let (b_id, waited_minute) = input[1]
        .split(',')
        .filter(|&token| token != "x")
        .map(|s| s.parse::<i64>().unwrap())
        .map(|n| (((t0 - 1) / n + 1) * n - t0, n))
        .min()
        .unwrap();
    Some(b_id * waited_minute)
}

fn extended_euclidean(t: i64, t_next: i64, r: i64, r_next: i64) -> i64 {
    match r_next {
        0 => t,
        _ => {
            let q = r / r_next;
            extended_euclidean(t_next, t - q * t_next, r_next, r - q * r_next)
        }
    }
}

fn mod_inverse(x: i64, m: i64) -> i64 {
    // Assumes that x and m are coprime
    (extended_euclidean(0, 1, m, x) + m) % m
}

pub fn part2() -> Option<i64> {
    let (remainders, mods): (Vec<i64>, Vec<i64>) = lines()[1]
        .split(',')
        .enumerate()
        .filter(|&(_, s)| s != "x")
        .map(|(i, n)| (i as i64, n.parse::<i64>().unwrap()))
        .map(|(i, n)| (n - i, n))
        .unzip();

    let product = mods.iter().fold(1, |a, &b| a * b);

    let sum: i64 = mods
        .iter()
        .zip(remainders.iter())
        .map(|(&m, &r)| {
            let n = product / m;
            r * mod_inverse(n, m) * n
        })
        .sum();

    Some((sum % product) as i64)
}
