#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> String {
    input.trim().to_string()
}

fn step(s: String) -> String {
    let mut last = '*';
    let mut len = 0;
    let mut result = String::new();
    for c in s.chars() {
        if c != last {
            if len > 0 {
                result.push_str(&len.to_string());
                result.push_str(&last.to_string())
            }
            last = c;
            len = 1;
        } else {
            len += 1;
        }
    }
    if len > 0 {
        result.push_str(&len.to_string());
        result.push_str(&last.to_string())
    }
    result
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &String) -> usize {
    let mut s = input.clone();
    for _ in 0..40 {
        s = step(s);
    }
    s.len()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &String) -> usize {
    input.len()
}
