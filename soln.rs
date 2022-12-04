// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 3.  
//! Bart Massey 2022

use std::collections::HashSet;

use aoc::*;

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
                let misplaced = left.intersection(&right);
                let mut elems = misplaced.into_iter();
                let e = *elems.next().expect("undamaged knapsack");
                assert!(elems.next().is_none(), "misdamaged knapsack");
                total += priority(e);
            }
            println!("{total}");
        }
        Part2 => todo!(),
    }
}
