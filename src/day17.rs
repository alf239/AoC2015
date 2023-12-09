use aoc_parse::{parser, prelude::*};

const EGGNOGG: usize = 150;

#[derive(Debug)]
pub struct Task {
    containers: Vec<usize>,
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Task {
    let p = parser!(ctr:lines(usize) => Task { containers: ctr});
    p.parse(input).unwrap()
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &Task) -> i64 {
    let mut dp = [0; EGGNOGG + 1];
    let mut t: [i64; EGGNOGG + 1];
    dp[0] = 1;
    for &c in input.containers.iter() {
        t = dp;
        for i in 0..=EGGNOGG - c {
            if dp[i] > 0 {
                t[i + c] += dp[i];
            }
        }
        dp = t;
    }
    dp[EGGNOGG]
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &Task) -> i64 {
    let mut dp = vec![[0; EGGNOGG + 1]; input.containers.len() + 1];
    dp[0][0] = 1;
    for (k, &c) in input.containers.iter().enumerate() {
        for nc in (0..k).rev() {
            for i in 0..=EGGNOGG - c {
                if dp[nc][i] > 0 {
                    dp[nc + 1][i + c] += dp[nc][i];
                }
            }
        }
    }
    for dp_line in dp.iter() {
        if dp_line[EGGNOGG] != 0 {
            return dp_line[EGGNOGG];
        }
    }
    0
}
