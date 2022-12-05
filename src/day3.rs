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
            sacks,
            badge_id: (0usize..52usize).find(|i| sacks.iter().all(|sack| sack.has_in_either(*i)))
                .unwrap(),
        }
    }
}

pub fn realistic() -> io::Result<()> {
    println!("    Realistic:");

    let sacks: Vec<Sack> = fs::read_to_string("input/day3.txt")?
        .lines()
        .map(Sack::from)
        .collect();
    
    #[allow(clippy::needless_collect)] // allowing for the sake of legibility.
    let groups: Vec<Group> = sacks.chunks(3).map(Group::from).collect();

    let (misplaced_sum, badge_sum) = groups.into_iter().map(|group| {
            let misplaced_sum = group.sacks.iter().map(|sack| {
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
        .reduce(|acc, e| (acc.0 + e.0, acc.1 + e.1))
        .unwrap();

    println!("        part 1: {}", misplaced_sum);
    println!("        part 2: {}", badge_sum);
    println!();

    Ok(())
}

pub fn ugly() -> io::Result<()> {

    println!("    Functional Ugly:");

    let sacks: Vec<Vec<(usize, usize)>> = fs::read_to_string("input/day3.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (left_str, right_str) = line.split_at(line.len() / 2);
            let occurances: Vec<(usize, usize)> = ('a'..='z').chain('A'..='Z')
                .map(|c| (left_str.matches(c).count(), right_str.matches(c).count()))
                .collect();
            occurances
        })
        .collect();
    
    let (misplaced_sum, badge_sum) = sacks.chunks(3)
        .map(|group| {
            let misplaced_priority = group.iter()
                .map(|sack| (0usize..52usize)
                     .map(|i| if sack[i].0 > 0 && sack[i].1 > 0 {
                         1 + i as u32
                     } else {
                         0 
                     })
                     .sum::<u32>())
                .sum::<u32>();
            let badge_priority = (0usize..52usize)
                .find(|i| group.iter().all(|sack| sack[*i].0 > 0 || sack[*i].1 > 0))
                .unwrap();
            (misplaced_priority, 1 + badge_priority as u32)
        })
        .reduce(|acc, e| (acc.0 + e.0, acc.1 + e.1))
        .unwrap();

    println!("        part 1: {}", misplaced_sum);
    println!("        part 2: {}", badge_sum);
    println!();
    
    Ok(())
}

pub fn run() -> io::Result<()> {
    println!("Day 3:");
    realistic().and(ugly())
}
