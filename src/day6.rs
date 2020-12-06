// Copyright (c) 2020 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::util::lines;

fn count_set_bits_u32(n: u32) -> u32 {
    let mut c = n;
    c = (c & 0x55555555) + ((c & 0xAAAAAAAA) >> 1);
    c = (c & 0x33333333) + ((c & 0xCCCCCCCC) >> 2);
    c = (c & 0x0F0F0F0F) + ((c & 0xF0F0F0F0) >> 4);
    c = (c & 0x00FF00FF) + ((c & 0xFF00FF00) >> 8);
    c = (c & 0x0000FFFF) + ((c & 0xFFFF0000) >> 16);
    c
}

fn set_of_yeses(string: &std::string::String) -> u32 {
    string
        .as_bytes()
        .iter()
        .map(|c| 1u32 << (c - 'a' as u8))
        .fold(0, |x, y| x | y)
}

fn set_merge(s1: u32, s2: u32) -> u32 {
    s1 | s2
}

pub fn part1() -> Option<i64> {
    let answer: u32 = lines()
        .split(|l| l == "")
        .map(|group| {
            count_set_bits_u32(
                group // Count `Yes`es
                    .iter()
                    .map(set_of_yeses)
                    .fold(0, set_merge),
            )
        })
        .sum();
    Some(answer as i64)
}

pub fn part2() -> Option<i64> {
    let answer: u32 = lines()
        .split(|l| l == "")
        .map(|group| {
            count_set_bits_u32(
                !group // Count `No`s
                    .iter()
                    .map(set_of_yeses)
                    .map(|c| !c)
                    .fold(0, set_merge),
            )
        })
        .sum();
    Some(answer as i64)
}
