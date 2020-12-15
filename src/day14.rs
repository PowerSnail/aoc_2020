// Copyright (c) 2020 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;

use regex::Regex;

use crate::util::lines;
use itertools::Itertools;

const THIRTY_SIX: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111;

struct Mask {
    floating: u64,
    value: u64,
}

impl Mask {
    pub fn new(floating: u64, value: u64) -> Mask {
        Mask { floating, value }
    }

    pub fn from_string(mask: &str) -> Mask {
        let mut floating: u64 = 0;
        let mut val: u64 = 0;
        for (i, c) in mask.chars().rev().enumerate() {
            match c {
                'X' => floating |= 1 << i,
                '1' => val |= 1 << i,
                _ => (),
            }
        }
        Mask::new(floating & THIRTY_SIX, val & THIRTY_SIX)
    }

    pub fn apply_to(&self, n: u64) -> u64 {
        n & self.floating | self.value & (!self.floating)
    }

    pub fn make_addr(&self, addr: u64) -> Vec<u64> {
        (0..36).filter(|&i| 1 & (self.floating >> i) == 1).fold(
            vec![addr & (!self.floating) | self.value],
            |bases, i| {
                bases
                    .into_iter()
                    .cartesian_product([0u64, 1u64 << i].iter())
                    .map(|(base, &floating)| base + floating)
                    .collect_vec()
            },
        )
    }
}

pub fn part1() -> Option<i64> {
    let re =
        Regex::new(r"^(mask = (?P<mask>[X10]+))|(mem\[(?P<addr>\d+)\] = (?P<value>\d+))$").unwrap();

    let mut mask = Mask::new(0, 0);
    let mut mem: HashMap<usize, u64> = HashMap::new();

    for line in lines() {
        let cap = re.captures(&line).unwrap();

        match (cap.name("mask"), cap.name("addr"), cap.name("value")) {
            (Some(m), None, None) => {
                mask = Mask::from_string(m.as_str());
            }
            (None, Some(addr), Some(value)) => {
                let addr: usize = addr.as_str().parse().unwrap();
                let value = value.as_str().parse().unwrap();
                mem.entry(addr).insert(mask.apply_to(value));
            }
            _ => (),
        }
    }
    Some(mem.values().sum::<u64>() as i64)
}

pub fn part2() -> Option<i64> {
    let re =
        Regex::new(r"^(mask = (?P<mask>[X10]+))|(mem\[(?P<addr>\d+)\] = (?P<value>\d+))$").unwrap();

    let mut mask = Mask::new(0, 0);
    let mut mem = HashMap::new();

    for line in lines() {
        let cap = re.captures(&line).unwrap();

        match (cap.name("mask"), cap.name("addr"), cap.name("value")) {
            (Some(m), None, None) => {
                mask = Mask::from_string(m.as_str());
            }
            (None, Some(addr), Some(value)) => {
                let addr: u64 = addr.as_str().parse().unwrap();
                let value = value.as_str().parse().unwrap();
                for addr in mask.make_addr(addr) {
                    mem.entry(addr).insert(value);
                }
            }
            _ => (),
        }
    }
    Some(mem.values().sum::<u64>() as i64)
}
