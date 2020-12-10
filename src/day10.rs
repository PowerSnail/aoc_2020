// Copyright (c) 2020 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::util::lines_as;

fn count_diff(numbers: &[i64], target: i64) -> i64 {
    match numbers {
        &[] | &[_] => 0,
        &[x, y, ..] => (y - x == target) as i64 + count_diff(&numbers[1..], target),
    }
}

fn count_valid_arrangement(numbers: &[i64], mem: &mut [i64]) -> i64 {
    if mem[numbers.len()] != -1 {
        return mem[numbers.len()];
    }
    let result = match numbers {
        &[] | &[_] | &[_, _] => 1,
        &[x, _, _, y, ..] if y - x <= 3 => {
            count_valid_arrangement(&numbers[1..], mem)
                + count_valid_arrangement(&numbers[2..], mem)
                + count_valid_arrangement(&numbers[3..], mem)
        }
        &[x, _, y, ..] if y - x <= 3 => {
            count_valid_arrangement(&numbers[1..], mem)
                + count_valid_arrangement(&numbers[2..], mem)
        }
        _ => count_valid_arrangement(&numbers[1..], mem),
    };
    mem[numbers.len()] = result;
    result
}

pub fn part1() -> Option<i64> {
    let mut numbers: Vec<i64> = lines_as();
    numbers.push(0);
    numbers.sort();
    numbers.push(numbers.last().unwrap() + 3);
    Some(count_diff(&numbers, 1) * count_diff(&numbers, 3))
}

pub fn part2() -> Option<i64> {
    let mut numbers: Vec<i64> = lines_as();
    numbers.push(0);
    numbers.sort();
    numbers.push(numbers.last().unwrap() + 3);

    Some(count_valid_arrangement(
        &numbers,
        &mut vec![-1; numbers.len() + 1],
    ))
}
