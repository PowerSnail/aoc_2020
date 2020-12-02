use clap::clap_app;
use std::{str::FromStr, io::{self, BufRead}};
use regex::Regex;


fn read_lines<T>() -> Vec<T>
    where T : FromStr 
{
    let stdin = io::stdin();
    let mut numbers: Vec<T> = Vec::new();
    for line in stdin.lock().lines() {
        numbers.push(line.unwrap().parse().unwrap_or_else(|_| panic!("Parse Failed!")));
    }
    return numbers;
} 

fn day1_part1() -> Option<i32> {
    let mut numbers: Vec<i32> = read_lines();
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

fn day1_part2() -> Option<i32> {
    let mut numbers: Vec<i32> = read_lines();
    numbers.sort();

    for lo in 0..numbers.len() - 1 {
        for hi in (lo..numbers.len()).rev() {
            let remain = 2020 - numbers[lo] - numbers[hi];
            match numbers.binary_search(&remain) {
                Ok(_) => return Some(remain * numbers[lo] * numbers[hi]),
                _ => continue
            }
        }
    }
    None
}

fn day2_part1() -> Option<i32> {
    let stdin = io::stdin();
    let password_rule_re = Regex::new(r"(\d+)-(\d+) ([a-zA-Z0-9_]): ([a-zA-Z0-9_]*)").unwrap();
    let mut count_valid = 0;
    for line in stdin.lock().lines() {
        let line = &line.unwrap();
        let capture = password_rule_re.captures(line).unwrap();
        let target = capture[3].as_bytes()[0];
        let letter_count = capture[4].as_bytes().into_iter().filter(|&&x| x == target).count();
        if capture[1].parse::<usize>().unwrap() <= letter_count && letter_count <= capture[2].parse::<usize>().unwrap() {
            count_valid += 1;
        }
    }
    Some(count_valid)
}

fn day2_part2() -> Option<i32> {
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

fn main() {
    let matches = clap_app!(myapp =>
        (@setting SubcommandRequiredElseHelp)
        (@subcommand "day1-part1" =>)
        (@subcommand "day1-part2" =>)
        (@subcommand "day2-part1" =>)
        (@subcommand "day2-part2" =>)
    ).get_matches();

    let answer = match matches.subcommand() {
        Some(("day1-part1", _)) => day1_part1(),
        Some(("day1-part2", _)) => day1_part2(),
        Some(("day2-part1", _)) => day2_part1(),
        Some(("day2-part2", _)) => day2_part2(),
        _ => unreachable!(),
    };

    match answer {
        Some(answer) => println!("{}", answer),
        None => println!("ERROR: no answer found"),
    }
}
