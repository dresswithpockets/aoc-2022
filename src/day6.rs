use itertools::Itertools;
use std::fs;
use std::hash::Hash;

fn find_first_unique_chunk<I>(v: &Vec<I>, n: usize) -> Option<usize>
where
    I: Clone + Eq + Hash,
{
    let offset = n - 1;
    (offset..v.len()).find(|i| v[i - offset..=*i].iter().unique().count() == n)
}

pub fn imperative() {
    println!("  Imperative:");

    let input = fs::read_to_string("input/day6.txt").unwrap();
    let chars = input.trim().chars().collect::<Vec<char>>();

    let end_of_packet_marker = find_first_unique_chunk(&chars, 4).unwrap();
    let end_of_message_marker = find_first_unique_chunk(&chars, 14).unwrap();

    println!("        end of packet marker: {}", end_of_packet_marker);
    println!("        end of message marker: {}", end_of_message_marker);
}

pub fn run() {
    println!("Day 6:");
    imperative();
}
