use md5;

fn just_solve_it(secret_key: &str, fingerprint: &str) -> Option<u32> {
    for i in 0.. {
        let formatted = format!("{}{}", secret_key, i);
        let hash = md5::compute(formatted);
        let str_hash = format!("{:x}", hash);
        if str_hash.starts_with(fingerprint) {
            return Some(i);
        }
    }
    None
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> Option<u32> {
    just_solve_it(input, "00000")
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> Option<u32> {
    just_solve_it(input, "000000")
}
