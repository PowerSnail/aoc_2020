// Copyright (c) 2020 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::util::lines;

const FLOOR: u8 = '.' as u8;
const EMPTY: u8 = 'L' as u8;
const OCCUPIED: u8 = '#' as u8;

fn init_seating(input: &Vec<String>) -> Vec<Vec<u8>> {
    let width = input[0].len();
    let height = input.len();

    let mut seating = vec![vec![EMPTY; width + 2]; height + 2];
    for (i, line) in input.into_iter().enumerate() {
        seating[i + 1][1..width + 1].copy_from_slice(line.as_bytes());
    }

    seating
}

fn count_occupied(x: usize, y: usize, seating: &Vec<Vec<u8>>) -> usize {
    seating[y - 1..y + 2]
        .iter()
        .map(|row| row[x - 1..x + 2].iter().filter(|&&n| n == OCCUPIED).count())
        .sum()
}

pub fn part1() -> Option<i64> {
    let mut seating = init_seating(&lines());
    let mut new = seating.clone();

    let mut has_change = true;

    while has_change {
        has_change = false;
        for y in 1..seating.len() {
            for x in 1..seating[0].len() - 1 {
                new[y][x] = match seating[y][x] {
                    OCCUPIED if count_occupied(x, y, &seating) >= 5 => {
                        has_change = true;
                        EMPTY
                    }
                    EMPTY if count_occupied(x, y, &seating) == 0 => {
                        has_change = true;
                        OCCUPIED
                    }
                    other => other,
                }
            }
        }
        std::mem::swap(&mut seating, &mut new);
    }
    Some(
        seating
            .iter()
            .map(|row| row.iter().filter(|&&n| n == OCCUPIED).count() as i64)
            .sum(),
    )
}

const DIRECTION: &[(i64, i64)] = &[
    (1, 0),
    (1, 1),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn search(x: i64, y: i64, dx: i64, dy: i64, seating: &Vec<Vec<u8>>) -> i64 {
    if x == 0 || x == seating[0].len() as i64 - 1 || y == 0 || y == seating.len() as i64 - 1 {
        return 0;
    }
    match seating[y as usize][x as usize] {
        FLOOR => search(x + dx, y + dy, dx, dy, seating),
        OCCUPIED => 1,
        EMPTY => 0,
        _ => unreachable!(),
    }
}

fn count_seen(x: usize, y: usize, seating: &Vec<Vec<u8>>) -> i64 {
    DIRECTION
        .iter()
        .map(|&(dx, dy)| search(x as i64 + dx, y as i64 + dy, dx, dy, seating))
        .sum()
}

pub fn part2() -> Option<i64> {
    let mut seating = init_seating(&lines());
    let mut new = seating.clone();

    let mut has_change = true;

    while has_change {
        has_change = false;
        for y in 1..seating.len() - 1 {
            for x in (1..seating[y].len() - 1).filter(|&x| seating[y][x] != FLOOR) {
                new[y][x] = match seating[y][x] {
                    OCCUPIED if count_seen(x, y, &seating) >= 5 => {
                        has_change = true;
                        EMPTY
                    }
                    EMPTY if count_seen(x, y, &seating) == 0 => {
                        has_change = true;
                        OCCUPIED
                    }
                    other => other,
                }
            }
        }

        std::mem::swap(&mut seating, &mut new);
    }
    Some(
        seating
            .iter()
            .map(|row| row.iter().filter(|&&n| n == OCCUPIED).count() as i64)
            .sum(),
    )
}
