// Copyright (c) 2020 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::util::lines;

fn binary_go<I>(lo: i64, hi: i64, mut instruction: I) -> i64
where
    I: Iterator<Item = char>,
{
    if lo == hi {
        return lo;
    }
    let half = (hi + lo) / 2;
    match instruction.next() {
        Some('F') | Some('L') => binary_go(lo, half, instruction),
        Some('B') | Some('R') => binary_go(half + 1, hi, instruction),
        _ => unreachable!(),
    }
}

fn get_id(line: String) -> i64 {
    let row = binary_go(0, 127, line.chars());
    let column = binary_go(0, 7, line.chars().skip(7));
    return row * 8 + column;
}

pub fn part1() -> Option<i64> {
    lines().into_iter().map(get_id).max()
}

pub fn part2() -> Option<i64> {
    let mut ids: Vec<i64> = lines().into_iter().map(get_id).collect();
    ids.sort();
    for i in 0..ids.len() - 1 {
        if ids[i] + 1 != ids[i + 1] {
            return Some(ids[i] + 1)
        }
    }
    None
}