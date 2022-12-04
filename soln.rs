// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 3.  
//! Bart Massey 2022

use std::collections::HashSet;

use aoc::*;
use itertools::Itertools;

fn compartment(chars: &[char]) -> HashSet<char> {
    chars.iter().copied().collect()
}

fn priority(c: char) -> u32 {
    let base = if c.is_ascii_lowercase() {
        'a' as u32
    } else if c.is_ascii_uppercase() {
        'A' as u32 - 26
    } else {
        panic!("unknown priority: {c}")
    };
    c as u32 - base + 1
}

fn main() {
    match get_part() {
        Part1 => {
            let mut total = 0u32;
            for line in input_lines() {
                let line: Vec<char> = line.chars().collect();
                let nline = line.len();
                assert!(nline & 1 == 0, "uneven knapsack");
                let left = compartment(&line[..nline / 2]);
                let right = compartment(&line[nline / 2..]);
                let mut elems = left.intersection(&right);
                let e = *elems.next().expect("undamaged knapsack");
                assert!(elems.next().is_none(), "misdamaged knapsack");
                total += priority(e);
            }
            println!("{total}");
        }
        Part2 => {
            let total = input_lines()
                .chunks(3)
                .into_iter()
                .map(|lines| {
                    let mut elems = lines
                        .map(|line| {
                            let line: Vec<char> = line.chars().collect();
                            compartment(&line)
                        })
                        .reduce(|a, e| a.intersection(&e).copied().collect())
                        .expect("no badge lines")
                        .into_iter();
                    let e = elems.next().expect("no badge");
                    assert!(elems.next().is_none(), "multiple badges");
                    priority(e)
                })
                .sum::<u32>();
            println!("{total}");
        }
    }
}
