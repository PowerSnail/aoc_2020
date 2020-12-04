// Copyright (c) 2020 PowerSnail
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::util::lines_as;

pub fn part1() -> Option<i64> {
    let mut numbers: Vec<i64> = lines_as();
    numbers.sort();

    let mut lo: usize = 0;
    let mut hi = numbers.len() - 1;

    while hi > lo {
        if numbers[lo] + numbers[hi] == 2020 {
            return Some(numbers[lo] * numbers[hi]);
        }
        if numbers[lo] + numbers[hi] > 2020 {
            hi -= 1;
        } else {
            lo += 1;
        }
    }
    None
}

pub fn part2() -> Option<i64> {
    let mut numbers: Vec<i64> = lines_as();
    numbers.sort();

    for lo in 0..numbers.len() - 1 {
        for hi in (lo..numbers.len()).rev() {
            let remain = 2020 - numbers[lo] - numbers[hi];
            match numbers.binary_search(&remain) {
                Ok(_) => return Some(remain * numbers[lo] * numbers[hi]),
                _ => continue,
            }
        }
    }
    None
}
