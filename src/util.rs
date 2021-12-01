use std::{
    io::{self, BufRead},
    str::FromStr,
};

// Copyright (c) 2020 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub fn lines_as<T>() -> Vec<T>
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

pub fn lines() -> Vec<String> {
    let stdin = io::stdin();
    stdin.lock().lines().map(|r| r.unwrap()).collect()
}

#[macro_export]
macro_rules! regex_static {
    ($i:ident, $j:literal) => {
        lazy_static! {
            static ref $i: Regex = Regex::new($j).unwrap();
        }
    };
}

#[macro_export]
macro_rules! remove_first {
    ($i:expr, $n:expr) => {{
        if let Some(x) = $i.iter().position(|x| *x == $n) {
            $i.remove(x);
        }
    }};
}
