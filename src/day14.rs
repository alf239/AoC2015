use std::cmp::min;

use itertools::Itertools;

pub struct Deer {
    speed: i64,
    streak: i64,
    cooldown: i64,
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Deer> {
    input
        .lines()
        .map(|l| {
            let mut attitude = l.trim().split(' ');
            // Rudolph can fly 22 km/s for 8 seconds, but then must rest for 165 seconds.
            attitude.next(); // Rudolph
            attitude.next(); // can
            attitude.next(); // fly
            let speed: i64 = attitude.next().unwrap().parse().unwrap(); // 22
            attitude.next(); // km/s
            attitude.next(); // for
            let streak: i64 = attitude.next().unwrap().parse().unwrap(); // 8
            attitude.next(); // seconds,
            attitude.next(); // but
            attitude.next(); // then
            attitude.next(); // must
            attitude.next(); // rest
            attitude.next(); // for
            let cooldown: i64 = attitude.next().unwrap().parse().unwrap(); // 165
            attitude.next(); // seconds
            Deer {
                speed,
                streak,
                cooldown,
            }
        })
        .collect_vec()
}

fn distance(d: &Deer, t: i64) -> i64 {
    let cycle = d.streak + d.cooldown;
    let full_cycles = t / cycle;
    let remainder = t % cycle;
    let flight = full_cycles * d.streak + min(d.streak, remainder);
    flight * d.speed
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &Vec<Deer>) -> i64 {
    input.iter().map(|d| distance(d, 2503)).max().unwrap()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &Vec<Deer>) -> i64 {
    2
}
