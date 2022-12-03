use itertools::Itertools;
use std::fs;
use std::io;

pub fn functional() -> io::Result<()> {
    println!("    Functional:");

    let score = fs::read_to_string("input/day2.txt")?
        .split("\n")
        .map(|line| match line.splitn(2, " ").collect_tuple() {
            Some(("A", "X")) => (1 + 3, 3 + 0),
            Some(("A", "Y")) => (2 + 6, 1 + 3),
            Some(("A", "Z")) => (3 + 0, 2 + 6),

            Some(("B", "X")) => (1 + 0, 1 + 0),
            Some(("B", "Y")) => (2 + 3, 2 + 3),
            Some(("B", "Z")) => (3 + 6, 3 + 6),

            Some(("C", "X")) => (1 + 6, 2 + 0),
            Some(("C", "Y")) => (2 + 0, 3 + 3),
            Some(("C", "Z")) => (3 + 3, 1 + 6),
            _ => panic!("Unexpected result from line: '{}'", line),
        })
        .fold((0, 0), |acc, e| (acc.0 + e.0, acc.1 + e.1));

    println!("        part 1: {}", score.0);
    println!("        part 2: {}", score.1);
    println!("");

    Ok(())
}

pub fn run() -> io::Result<()> {
    println!("Day 2:");
    functional()
}
