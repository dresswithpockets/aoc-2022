use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::vec::Vec;

fn print_usage(cmd: &String) {
    println!("{} (day) (variant)", cmd);
}

fn day1_progscan() -> io::Result<()> {
    println!("    Progressive Scan: ");

    let file = File::open("day1.txt")?;
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

fn day1_functional() -> io::Result<()> {
    println!("    Functional: ");

    let mut totals: Vec<i32> = fs::read_to_string("day1.txt")?
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

fn day1() -> io::Result<()> {
    println!("Day 1: ");
    day1_progscan().and(day1_functional())
}

fn run_all() -> io::Result<()> {
    day1()
}

fn main() -> io::Result<()> {

    
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print_usage(&args[0]);
        println!("Defaulting to run_all\n");
        return run_all();
    }

    let day = &args[1];
    let variant = &args[2];

    let result = match &day[..] {
        "day1" => match &variant[..] {
            "functional" => day1_functional(),
            _ => day1_progscan(),
        },
        _ => Err(io::Error::new(io::ErrorKind::Other, "oh no!")),
    };

    if let Err(err) = result {
        println!("{}", err);
        print_usage(&args[0]);
    }

    Ok(())
}
