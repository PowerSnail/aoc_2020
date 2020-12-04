use clap::clap_app;
use regex::Regex;
use std::{
    io::{self, BufRead},
    str::FromStr,
};

fn read_lines<T>() -> Vec<T>
where
    T: FromStr,
{
    let stdin = io::stdin();
    let mut numbers: Vec<T> = Vec::new();
    for line in stdin.lock().lines() {
        numbers.push(
            line.unwrap()
                .parse()
                .unwrap_or_else(|_| panic!("Parse Failed!")),
        );
    }
    return numbers;
}

fn day1_part1() -> Option<i64> {
    let mut numbers: Vec<i64> = read_lines();
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

fn day1_part2() -> Option<i64> {
    let mut numbers: Vec<i64> = read_lines();
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

fn day2_part1() -> Option<i64> {
    let stdin = io::stdin();
    let password_rule_re = Regex::new(r"(\d+)-(\d+) ([a-zA-Z0-9_]): ([a-zA-Z0-9_]*)").unwrap();
    let mut count_valid = 0;
    for line in stdin.lock().lines() {
        let line = &line.unwrap();
        let capture = password_rule_re.captures(line).unwrap();
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

fn day2_part2() -> Option<i64> {
    let stdin = io::stdin();
    let password_rule_re = Regex::new(r"(\d+)-(\d+) ([a-zA-Z0-9_]): ([a-zA-Z0-9_]*)").unwrap();
    let mut count_valid = 0;
    for line in stdin.lock().lines() {
        let line = &line.unwrap();
        let capture = password_rule_re.captures(line).unwrap();
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

fn day3_part1() -> Option<i64> {
    let stdin = io::stdin();
    let mut x: usize = 3;
    let mut tree_count = 0;
    for line in stdin.lock().lines().skip(1) {
        let line = &line.unwrap();
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

fn day3_part2() -> Option<i64> {
    let stdin = io::stdin();
    let rights = vec![1, 3, 5, 7, 1];
    let downs = vec![1, 1, 1, 1, 2];
    let mut tree_counts = vec![0, 0, 0, 0, 0];

    for (row, line) in stdin.lock().lines().enumerate().skip(1) {
        let line = &line.unwrap();
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

fn day4_part1() -> Option<i64> {
    let mut valid_count = 0;
    let mut passport = vec![false; 7];

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
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

fn day4_part2() -> Option<i64> {
    let re_hgt = Regex::new(r"^(\d+)(in|cm)$").unwrap();
    let re_hcl = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    let re_ecl = Regex::new(r"^(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)$").unwrap();
    let re_pid = Regex::new(r"^\d{9}$").unwrap();

    let mut valid_count = 0;
    let mut passport = vec![false; 7];

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
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
                    // if re_hgt.is_match(value) {
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
fn main() {
    let matches = clap_app!(myapp =>
        (@setting SubcommandRequiredElseHelp)
        (@subcommand "day1-part1" =>)
        (@subcommand "day1-part2" =>)
        (@subcommand "day2-part1" =>)
        (@subcommand "day2-part2" =>)
        (@subcommand "day3-part1" =>)
        (@subcommand "day3-part2" =>)
        (@subcommand "day4-part1" =>)
        (@subcommand "day4-part2" =>)
    )
    .get_matches();

    let answer = match matches.subcommand() {
        Some(("day1-part1", _)) => day1_part1(),
        Some(("day1-part2", _)) => day1_part2(),
        Some(("day2-part1", _)) => day2_part1(),
        Some(("day2-part2", _)) => day2_part2(),
        Some(("day3-part1", _)) => day3_part1(),
        Some(("day3-part2", _)) => day3_part2(),
        Some(("day4-part1", _)) => day4_part1(),
        Some(("day4-part2", _)) => day4_part2(),
        _ => unreachable!(),
    };

    match answer {
        Some(answer) => println!("{}", answer),
        None => println!("ERROR: no answer found"),
    }
}
