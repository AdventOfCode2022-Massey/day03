# Advent of Code 2022: Day 3
Bart Massey

---

We used a solution that takes advantage of set
operations. There was a dumb bug computing priorities that
we ultimately debugged using the test data provided with the
problem: it was causing an underflow panic. The second part
gratuitously used `itertools` to do the grouping.

---

Solution to [this problem](https://adventofcode.com/2022/day/3).

Save your problem input to `input.txt` and run with

    cargo run --release 1 <input.txt
    cargo run --release 2 <input.txt

---

This program is licensed under the "MIT License".
Please see the file LICENSE in this distribution
for license terms.
