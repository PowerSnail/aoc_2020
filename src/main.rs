use clap::clap_app;
use std::io::{self, BufRead};

fn read_numbers() -> Vec<i32> {
    let stdin = io::stdin();
    let mut numbers: Vec<i32> = Vec::new();
    for line in stdin.lock().lines() {
        numbers.push(line.unwrap().parse().unwrap())
    }
    return numbers;
}

fn day1_part1() -> Option<i32> {
    let mut numbers = read_numbers();
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
    let mut numbers: Vec<i32> = read_numbers();
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

fn main() {
    let matches = clap_app!(myapp =>
        (@setting SubcommandRequiredElseHelp)
        (@subcommand "day1-part1" =>)
        (@subcommand "day1-part2" =>)
    ).get_matches();

    let answer = match matches.subcommand() {
        Some(("day1-part1", _)) => day1_part1(),
        Some(("day1-part2", _)) => day1_part2(),
        _ => unreachable!(),
    };

    match answer {
        Some(answer) => println!("{}", answer),
        None => println!("ERROR: no answer found"),
    }
}
