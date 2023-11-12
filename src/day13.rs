use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> HashMap<String, HashMap<String, i64>> {
    let mut result = HashMap::new();
    input.lines().for_each(|l| {
        let mut attitude = l.trim().split(' ');
        // Alice would gain 2 happiness units by sitting next to Bob.
        let subj = attitude.next().unwrap(); // Alice
        attitude.next(); // would
        let sign = if attitude.next().unwrap() == "gain" {
            1
        } else {
            -1
        }; // gain
        let value: i64 = attitude.next().unwrap().parse().unwrap(); // 2
        attitude.next(); // happiness
        attitude.next(); // units
        attitude.next(); // by
        attitude.next(); // sitting
        attitude.next(); // next
        attitude.next(); // to
        let obj_dot = attitude.next().unwrap();
        let obj = &obj_dot[..obj_dot.len() - 1];

        result
            .entry(subj.to_string())
            .or_insert_with(HashMap::new)
            .insert(obj.to_string(), sign * value);
    });
    result
}

fn cost_1(input: &HashMap<String, HashMap<String, i64>>, a: &str, b: &str) -> i64 {
    input.get(a).unwrap().get(b).unwrap() + input.get(b).unwrap().get(a).unwrap()
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &HashMap<String, HashMap<String, i64>>) -> i64 {
    let guests = input.keys().collect_vec();
    let n = guests.len();
    let head = guests[0];
    let tail = &guests[1..];
    tail.iter()
        .permutations(n - 1)
        .map(|arr| {
            let mut prev = head;
            let last = arr[arr.len() - 1].as_str();
            let mut cost = cost_1(input, last, head);
            for ele in arr {
                cost += cost_1(input, prev, ele);
                prev = ele;
            }
            cost
        })
        .max()
        .unwrap()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &HashMap<String, HashMap<String, i64>>) -> i64 {
    let tail = input.keys().collect_vec();
    tail.iter()
        .permutations(input.len())
        .map(|arr| {
            arr.windows(2)
                .map(|ele| cost_1(input, ele[0], ele[1]))
                .sum()
        })
        .max()
        .unwrap()
}
