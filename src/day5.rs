
use itertools::Itertools;
use std::fs;
use std::io;
use std::str::FromStr;

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

type Stack = Vec<char>;

type Cargo = Vec<Stack>;

impl Move {
    fn apply_series(&self, cargo: &mut Cargo) {
        for _ in 0..self.count {
            let popped = cargo[self.from].pop().unwrap();
            cargo[self.to].push(popped);
        }
    }

    fn apply_chunk(&self, cargo: &mut Cargo) {
        // note(ash) could probably have a shared move stack used across all apply_chunk calls
        let mut move_stack = Vec::new();
        for _ in 0..self.count {
            let popped = cargo[self.from].pop().unwrap();
            move_stack.push(popped);
        }

        while !move_stack.is_empty() {
            cargo[self.to].push(move_stack.pop().unwrap());
        }
    }
}

impl FromStr for Move {
    type Err = i32;

    fn from_str(s: &str) -> Result<Move, Self::Err> {
        let (_, count, _, from, _, to) = s.split(' ').collect_tuple().unwrap();
        Ok(Move {
            count: count.parse::<usize>().unwrap(),
            from: from.parse::<usize>().unwrap() - 1,
            to: to.parse::<usize>().unwrap() - 1
        })
    }
}

fn parse_input() -> (Vec<Move>, Cargo) {
    let data = fs::read_to_string("input/day5.txt").unwrap();
    let (cargo_chunk, move_chunk) = data.split_once("\n\n").unwrap();

    let cargo_lines = cargo_chunk.lines().rev().collect_vec();
    let stack_count = cargo_lines.first().unwrap().len() / 4 + 1;

    let mut cargo = (0..stack_count).map(|_| Stack::new()).collect::<Cargo>();

    // the first line in the reversed chunk will be the numbers for each stack, so we just skip it
    // and iterate over the rest
    for line in cargo_lines.iter().skip(1) {
        // columns are always 4 spaces wide, padded to fill the entire matrix
        let chunks = line.chars().chunks(4);

        // each stack character is the 2nd item in each chunk of 4
        let row = chunks.into_iter().map(|mut c| c.nth(1).unwrap());
        
        for (i, c) in row.enumerate() {
            // we can skip adding whitespace to each cargo stack - there will never be white
            // space between two items in a stack
            if c.is_whitespace() {
                continue;
            }
            
            cargo[i].push(c);
        }
    }

    let moves = move_chunk.lines().filter_map(|line| line.parse::<Move>().ok()).collect_vec();

    (moves, cargo)
}

pub fn imperative() -> io::Result<()> {
    println!("    Imperative:");
    
    let (moves, mut part_1_stack) = parse_input();

    let mut part_2_stack = part_1_stack.iter().cloned().collect_vec();

    for m in moves.iter() {
        m.apply_series(&mut part_1_stack);
        m.apply_chunk(&mut part_2_stack);
    }

    let part_1_string = part_1_stack.iter().map(|stack| stack.last().unwrap()).collect::<String>();
    let part_2_string = part_2_stack.iter().map(|stack| stack.last().unwrap()).collect::<String>();

    println!("        part 1: {}", part_1_string);
    println!("        part 2: {}", part_2_string);

    Ok(())
}

pub fn run() -> io::Result<()> {
    println!("Day 5:");
    imperative()
}
