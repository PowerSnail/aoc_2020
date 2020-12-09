// Copyright (c) 2020 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::BTreeSet;

use crate::util::lines;

const N: usize = 25;

fn can_be_added(set: &BTreeSet<i64>, target: i64) -> bool {
    for &n in set {
        let diff = target - n;
        if diff != n && set.contains(&diff) {
            return true;
        }
    }
    false
}

fn find_invalid(numbers: &Vec<i64>) -> Option<i64> {
    let mut set = BTreeSet::new();
    for &n in &numbers[..N] {
        set.insert(n);
    }
    for (i, &n) in numbers[N..].iter().enumerate() {
        if !can_be_added(&set, n) {
            return Some(n);
        }
        set.remove(&numbers[i]);
        set.insert(n);
    }
    None
}

pub fn part1() -> Option<i64> {
    let numbers: Vec<i64> = lines().into_iter().map(|l| l.parse().unwrap()).collect();
    find_invalid(&numbers)
}

pub fn part2() -> Option<i64> {
    let numbers: Vec<i64> = lines().into_iter().map(|l| l.parse().unwrap()).collect();
    let invalid = find_invalid(&numbers).unwrap();

    let mut sums = vec![vec![-1; numbers.len()]; numbers.len()];

    for i in 0..numbers.len() - 1 {
        sums[i][i] = numbers[i];
        for j in i + 1..numbers.len() {
            sums[i][j] = sums[i][j - 1] + numbers[j];
            if sums[i][j] == invalid {
                return Some(
                    numbers[i..j + 1].iter().min().unwrap()
                        + numbers[i..j + 1].iter().max().unwrap(),
                );
            }
        }
    }
    None
}
