// Copyright (c) 2020 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use regex::Regex;

use crate::{remove_first, util::lines};

struct Rule {
    is_departure: bool,
    lo_1: usize,
    hi_1: usize,
    lo_2: usize,
    hi_2: usize,
}

impl Rule {
    pub fn has(&self, n: usize) -> bool {
        n >= self.lo_1 && n <= self.hi_1 || n >= self.lo_2 && n <= self.hi_2
    }
}

fn get_rules(input: &[String]) -> Vec<Rule> {
    let re = Regex::new(
        r"^(?P<name>[a-z ]+): (?P<lo_1>\d+)-(?P<hi_1>\d+) or (?P<lo_2>\d+)-(?P<hi_2>\d+)$",
    )
    .unwrap();
    let mut rules = Vec::new();
    for line in input.iter() {
        let captures = re.captures(&line).unwrap();
        rules.push(Rule {
            is_departure: captures
                .name("name")
                .unwrap()
                .as_str()
                .starts_with("departure"),
            lo_1: captures.name("lo_1").unwrap().as_str().parse().unwrap(),
            hi_1: captures.name("hi_1").unwrap().as_str().parse().unwrap(),
            lo_2: captures.name("lo_2").unwrap().as_str().parse().unwrap(),
            hi_2: captures.name("hi_2").unwrap().as_str().parse().unwrap(),
        });
    }
    rules
}

fn parse_ticket(s: &str) -> Vec<usize> {
    s.split(',').map(|t| t.parse().unwrap()).collect()
}

fn parse_input(inputs: &[String]) -> (Vec<Rule>, Vec<usize>, Vec<Vec<usize>>) {
    let inputs: Vec<_> = inputs.split(|l| l == "").take(3).collect();
    let rules = get_rules(inputs[0]);
    let my_ticket = parse_ticket(&inputs[1][1]);
    let nearby_tickets: Vec<_> = inputs[2][1..].iter().map(|s| parse_ticket(s)).collect();
    (rules, my_ticket, nearby_tickets)
}

pub fn part1() -> Option<i64> {
    let (rules, _, nearby_tickets) = parse_input(&lines());
    let total: usize = nearby_tickets
        .into_iter()
        .flatten()
        .filter(|&n| rules.iter().map(|r| !r.has(n)).all(|b| b))
        .sum();
    Some(total as i64)
}

pub fn part2() -> Option<i64> {
    let (rules, my_ticket, nearby_tickets) = parse_input(&lines());

    let n_column = rules.len();
    let mut column_rules: Vec<Vec<usize>> = vec![(0..n_column).collect(); n_column];

    for row in nearby_tickets.iter() {
        let new_rules: Vec<_> = row
            .iter()
            .zip(column_rules.iter())
            .filter_map(|(&n, valid_rules)| {
                let new_rules: Vec<usize> = valid_rules
                    .iter()
                    .filter(|&&r| rules[r].has(n))
                    .map(|&n| n)
                    .collect();
                match new_rules.len() {
                    0 => None,
                    _ => Some(new_rules),
                }
            })
            .collect();
        if new_rules.len() == n_column {
            column_rules = new_rules;
        }
        // Ignore lines with invalid inputs
    }

    let mut unassigned: Vec<_> = (0..n_column).collect();
    while unassigned.len() > 0 {
        let &column = unassigned
            .iter()
            .find(|&&i| column_rules[i].len() == 1)
            .unwrap();
        remove_first!(unassigned, column);

        let rule_id = column_rules[column][0];
        for &j in unassigned.iter() {
            remove_first!(column_rules[j], rule_id);
        }
    }
    Some(
        my_ticket
            .into_iter()
            .enumerate()
            .filter_map(|(i, v)| match rules[column_rules[i][0]].is_departure {
                true => Some(v),
                false => None,
            })
            .fold_first(|x, y| x * y)
            .unwrap() as i64,
    )
}
