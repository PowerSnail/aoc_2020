// Copyright (c) 2020 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
//
// Damn it! I solved the wrong one!

use crate::util::lines;
use regex::Regex;

macro_rules! v_max {
    ($t:ident, $e:expr) => {
        $t.iter().map(|&x| x.max($e)).collect()
    };
}

macro_rules! v_times {
    ($t:ident, $e:expr) => {
        $t.iter().map(|&x| x * $e).collect()
    };
}

macro_rules! v_add {
    ($x:ident, $y:ident) => {
        $x.iter().zip($y.iter()).map(|(&x1, &x2)| x1 + x2).collect()
    };
}

fn find_max_score(data: &[Vec<i64>], count: i64, accu: Vec<i64>) -> i64 {
    if accu[accu.len() - 1] > 500 {
        return -1;
    }
    if count == 0 {
        let accu: Vec<i64> = v_max!(accu, 0);
        return accu.into_iter().take(4).fold_first(|x, y| x * y).unwrap();
    }

    let minimum = if data.len() == 1 { count } else { 0 };
    (minimum..count + 1)
        .map(|c| {
            let remained = &data[0];
            let remained: Vec<i64> = v_times!(remained, c);
            let accu: Vec<i64> = v_add!(remained, accu);
            find_max_score(&data[1..], count - c, accu)
        })
        .max()
        .unwrap()
}

pub fn part1() -> Option<i64> {
    let re = Regex::new(r"(?P<name>[a-zA-Z]+): capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavor>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>-?\d+)").unwrap();
    let data: Vec<Vec<i64>> = lines()
        .into_iter()
        .map(|l| {
            let c = re.captures(l.as_str()).unwrap();
            vec![
                c.name("capacity").unwrap().as_str().parse().unwrap(),
                c.name("durability").unwrap().as_str().parse().unwrap(),
                c.name("flavor").unwrap().as_str().parse().unwrap(),
                c.name("texture").unwrap().as_str().parse().unwrap(),
                0,
            ]
        })
        .collect();

    Some(find_max_score(&data, 100, vec![0, 0, 0, 0, 0]))
}

pub fn part2() -> Option<i64> {
    let re = Regex::new(r"(?P<name>[a-zA-Z]+): capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavor>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>-?\d+)").unwrap();
    let data: Vec<Vec<i64>> = lines()
        .into_iter()
        .map(|l| {
            let c = re.captures(l.as_str()).unwrap();
            vec![
                c.name("capacity").unwrap().as_str().parse().unwrap(),
                c.name("durability").unwrap().as_str().parse().unwrap(),
                c.name("flavor").unwrap().as_str().parse().unwrap(),
                c.name("texture").unwrap().as_str().parse().unwrap(),
                c.name("calories").unwrap().as_str().parse().unwrap(),
            ]
        })
        .collect();

    Some(find_max_score(&data, 100, vec![0, 0, 0, 0, 0]))
}
