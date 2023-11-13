use std::{cmp::min, collections::HashMap};

use itertools::{enumerate, Itertools};

pub struct Sue {
    known: HashMap<String, usize>,
}

fn init(s: &str) -> String {
    s[..s.len() - 1].to_string()
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<Sue> {
    input
        .lines()
        .map(|s| {
            let mut attitude = s.trim().split(' ');
            // Sue 12: pomeranians: 4, akitas: 6, goldfish: 8
            attitude.next(); // Sue
            attitude.next(); // 12:
            let a = init(attitude.next().unwrap()); // pomeranians:
            let va = init(attitude.next().unwrap()).parse().unwrap(); // 4,
            let b = init(attitude.next().unwrap()); // akitas:
            let vb = init(attitude.next().unwrap()).parse().unwrap(); // 6,
            let c = init(attitude.next().unwrap()); // goldfish:
            let vc = attitude.next().unwrap().parse().unwrap(); // 8
            let known = vec![(a, va), (b, vb), (c, vc)].into_iter().collect();
            Sue { known }
        })
        .collect()
}

pub fn check1(sue: &Sue, name: &str, expect: usize) -> bool {
    if let Some(&value) = sue.known.get(name) { if value != expect { return false; } }
    true
}

pub fn checkg(sue: &Sue, name: &str, expect: usize) -> bool {
    if let Some(&value) = sue.known.get(name) { if value <= expect { return false; } }
    true
}

pub fn checkf(sue: &Sue, name: &str, expect: usize) -> bool {
    if let Some(&value) = sue.known.get(name) { if value >= expect { return false; } }
    true
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Vec<Sue>) -> usize {
    for (i, sue) in enumerate(input) {
        if !check1(sue, "children", 3) { continue; }
        if !check1(sue, "cats", 7) { continue; }
        if !check1(sue, "samoyeds", 2) { continue; }
        if !check1(sue, "pomeranians", 3) { continue; }
        if !check1(sue, "akitas", 0) { continue; }
        if !check1(sue, "vizslas", 0) { continue; }
        if !check1(sue, "goldfish", 5) { continue; }
        if !check1(sue, "trees", 3) { continue; }
        if !check1(sue, "cars", 2) { continue; }
        if !check1(sue, "perfumes", 1) { continue; }
        return i + 1;
    }
    panic!("Can't find anything!")
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Vec<Sue>) -> usize {
    for (i, sue) in enumerate(input) {
        if !check1(sue, "children", 3) { continue; }
        if !checkg(sue, "cats", 7) { continue; }
        if !check1(sue, "samoyeds", 2) { continue; }
        if !checkf(sue, "pomeranians", 3) { continue; }
        if !check1(sue, "akitas", 0) { continue; }
        if !check1(sue, "vizslas", 0) { continue; }
        if !checkf(sue, "goldfish", 5) { continue; }
        if !checkg(sue, "trees", 3) { continue; }
        if !check1(sue, "cars", 2) { continue; }
        if !check1(sue, "perfumes", 1) { continue; }
        return i + 1;
    }
    panic!("Can't find anything!")
}
