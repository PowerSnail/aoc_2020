// Copyright (c) 2020 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashSet;

use crate::util::lines;

const NOP: i64 = 0;
const ACC: i64 = 1;
const JMP: i64 = 2;

fn parse_line(line: String) -> (i64, i64) {
    (
        match &line[..3] {
            "nop" => NOP,
            "acc" => ACC,
            "jmp" => JMP,
            _ => unreachable!()
        },
        line[4..].parse::<i64>().unwrap(),
    )
}

fn run_until_loop_or_terminate(commands: &Vec<(i64, i64)>) -> (i64, i64) {
    let mut pc: i64 = 0;
    let mut acc: i64 = 0;
    let mut past_pc = HashSet::new();

    while !past_pc.contains(&pc) && pc < commands.len() as i64 {
        past_pc.insert(pc);
        match &commands[pc as usize] {
            (NOP, _) => pc += 1,
            (ACC, arg) => {
                pc += 1;
                acc += arg
            }
            (JMP, step) => pc += step,
            _ => unreachable!(),
        };
    }
    (pc, acc)
}

pub fn part1() -> Option<i64> {
    let commands = lines().into_iter().map(parse_line).collect();
    let (_, acc) = run_until_loop_or_terminate(&commands);
    Some(acc)
}

pub fn part2() -> Option<i64> {
    let mut commands: Vec<(i64, i64)> = lines().into_iter().map(parse_line).collect();
    
    for i in 0..commands.len() {
        match commands[i] {
            (JMP, arg) => {
                commands[i] = (NOP, arg);
                let (pc, acc) = run_until_loop_or_terminate(&commands);
                if pc == commands.len() as i64 {
                    return Some(acc)
                }
                commands[i] = (JMP, arg);
            },
            (NOP, arg) => {
                commands[i] = (JMP, arg);
                let (pc, acc) = run_until_loop_or_terminate(&commands);
                if pc == commands.len() as i64 {
                    return Some(acc)
                }
                commands[i] = (NOP, arg);
            },
            _ => ()
        }
    }
    None
}
