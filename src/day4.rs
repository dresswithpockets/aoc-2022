use itertools::Itertools;
use std::fs;
use std::io;

pub fn functional() -> io::Result<()> {
    println!("    Realistic:");

    let range_pairs: Vec<((i32, i32), (i32, i32))> = fs::read_to_string("input/day4.txt")?
        .trim()
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|range| {
                    range
                        .split("-")
                        .map(|i| i.parse::<i32>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect();
    
    let completely_contains_count = range_pairs
        .iter()
        .filter(|pair| {
            (pair.0 .0 <= pair.1 .0 && pair.0 .1 >= pair.1 .1)
                || (pair.1 .0 <= pair.0 .0 && pair.1 .1 >= pair.0 .1)
        })
        .count();
    
    let partially_contains_count = range_pairs
        .iter()
        .filter(|pair| pair.0 .0 <= pair.1 .1 && pair.0 .1 >= pair.1 .0)
        .count();

    println!("        part 1: {}", completely_contains_count);
    println!("        part 2: {}", partially_contains_count);

    Ok(())
}

pub fn run() -> io::Result<()> {
    println!("Day 4:");
    functional()
}
