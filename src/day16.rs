use std::collections::HashMap;

use itertools::enumerate;

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

fn check<F>(sue: &Sue, name: &str, predicate: F) -> bool
where
    F: Fn(usize) -> bool,
{
    match sue.known.get(name) {
        Some(&value) => predicate(value),
        _ => true,
    }
}

fn check_eq(sue: &Sue, name: &str, expect: usize) -> bool {
    check(sue, name, |x| x == expect)
}

fn check_gt(sue: &Sue, name: &str, expect: usize) -> bool {
    check(sue, name, |x| x > expect)
}

fn check_ft(sue: &Sue, name: &str, expect: usize) -> bool {
    check(sue, name, |x| x < expect)
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Vec<Sue>) -> usize {
    for (i, sue) in enumerate(input) {
        if check_eq(sue, "children", 3)
            && check_eq(sue, "cats", 7)
            && check_eq(sue, "samoyeds", 2)
            && check_eq(sue, "pomeranians", 3)
            && check_eq(sue, "akitas", 0)
            && check_eq(sue, "vizslas", 0)
            && check_eq(sue, "goldfish", 5)
            && check_eq(sue, "trees", 3)
            && check_eq(sue, "cars", 2)
            && check_eq(sue, "perfumes", 1)
        {
            return i + 1;
        }
    }
    panic!("Can't find anything!")
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Vec<Sue>) -> usize {
    for (i, sue) in enumerate(input) {
        if check_eq(sue, "children", 3)
            && check_gt(sue, "cats", 7)
            && check_eq(sue, "samoyeds", 2)
            && check_ft(sue, "pomeranians", 3)
            && check_eq(sue, "akitas", 0)
            && check_eq(sue, "vizslas", 0)
            && check_ft(sue, "goldfish", 5)
            && check_gt(sue, "trees", 3)
            && check_eq(sue, "cars", 2)
            && check_eq(sue, "perfumes", 1)
        {
            return i + 1;
        }
    }
    panic!("Can't find anything!")
}
