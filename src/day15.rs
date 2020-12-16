// Copyright (c) 2020 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use crate::util::lines;

fn game(upper_limit: usize) -> i64 {
    let starting: Vec<_> = lines()[0]
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let mut mem = vec![-1; upper_limit];
    for (i, &v) in starting.iter().enumerate() {
        mem[v as usize] = i as i64;
    }

    let mut prev = *starting.last().unwrap();
    for turn in starting.len()..upper_limit {
        let spoken = match mem[prev as usize] {
            -1 => 0,
            n => turn as i64 - n - 1,
        };
        mem[prev as usize] = turn as i64 - 1;
        prev = spoken;
    }
    prev
}

pub fn part1() -> Option<i64> {
    Some(game(2020))
}

pub fn part2() -> Option<i64> {
    Some(game(30000000))
}
