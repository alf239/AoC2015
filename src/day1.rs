#[aoc(day1, part1, Bytes)]
pub fn solve_part1(input: &str) -> i32 {
    input.chars()
    .map(|c| match c {
        '(' => 1, 
        ')' => -1,
        _ => 0,
    })
    .sum()  
}

#[aoc(day1, part2, Bytes)]
pub fn solve_part2(input: &str) -> Option<usize> {
    input.chars()
         .scan(0, |state, c| {
            *state += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };
            Some(*state)
         })
         .position(|sum| sum < 0) 
         .map(|x| x + 1)
}