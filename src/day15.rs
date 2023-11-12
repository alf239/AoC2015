use std::cmp::max;

#[derive(Copy, Clone)]
pub struct Ingredient {
    c: i32,
    d: i32,
    f: i32,
    t: i32,
    kcal: i32,
}

fn init(s: &str) -> String {
    s[..s.len() - 1].to_string()
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Ingredient> {
    input
        .lines()
        .map(|l| {
            let mut attitude = l.trim().split(' ');
            // Sugar: capacity 3, durability 0, flavor 0, texture -3, calories 2
            attitude.next(); // Sugar:
            attitude.next(); // capacity
            let c = init(attitude.next().unwrap()).parse().unwrap(); // 3,
            attitude.next(); // durability
            let d = init(attitude.next().unwrap()).parse().unwrap(); // 0,
            attitude.next(); // flavor
            let f = init(attitude.next().unwrap()).parse().unwrap(); // 0,
            attitude.next(); // texture
            let t = init(attitude.next().unwrap()).parse().unwrap(); // -3,
            attitude.next(); // calories
            let kcal = attitude.next().unwrap().parse().unwrap(); // 2
            Ingredient { c, d, f, t, kcal }
        })
        .collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &Vec<Ingredient>) -> i32 {
    let a = input[0];
    let b = input[1];
    let c = input[2];
    let d = input[3];
    let mut watermark = 0;
    for i in 0..=100 {
        for j in 0..=100 - i {
            for k in 0..=100 - i - j {
                let l = 100 - i - j - k;
                let capacity = max(0, a.c * i + b.c * j + c.c * k + d.c * l);
                let durability = max(0, a.d * i + b.d * j + c.d * k + d.d * l);
                let flavor = max(0, a.f * i + b.f * j + c.f * k + d.f * l);
                let texture = max(0, a.t * i + b.t * j + c.t * k + d.t * l);
                watermark = max(capacity * durability * flavor * texture, watermark);
            }
        }
    }
    watermark
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &Vec<Ingredient>) -> i32 {
    let a = input[0];
    let b = input[1];
    let c = input[2];
    let d = input[3];
    let mut watermark = 0;
    for i in 0..=100 {
        for j in 0..=100 - i {
            for k in 0..=100 - i - j {
                let l = 100 - i - j - k;
                let capacity = max(0, a.c * i + b.c * j + c.c * k + d.c * l);
                let durability = max(0, a.d * i + b.d * j + c.d * k + d.d * l);
                let flavor = max(0, a.f * i + b.f * j + c.f * k + d.f * l);
                let texture = max(0, a.t * i + b.t * j + c.t * k + d.t * l);
                let calories = max(0, a.kcal * i + b.kcal * j + c.kcal * k + d.kcal * l);
                if calories == 500 {
                    watermark = max(capacity * durability * flavor * texture, watermark);
                }
            }
        }
    }
    watermark
}
