// Copyright (c) 2020 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;

use crate::{regex_static, util::lines};
use lazy_static::lazy_static;
use regex::Regex;

// fn _parse<'a>(i: usize, lines: &[&str], regexes: &'a mut [Option<String>]) -> &'a str {
//     if let Some(ref s) = regexes[i] {
//         return s;
//     }
//     if lines[i].chars().nth(0).unwrap() == '"' {
//         regexes[i] = Some(lines[i][1..2].to_owned());
//     } else {
//         let part = lines[i]
//             .split(" | ")
//             .map(|s| {
//                 let part = s
//                     .split(' ')
//                     .map(|s| s.parse::<usize>().unwrap())
//                     .map(|i| _parse(i, lines, regexes).to_owned())
//                     .collect::<Vec<_>>()
//                     .concat();
//                 format!("{}", &part)
//             })
//             .collect::<Vec<_>>()
//             .join("|");
//         regexes[i] = Some(format!("({})", &part));
//     }

//     if let Some(ref s) = regexes[i] {
//         return s;
//     }
//     unreachable!()
// }

// fn parse_rules(lines: &[String]) -> Option<Regex> {
//     let mut sorted = vec![""; lines.len()];
//     for s in lines.iter() {
//         let cap = RE_LINE.captures(s)?;
//         let id: usize = cap.name("ID")?.as_str().parse().ok()?;
//         let content = cap.name("Content")?.as_str();
//         sorted[id] = content;
//     }

//     let mut regexes = vec![None; lines.len()];
//     let r0 = format!("^{}$", _parse(0, &sorted, &mut regexes));
//     Regex::new(&r0).ok()
// }

#[derive(Debug, Clone)]
enum Symbol {
    NonTerminal(usize),
    Terminal(char),
    Sentinel,
}

struct Grammar {
    symbols: Vec<Symbol>,
    rules: Vec<(usize, Vec<Symbol>)>,
}

regex_static!(
    RE_LINE,
    r#"^(?P<ID>\d+): ("(?P<Terminal>.)"|(?P<NonTerminal>\d.*))$"#
);

fn parse_rule_list(lines: &[String]) -> Grammar {
    let mut rules = Vec::new();
    let mut symbols = vec![Symbol::Sentinel; lines.len()];

    for line in lines {
        let cap = RE_LINE.captures(line).unwrap();
        let id = cap.name("ID").unwrap().as_str().parse().unwrap();

        if let Some(m) = cap.name("Terminal") {
            symbols[id] = Symbol::Terminal(m.as_str().chars().nth(0).unwrap());
        } else {
            let s = cap.name("NonTerminal").unwrap().as_str();
            for group in s.split(" | ") {
                let lhs = group
                    .split(' ')
                    .map(|s| s.parse::<usize>().unwrap())
                    .map(|i| Symbol::NonTerminal(i))
                    .collect::<Vec<_>>();
                rules.push((id, lhs));
            }
            symbols[id] = Symbol::NonTerminal(id);
        }
    }

    Grammar { symbols, rules }
}

fn build_first_set_(
    id: usize,
    grammar: &Grammar,
    first: &mut Vec<[bool; 256]>,
    visited: &mut Vec<bool>,
) {
    if visited[id] {
        return;
    }
    if let Symbol::NonTerminal(n) = grammar.symbols[id] {
        for rule in grammar.rules.iter().filter(|r| r.0 == id) {
            match rule.1[0] {
                Symbol::Terminal(n) => first[id][n as usize] = true,
                Symbol::NonTerminal(n) => {
                    build_first_set_(n, grammar, first, visited);
                    for i in 0..256 {
                        first[id][i] |= first[n][i];
                    }
                }
                Symbol::Sentinel => unreachable!(),
            }
        }
    }
    visited[id] = true;
}

fn build_first_set(grammar: &Grammar) -> Vec<[bool; 256]> {
    let mut first = vec![[false; 256]; grammar.symbols.len()];
    let mut visited = vec![false; grammar.symbols.len()];

    build_first_set_(0, grammar, &mut first, &mut visited);
    return first;
}

fn build_table(grammar: &Grammar) {
    let first_set = build_first_set(grammar);
    let mut table = vec![[-1; 256]; grammar.symbols.len()];

    for rule in grammar.rules.iter() {
        if rule.0
    }
}

pub fn part1() -> Option<i64> {
    let mut input = lines();
    let empty_line = input.iter().position(|s| s == "")?;
    let r = parse_rules(&mut input[..empty_line])?;
    let messages = &input[empty_line + 1..];

    Some(messages.iter().filter(|&s| r.is_match(s)).count() as i64)
}

pub fn part2() -> Option<i64> {
    let mut input = lines();
    let empty_line = input.iter().position(|s| s == "")?;
    let r = parse_rules(&mut input[..empty_line])?;
    let messages = &input[empty_line + 1..];

    Some(messages.iter().filter(|&s| r.is_match(s)).count() as i64)
}
