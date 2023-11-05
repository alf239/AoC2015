#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|s: &str| s.trim().to_string()).collect()
}

fn unescaped_length(s: &str) -> usize {
    let mut i = 1; // Skip the quote
    let mut l = 0;
    let chars: Vec<char> = s.chars().collect();

    while i < chars.len() {
        match chars.get(i).unwrap() {
            '\\' => match chars.get(i + 1).unwrap() {
                '\\' => {
                    l += 1;
                    i += 2;
                }
                '"' => {
                    l += 1;
                    i += 2;
                }
                'x' => {
                    l += 1;
                    i += 4;
                }
                c => panic!("Invalid escape sequence \\{}", c)
            }
            ,
            '"' => i += 1, // supposedly the closing quote
            _ => {
                i += 1;
                l += 1;
            }
        }
    }

    return l;
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Vec<String>) -> i32 {
    input
        .iter()
        .map(|s| (s.len() as i32) - (unescaped_length(s) as i32))
        .sum()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Vec<String>) -> i32 {
    0
}
