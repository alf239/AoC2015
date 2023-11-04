use md5;

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> Option<u32> {
    for i in 0.. {
        let formatted = format!("{}{}", input, i);
        let hash = md5::compute(formatted);
        let str_hash = format!("{:x}", hash);
        if str_hash.starts_with("00000") {
            return Some(i);
        }
    }
    None
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> usize {
    2
}
