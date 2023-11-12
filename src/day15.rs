use std::cmp::max;

#[derive(Copy, Clone)]
pub struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
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
            let capacity = init(attitude.next().unwrap()).parse().unwrap(); // 3,
            attitude.next(); // durability
            let durability = init(attitude.next().unwrap()).parse().unwrap(); // 0,
            attitude.next(); // flavor
            let flavor = init(attitude.next().unwrap()).parse().unwrap(); // 0,
            attitude.next(); // texture
            let texture = init(attitude.next().unwrap()).parse().unwrap(); // -3,
            attitude.next(); // calories
            let calories = attitude.next().unwrap().parse().unwrap(); // 2
            Ingredient {
                capacity,
                durability,
                flavor,
                texture,
                calories,
            }
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
                let capacity = max(
                    0,
                    a.capacity * i + b.capacity * j + c.capacity * k + d.capacity * l,
                );
                let durability = max(
                    0,
                    a.durability * i + b.durability * j + c.durability * k + d.durability * l,
                );
                let flavor = max(0, a.flavor * i + b.flavor * j + c.flavor * k + d.flavor * l);
                let texture = max(
                    0,
                    a.texture * i + b.texture * j + c.texture * k + d.texture * l,
                );
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
                let capacity = max(
                    0,
                    a.capacity * i + b.capacity * j + c.capacity * k + d.capacity * l,
                );
                let durability = max(
                    0,
                    a.durability * i + b.durability * j + c.durability * k + d.durability * l,
                );
                let flavor = max(0, a.flavor * i + b.flavor * j + c.flavor * k + d.flavor * l);
                let texture = max(
                    0,
                    a.texture * i + b.texture * j + c.texture * k + d.texture * l,
                );
                let calories = max(
                    0,
                    a.calories * i + b.calories * j + c.calories * k + d.calories * l,
                );
                if calories == 500 {
                    watermark = max(capacity * durability * flavor * texture, watermark);
                }
            }
        }
    }
    watermark
}
