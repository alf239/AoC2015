use std::collections::HashMap;

fn nice_string(s: &str) -> bool {
    let bad_strings = ["ab", "cd", "pq", "xy"];
    let vowels = "aeiou";
    let nr_vowels = s.chars().filter(|&c| vowels.contains(c)).count();
    let has_pairs = s
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .any(|pair| pair[0] == pair[1]);
    let bad = bad_strings.iter().any(|bad| s.contains(bad));
    nr_vowels >= 3 && has_pairs && !bad
}

fn nice_string_2(s: &str) -> bool {
    let mut seen_pairs: HashMap<String, usize> = HashMap::new();
    let mut good_pair = false;
    let chars = &s.chars().collect::<Vec<char>>();
    for i in 1..chars.len() {
        let pair: String = chars[
            i - 1..i + 1].iter().collect();
        match seen_pairs.get(&pair) {
            Some(&prev) if prev < i - 1 => {
                good_pair = true;
                break;
            }
            Some(_) => {}
            None => {
                seen_pairs.insert(pair, i);
            }
        }
    }
    let has_pairs = chars.windows(3).any(|pair| pair[0] == pair[2]);
    good_pair && has_pairs
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
    input.iter().filter(|s| nice_string_2(s)).count()
}

#[cfg(test)]
mod tests {
    use super::*; // Bring the outer module's functions into the scope of the tests module

    #[test]
    fn part_1_description() {
        assert_eq!(true, nice_string("ugknbfddgicrmopn"));
        assert_eq!(true, nice_string("aaa"));
        assert_eq!(false, nice_string("jchzalrnumimnmhp"));
        assert_eq!(false, nice_string("haegwjzuvuyypxyu"));
        assert_eq!(false, nice_string("dvszwmarrgswjxmb"));
    }

    #[test]
    fn part_2_description() {
        assert_eq!(true, nice_string_2("qjhvhtzxzqqjkmpb"));
        assert_eq!(true, nice_string_2("xxyxx"));
        assert_eq!(false, nice_string_2("uurcxstgmygtbstg"));
        assert_eq!(false, nice_string_2("ieodomkazucvgmuy"));
    }
}
