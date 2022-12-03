use std::fs;
use std::io;
use std::iter;

struct Sack {
    left: Vec<u32>,
    right: Vec<u32>,
}

struct Group<'a> {
    sacks: &'a [Sack],
    badge_id: usize,
}

impl Sack {
    pub fn new() -> Self {
        Self {
            left: iter::repeat(0).take(52).collect(),
            right: iter::repeat(0).take(52).collect(),
        }
    }

    pub fn add_left(&mut self, id: usize) {
        self.left[id] += 1;
    }

    pub fn add_right(&mut self, id: usize) {
        self.right[id] += 1;
    }

    pub fn has_in_either(&self, id: usize) -> bool {
        self.left[id] > 0 || self.right[id] > 0
    }
}

fn name_to_id(name: char) -> usize {
    match name {
        'a'..='z' => name as usize - 'a' as usize,
        _ => name as usize - 'A' as usize + 26
    }
}

fn id_to_priority(id: usize) -> u32 {
    1 + id as u32
}

impl From<&str> for Sack {
    fn from(string: &str) -> Sack {
        let mut sack = Sack::new();
        
        let (left, right) = string.split_at(string.len() / 2);
        for c in left.chars() {
            sack.add_left(name_to_id(c));
        }
        for c in right.chars() {
            sack.add_right(name_to_id(c));
        }
        
        sack
    }
}

impl<'a> From<&'a[Sack]> for Group<'a> {
    fn from(sacks: &'a[Sack]) -> Group<'a> {
        Group {
            sacks: sacks,
            badge_id: (0usize..52usize)
                .filter(|i| {
                    sacks.into_iter()
                        .all(|sack| sack.has_in_either(*i))
                }).nth(0).unwrap(),
        }
    }
}

pub fn realistic() -> io::Result<()> {
    println!("    Realistic:");

    let sacks: Vec<Sack> = fs::read_to_string("input/day3.txt")?
        .split("\n")
        .map(Sack::from)
        .collect();
    
    let groups: Vec<Group> = sacks.chunks(3).map(Group::from).collect();

    let (misplaced_sum, badge_sum) = groups.into_iter().map(|group| {
            let misplaced_sum = group.sacks.into_iter().map(|sack| {
                (0..52).map(|i| {
                    if sack.left[i] > 0 && sack.right[i] > 0 {
                        id_to_priority(i)
                    } else {
                        0
                    }
                }).sum::<u32>()
            })
            .sum::<u32>();
        
            let badge_sum = id_to_priority(group.badge_id);
            (misplaced_sum, badge_sum)
        })
        .fold((0, 0), |acc, e| (acc.0 + e.0, acc.1 + e.1));

    println!("        part 1: {}", misplaced_sum);
    println!("        part 2: {}", badge_sum);

    Ok(())
}

pub fn run() -> io::Result<()> {
    println!("Day 3:");
    realistic()
}