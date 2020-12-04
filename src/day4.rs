// Copyright (c) 2020 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use regex::Regex;

use crate::util::lines;

pub fn part1() -> Option<i64> {
    let mut valid_count = 0;
    let mut passport = vec![false; 7];

    for line in lines() {
        if line == "" {
            valid_count += passport.iter().all(|&b| b) as i64;
            passport = vec![false; 7];
            continue;
        }
        for token in line.split(" ") {
            match token.split(":").nth(0).unwrap() {
                "byr" => passport[0] = true,
                "iyr" => passport[1] = true,
                "eyr" => passport[2] = true,
                "hgt" => passport[3] = true,
                "hcl" => passport[4] = true,
                "ecl" => passport[5] = true,
                "pid" => passport[6] = true,
                "cid" => (),
                _ => {
                    println!("Not matching: {}", token);
                    panic!("Parser error");
                }
            }
        }
    }
    Some(valid_count)
}

pub fn part2() -> Option<i64> {
    let re_hgt = Regex::new(r"^(\d+)(in|cm)$").unwrap();
    let re_hcl = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    let re_ecl = Regex::new(r"^(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)$").unwrap();
    let re_pid = Regex::new(r"^\d{9}$").unwrap();

    let mut valid_count = 0;
    let mut passport = vec![false; 7];

    for line in lines() {
        let line = line.trim();
        if line == "" {
            valid_count += passport.iter().all(|&b| b) as i64;
            passport = vec![false; 7];
            continue;
        }
        for token in line.split(" ") {
            let tokens = token.split(":").take(2).collect::<Vec<_>>();
            match &tokens[..] {
                ["byr", value] => {
                    if let Ok(number) = value.parse::<u32>() {
                        passport[0] = number >= 1920 && number <= 2002;
                    }
                }
                ["iyr", value] => {
                    if let Ok(number) = value.parse::<u32>() {
                        passport[1] = number >= 2010 && number <= 2020;
                    }
                }
                ["eyr", value] => {
                    if let Ok(number) = value.parse::<u32>() {
                        passport[2] = number >= 2020 && number <= 2030;
                    }
                }
                ["hgt", value] => {
                    if let Some(cap) = re_hgt.captures(value) {
                        if let Ok(number) = cap[1].parse::<u32>() {
                            passport[3] = match &cap[2] {
                                "cm" => number >= 150 && number <= 193,
                                "in" => number >= 59 && number <= 76,
                                _ => false,
                            }
                        }
                    }
                }
                ["hcl", value] => passport[4] = re_hcl.is_match(value),
                ["ecl", value] => passport[5] = re_ecl.is_match(value),
                ["pid", value] => passport[6] = re_pid.is_match(value),
                ["cid", _] => (),
                _ => {
                    println!("Not matching: {}", token);
                    panic!("Parser error");
                }
            }
        }
    }
    Some(valid_count)
}
