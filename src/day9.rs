use std::{collections::HashMap, cmp::{min, max}};

use itertools::Itertools;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> (HashMap<String, usize>, HashMap<(usize, usize), usize>) {
    let mut cities: HashMap<String, usize> = HashMap::new();
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();
    for s in input.lines() {
        let words: Vec<&str> = s.trim().split(' ').collect();
        let i = cities.len();
        let a = *cities.entry(words[0].to_string()).or_insert(i);
        let i = cities.len();
        let b = *cities.entry(words[2].to_string()).or_insert(i);
        map.insert((min(a, b), max(a, b)), words[4].parse().unwrap());
    }
    (cities, map)
}

fn cost(dist: &HashMap<(usize, usize), usize>, route: Vec<usize>) -> usize {
    (1..route.len()).map(|i| {
        let a = route[i - 1];
        let b = route[i];
        let k = (min(a,b), max(a,b));
        dist.get(&k).unwrap()
    }).sum()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &(HashMap<String, usize>, HashMap<(usize, usize), usize>)) -> usize {
    let n = input.0.len();
    let numbers: Vec<usize> = (0..n).collect();
    numbers
        .into_iter()
        .permutations(n)
        .map(|perm| cost(&input.1, perm))
        .min()
        .unwrap()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &(HashMap<String, usize>, HashMap<(usize, usize), usize>)) -> usize {
    let n = input.0.len();
    let numbers: Vec<usize> = (0..n).collect();
    numbers
        .into_iter()
        .permutations(n)
        .map(|perm| cost(&input.1, perm))
        .max()
        .unwrap()
}
