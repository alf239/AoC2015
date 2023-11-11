use serde_json::Value;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Value {
    serde_json::from_str(input).unwrap()
}

fn sum_of_ints(input: &Value) -> i64 {
    match input {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Object(vals) => vals.values().map(|v| sum_of_ints(v)).sum(),
        Value::Array(vals) => vals.into_iter().map(|v| sum_of_ints(v)).sum(),
        Value::String(_) => 0
    }
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Value) -> i64 {
    sum_of_ints(input)
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Value) -> i64 {
    2
}
