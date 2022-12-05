use itertools::Itertools;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl From<String> for Move {
    fn from(line: String) -> Move {
        let (_, count, _, from, _, to) = line.split(' ').collect_tuple().unwrap();
        Move {
            count: count.parse::<usize>().unwrap(),
            from: from.parse::<usize>().unwrap() - 1,
            to: to.parse::<usize>().unwrap() - 1
        }
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> where T: Clone, {
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn parse_input() -> (Vec<Move>, Vec<Vec<char>>) {
    let mut moves: Vec<Move> = Vec::new();
    let mut cargo_queue: Vec<Vec<Option<char>>> = Vec::new();

    let file = File::open("input/day5.txt").unwrap();
    let reader = BufReader::new(file);

    let mut in_move_list = false;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            in_move_list = true;
            continue;
        }

        if in_move_list {
            moves.push(Move::from(line));
        } else if line.starts_with('[') {
            let r: Vec<Option<char>> = line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|c| if c[0] == '[' { Some(c[1]) } else { None })
                .collect();
            
            cargo_queue.push(r);
        }
    }

    // transpose row-major into column-major
    let transposed = transpose(cargo_queue);

    // get rid of None's each column
    let mut cargo_stack = transposed
        .iter()
        .map(|e| e.iter().filter_map(|i| *i).collect_vec())
        .collect_vec();

    // transpose() transposes diagonally; items are in reverse order, so reverse each row in column-major
    for stack in cargo_stack.iter_mut() {
        stack.reverse();
    }

    (moves, cargo_stack)
}

pub fn imperative() -> io::Result<()> {
    println!("    Imperative:");
    
    let (moves, mut part_1_stack) = parse_input();

    let mut part_2_stack = part_1_stack.iter().cloned().collect_vec();

    // move according to part-1 behaviour (one item at a time)
    for m in moves.iter() {
        for _ in 0..m.count {
            let popped = part_1_stack[m.from].pop().unwrap();
            part_1_stack[m.to].push(popped);
        }
    }

    // move according to part-2 behaviour (n items at a time, retain order)
    let mut move_stack = Vec::new();
    for m in moves.iter() {
        for _ in 0..m.count {
            let popped = part_2_stack[m.from].pop().unwrap();
            move_stack.push(popped);
        }

        while !move_stack.is_empty() {
            part_2_stack[m.to].push(move_stack.pop().unwrap());
        }
    }

    let mut part_1_string = String::new();
    for stack in part_1_stack {
        part_1_string.push(*stack.last().unwrap());
    }

    let mut part_2_string = String::new();
    for stack in part_2_stack {
        part_2_string.push(*stack.last().unwrap());
    }

    println!("        part 1: {}", part_1_string);
    println!("        part 2: {}", part_2_string);

    Ok(())
}

pub fn run() -> io::Result<()> {
    println!("Day 5:");
    imperative()
}
