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
pub fn solve_part1(input: &Vec<Ingredient>) -> i64 {
    1
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &Vec<Ingredient>) -> i64 {
    2
}
