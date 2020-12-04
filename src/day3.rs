// Copyright (c) 2020 PowerSnail
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::util::lines;

pub fn part1() -> Option<i64> {
    let mut x: usize = 3;
    let mut tree_count = 0;
    for line in lines().into_iter().skip(1) {
        let line = line.as_bytes();
        let width = line.len();
        let x_ = x % width;
        if line[x_] == '#' as u8 {
            tree_count += 1;
        }
        x += 3;
    }
    Some(tree_count)
}

pub fn part2() -> Option<i64> {
    let rights = vec![1, 3, 5, 7, 1];
    let downs = vec![1, 1, 1, 1, 2];
    let mut tree_counts = vec![0, 0, 0, 0, 0];

    for (row, line) in lines().into_iter().enumerate().skip(1) {
        let line = line.as_bytes();
        let width = line.len();

        for i in 0..5 {
            if row % downs[i] == 0 && line[rights[i] * row / downs[i] % width] == '#' as u8 {
                tree_counts[i] += 1;
            }
        }
    }
    println!("{:#?}", tree_counts);
    Some(tree_counts[0] * tree_counts[1] * tree_counts[2] * tree_counts[3] * tree_counts[4])
}
