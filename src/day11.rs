use std::cmp::max;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> String {
    input.trim().to_string()
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn step(s: String) -> String {
    let mut result = String::new();
    let mut carry = 1;
    for c in s.chars().into_iter().rev() {
        let c1 = std::char::from_u32((c as u32) + carry).unwrap();
        if c1 > 'z' {
            result.push('a');
            carry = 1;
        } else {
            result.push(c1);
            carry = 0;
        }
    }
    reverse_string(&result)
}

fn good(s: &str) -> bool {
    let mut best_run = 1;
    let mut run = 0;
    let mut prev = '_';
    let mut bad = false;
    for c in s.chars().into_iter() {
        if c == 'i' || c == 'o' || c == 'l' {
            bad = true;
            break; // already bad, whatevs
        }
        if prev != prev_char(c) {
            run = 1;
        } else {
            run += 1;
            best_run = max(run, best_run);
        }
        prev = c;
    }
    let mut has_pair = false;
    let mut pair_at = 0; // index of _last_ element in a pair, 0 for "no pair yet"
    prev = '_';
    for (i, c) in s.chars().enumerate() {
        if prev == c {
            if pair_at > 0 {
                if i - pair_at > 1 {
                    has_pair = true;
                    break;
                }
            } else {
                pair_at = i;
            }
        }
        prev = c;
    }
    (best_run >= 3) && !bad && has_pair
}

fn prev_char(c: char) -> char {
    std::char::from_u32((c as u32) - 1).unwrap()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &String) -> String {
    let mut s = input.clone();
    loop {
        s = step(s);
        if good(&s) {
            return s;
        }
    }
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &String) -> String {
    "".to_string()
}
