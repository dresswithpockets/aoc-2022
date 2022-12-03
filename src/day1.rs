use std::fs;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::vec::Vec;

pub fn progscan() -> io::Result<()> {
    println!("    Progressive Scan: ");

    let file = File::open("input/day1.txt")?;
    let reader = BufReader::new(file);

    let mut totals = Vec::new();
    let mut current_elf_cals = 0;

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            totals.push(current_elf_cals);
            current_elf_cals = 0;
            continue;
        }

        current_elf_cals += line.parse::<i32>().unwrap();
    }

    totals.sort_by(|a, b| b.cmp(a));

    println!("        top 1: {}", totals[0]);
    println!("        top 3 sum: {}", totals[0] + totals[1] + totals[2]);
    println!("");

    Ok(())
}

pub fn functional() -> io::Result<()> {
    println!("    Functional: ");

    let mut totals: Vec<i32> = fs::read_to_string("input/day1.txt")?
        .split("\n\n")
        .map(|elf_set| {
            elf_set
                .split("\n")
                .filter_map(|x| x.trim().parse::<i32>().ok())
                .sum()
        })
        .collect();

    totals.sort_by(|a, b| b.cmp(a));

    println!("        top 1: {}", totals[0]);
    println!("        top 3 sum: {}", totals.iter().take(3).sum::<i32>());
    println!("");

    Ok(())
}

pub fn run() -> io::Result<()> {
    println!("Day 1: ");
    progscan().and(functional())
}
