use std::cmp::min;

type Gift = (u32, u32, u32);

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Gift> {
    input
        .lines()
        .map(|l| {
            let mut gift = l.trim().split('x').map(|d| d.parse().unwrap());
            (
                gift.next().unwrap(),
                gift.next().unwrap(),
                gift.next().unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Gift>) -> u32 {
    input
        .iter()
        .map(|&gift| {
            let (l, w, h) = gift;
            let smallest = min(min(l * w, w * h), h * l);
            2 * l * w + 2 * w * h + 2 * h * l + smallest
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Gift>) -> u32 {
    input
        .iter()
        .map(|&gift| {
            let (l, w, h) = gift;
            let smallest = min(min(l + w, w + h), h + l);
            2 * smallest + l * w * h
        })
        .sum()
}
