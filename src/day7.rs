// Copyright (c) 2020 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::util::lines;

struct Graph {
    string_to_id: HashMap<String, usize>,
    adj_matrix: Vec<Vec<usize>>,
}

fn build_graph(lines: Vec<String>) -> Graph {
    let re_child = Regex::new(r"(?P<count>\d+) (?P<color>[a-z ]+) bags?").unwrap();
    let re_rule = Regex::new(r"^(?P<parent>[a-z ]+) bags contain").unwrap();

    let parents: Vec<String> = lines
        .iter()
        .map(|l| {
            re_rule
                .captures(&l)
                .unwrap()
                .name("parent")
                .unwrap()
                .as_str()
                .to_string()
        })
        .collect();

    let string_to_id: HashMap<String, usize> = parents
        .iter()
        .enumerate()
        .map(|(i, s)| (s.to_string(), i))
        .collect();

    let child_set = lines.iter().map(|l| {
        re_child.captures_iter(&l).map(|cap| {
            (
                cap.name("count").unwrap().as_str().parse().unwrap(),
                string_to_id[cap.name("color").unwrap().as_str()],
            )
        })
    });

    let mut adj_matrix = vec![vec![0usize; string_to_id.len()]; string_to_id.len()];
    for (parent, children) in parents.iter().zip(child_set) {
        for (count, color) in children {
            adj_matrix[string_to_id[parent]][color] = count;
        }
    }

    Graph {
        string_to_id,
        adj_matrix,
    }
}

pub fn part1() -> Option<i64> {
    let graph = build_graph(lines());
    let shiny_gold = graph.string_to_id["shiny gold"];

    let mut discovered = HashSet::new();
    let mut stack = vec![shiny_gold];

    while !stack.is_empty() {
        let child = stack.pop().unwrap();

        for i in 0..graph.adj_matrix.len() {
            if graph.adj_matrix[i][child] > 0 {
                discovered.insert(i);
                stack.push(i);
            }
        }
    }

    Some(discovered.len() as i64)
}

fn count_children(graph: &Graph, node: usize, mem: &mut Vec<i64>) -> i64 {
    if mem[node] != -1 {
        return mem[node];
    }
    let answer: usize = graph.adj_matrix[node]
        .iter()
        .enumerate()
        .filter(|(_, &count)| count > 0)
        .map(|(i, count)| count + count * count_children(graph, i, mem) as usize)
        .sum();
    mem[node] = answer as i64;
    return answer as i64;
}

pub fn part2() -> Option<i64> {
    let graph = build_graph(lines());
    let shiny_gold = graph.string_to_id["shiny gold"];
    let mut mem = vec![-1; graph.adj_matrix.len()];
    Some(count_children(&graph, shiny_gold, &mut mem))
}
