// Copyright (c) 2020 PowerSnail
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::util::lines;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    LeftBracket,
    RightBracket,
    Add,
    Multiply,
    Number(i64),
}

fn tokenize(line: &str) -> Vec<Token> {
    line.chars()
        .filter_map(|c| match c {
            ' ' => None,
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                Some(Token::Number((c as u8 - '0' as u8) as i64))
            }
            '+' => Some(Token::Add),
            '*' => Some(Token::Multiply),
            '(' => Some(Token::LeftBracket),
            ')' => Some(Token::RightBracket),
            _ => unreachable!(),
        })
        .collect()
}

fn eval_expr_easy(tokens: &[Token], i: usize) -> (usize, i64) {
    let (i, lhs) = match tokens[i] {
        Token::Number(n) => (i + 1, n),
        Token::LeftBracket => eval_expr_easy(tokens, i + 1),
        _ => panic!(format!("Malformed; got {:?} at left hand side", tokens[i])),
    };
    if i == tokens.len() {
        return (i, lhs);
    }
    match tokens[i] {
        Token::Add => {
            let (i, rhs) = eval_expr_easy(tokens, i + 1);
            (i, lhs + rhs)
        }
        Token::Multiply => {
            let (i, rhs) = eval_expr_easy(tokens, i + 1);
            (i, lhs * rhs)
        }
        Token::RightBracket => (i + 1, lhs),
        _ => panic!(format!(
            "Malformed; got {:?} after left hand side",
            tokens[i]
        )),
    }
}

pub fn part1() -> Option<i64> {
    Some(
        lines()
            .iter()
            .map(|s| {
                // Reverse the string, because I'm too lazy to implement left associativity
                s.chars()
                    .rev()
                    .map(|c| match c {
                        '(' => ')',
                        ')' => '(',
                        _ => c,
                    })
                    .collect::<String>()
            })
            .map(|s| tokenize(&s))
            .map(|tokens| eval_expr_easy(&tokens, 0).1)
            .sum(),
    )
}

// Expr := Expr2 (* Expr2)*
// Expr2 := Element (+ Element)*
// Element := Number | ( Expr )

fn eval_element(tokens: &[Token], i: usize) -> (usize, i64) {
    match tokens[i] {
        Token::Number(n) => (i + 1, n),
        Token::LeftBracket => {
            let (i, val) = eval_expr(tokens, i + 1);
            match tokens[i] {
                Token::RightBracket => (i + 1, val),
                _ => panic!("Parser Error"),
            }
        }
        _ => panic!("Parser Error"),
    }
}

fn eval_expr2(tokens: &[Token], i: usize) -> (usize, i64) {
    let (mut i, mut left) = eval_element(tokens, i);

    while i < tokens.len() && tokens[i] == Token::Add {
        let next_group = eval_element(tokens, i + 1);
        i = next_group.0;
        left += next_group.1;
    }

    (i, left)
}

fn eval_expr(tokens: &[Token], i: usize) -> (usize, i64) {
    let (mut i, mut left) = eval_expr2(tokens, i);

    while i < tokens.len() && tokens[i] == Token::Multiply {
        let next_group = eval_expr2(tokens, i + 1);
        i = next_group.0;
        left *= next_group.1;
    }

    (i, left)
}

pub fn part2() -> Option<i64> {
    Some(
        lines()
            .iter()
            .map(|s| tokenize(s))
            .map(|tokens| eval_expr(&tokens, 0).1)
            .sum(),
    )
}
