// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 3.  
//! Ben and Bart Massey 2022

use std::collections::HashSet;

use aoc::*;
use itertools::Itertools;

/// What are the contents of a compartment?
fn compartment(chars: &[char]) -> HashSet<char> {
    chars.iter().copied().collect()
}

/// What is the priority of the given char?
fn priority(c: char) -> u32 {
    let base = if c.is_ascii_lowercase() {
        'a' as u32
    } else if c.is_ascii_uppercase() {
        // XXX `-` here because we will subtract again below.
        // Note that this only works because b`A` is known
        // to be ≥ 26.
        'A' as u32 - 26
    } else {
        panic!("unknown priority: {c}")
    };
    // XXX Don't forget to add one for the priority offset.
    c as u32 - base + 1
}

fn main() {
    match get_part() {
        Part1 => {
            // This part is imperative and follows Ben's pseudocode.
            // Start with running sum.
            let mut total = 0u32;
            for line in input_lines() {
                // Convert from `&str` to `char`s.
                let line: Vec<char> = line.chars().collect();
                let nline = line.len();
                assert!(nline & 1 == 0, "uneven knapsack");

                // Find the mispacked element. Note that
                // `intersection()` returns an iterator over
                // element references, which is quite
                // annoying.
                let left = compartment(&line[..nline / 2]);
                let right = compartment(&line[nline / 2..]);
                let mut elems = left.intersection(&right);

                // Extract and add in the mispacked element.
                let e = *elems.next().expect("undamaged knapsack");
                assert!(elems.next().is_none(), "misdamaged knapsack");
                total += priority(e);
            }
            println!("{total}");
        }
        Part2 => {
            // We'll go functional-style here because
            // `itertools::chunks()`.
            let total = input_lines()
                .chunks(3)
                .into_iter()
                .map(|lines| {
                    // Pick out the badge.
                    let mut elems = lines
                        .map(|line| {
                            // This whole knapsack is one
                            // compartment.
                            let line: Vec<char> =
                                line.chars().collect();
                            compartment(&line)
                        })
                        .reduce(|a, e| {
                            // Start with the first
                            // compartment in accumulator
                            // `a` and and keep intersecting
                            // compartments `e` until
                            // done. Need to keep ripping
                            // off gratuitous element
                            // references via `copied()`.
                            a.intersection(&e).copied().collect()
                        })
                        .expect("no badge lines")
                        .into_iter();

                    // Extract and add in the badge.
                    let e = elems.next().expect("no badge");
                    assert!(elems.next().is_none(), "multiple badges");
                    priority(e)
                })
                .sum::<u32>();
            println!("{total}");
        }
    }
}
