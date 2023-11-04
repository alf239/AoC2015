use std::collections::HashSet;

fn visited(input: &str) -> HashSet<(i32, i32)> {
    input
        .chars()
        .scan((0, 0), |s, c| {
            match c {
                '>' => s.0 += 1,
                '<' => s.0 -= 1,
                '^' => s.1 += 1,
                'v' => s.1 -= 1,
                _ => {}
            }
            Some(*s)
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> usize {
    visited(input).len()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut evens = String::new();
    let mut odds = String::new();

    for (index, character) in input.char_indices() {
        if index % 2 == 0 {
            evens.push(character);
        } else {
            odds.push(character);
        }
    }

    let santa = visited(&odds);
    let robo_santa = visited(&evens);

    santa.union(&robo_santa).clone().count()
}
