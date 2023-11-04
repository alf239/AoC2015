fn nice_string(s: &str) -> bool {
    let bad_strings = ["ab", "cd", "pq", "xy"];
    let vowels = "aeiou";
    let nr_vowels = s.chars().filter(|&c| vowels.contains(c)).count();
    let has_pairs = s.chars().collect::<Vec<char>>().windows(2).any(|pair| pair[0] == pair[1]);
    let bad = bad_strings.iter().any(|bad| s.contains(bad));
    nr_vowels >= 3 && has_pairs && !bad
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|s: &str| s.to_string()).collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Vec<String>) -> usize {
    input.iter().filter(|s| nice_string(s)).count()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Vec<String>) -> usize {
    2
}

#[cfg(test)]
mod tests {
    use super::*; // Bring the outer module's functions into the scope of the tests module

    #[test]
    fn from_task_description() {
        assert_eq!(true, nice_string("ugknbfddgicrmopn"));
        assert_eq!(true, nice_string("aaa"));
        assert_eq!(false, nice_string("jchzalrnumimnmhp"));
        assert_eq!(false, nice_string("haegwjzuvuyypxyu"));
        assert_eq!(false, nice_string("dvszwmarrgswjxmb"));
    }
}
