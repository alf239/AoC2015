use std::collections::HashMap;

#[derive(Clone)]
pub enum NodeDef {
    Value(String),

    Not(String),

    And(String, String),
    Or(String, String),
    Lshift(String, String),
    Rshift(String, String),
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, NodeDef> {
    let mut defs = HashMap::new();
    for l in input.lines() {
        let words: Vec<&str> = l.trim().split(' ').collect();
        match words.len() {
            3 => {
                // 456 -> y
                defs.insert(words[2].to_string(), NodeDef::Value(words[0].to_string()));
            }
            4 => {
                // NOT x -> h
                defs.insert(words[3].to_string(), NodeDef::Not(words[1].to_string()));
            }
            5 => {
                // x AND y -> d
                let a = words[0].to_string();
                let b = words[2].to_string();
                let node = match words[1] {
                    "AND" => NodeDef::And(a, b),
                    "OR" => NodeDef::Or(a, b),
                    "LSHIFT" => NodeDef::Lshift(a, b),
                    "RSHIFT" => NodeDef::Rshift(a, b),
                    _ => panic!("Unknown operation in line \"{}\"", l),
                };
                defs.insert(words[4].to_string(), node);
            }
            _ => panic!("Unknown format in line \"{}\"", l),
        }
    }
    defs
}

fn eval(env: &HashMap<String, NodeDef>, cache: &mut HashMap<String, u16>, v: &str) -> u16 {
    let known = cache.get(v);
    if known.is_some() {
        return *known.unwrap();
    }
    let result = match env.get(v) {
        Some(def) => match def {
            NodeDef::Value(x) => eval(env, cache,  x),
            NodeDef::Not(x) => !eval(env, cache, x),
            NodeDef::And(a, b) => eval(env, cache, a) & eval(env, cache, b),
            NodeDef::Or(a, b) => eval(env, cache, a) | eval(env, cache, b),
            NodeDef::Lshift(a, b) => eval(env, cache, a) << eval(env, cache, b),
            NodeDef::Rshift(a, b) => eval(env, cache, a) >> eval(env, cache, b),
        },
        None => v.parse().unwrap()
    };
    cache.insert(v.to_string(), result);
    result
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &HashMap<String, NodeDef>) -> u16 {
    let mut cache = HashMap::new();
    eval(input, &mut cache, "a")
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &HashMap<String, NodeDef>) -> u16 {
    let a = solve_part1(input);
    let mut input2 = (*input).clone();
    input2.insert("b".to_string(), NodeDef::Value(a.to_string()));
    let mut cache = HashMap::new();
    eval(&input2, &mut cache, "a")
}
