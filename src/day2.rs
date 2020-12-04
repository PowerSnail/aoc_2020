// Copyright (c) 2020 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::util::lines;

use regex::Regex;

pub fn part1() -> Option<i64> {
    let password_rule_re = Regex::new(r"(\d+)-(\d+) ([a-zA-Z0-9_]): ([a-zA-Z0-9_]*)").unwrap();
    let mut count_valid = 0;
    for line in lines() {
        let capture = password_rule_re.captures(&line).unwrap();
        let target = capture[3].as_bytes()[0];
        let letter_count = capture[4]
            .as_bytes()
            .into_iter()
            .filter(|&&x| x == target)
            .count();
        if capture[1].parse::<usize>().unwrap() <= letter_count
            && letter_count <= capture[2].parse::<usize>().unwrap()
        {
            count_valid += 1;
        }
    }
    Some(count_valid)
}

pub fn part2() -> Option<i64> {
    let password_rule_re = Regex::new(r"(\d+)-(\d+) ([a-zA-Z0-9_]): ([a-zA-Z0-9_]*)").unwrap();
    let mut count_valid = 0;
    for line in lines() {
        let capture = password_rule_re.captures(&line).unwrap();
        let i: usize = capture[1].parse::<usize>().unwrap() - 1;
        let j: usize = capture[2].parse::<usize>().unwrap() - 1;
        let target = capture[3].as_bytes()[0];
        let password = capture[4].as_bytes();

        if (password[i] == target) ^ (password[j] == target) {
            count_valid += 1;
        }
    }
    Some(count_valid)
}
