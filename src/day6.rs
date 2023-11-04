use std::cmp::max;

use bit_vec::BitVec;

pub type Coords =  (usize, usize, usize, usize);

pub enum Command {
    TurnOn(Coords),
    TurnOff(Coords),
    Toggle(Coords),
}

fn parse_coords(s1: &str, s2: &str) -> Coords {
    let mut c1 = s1.trim().split(',').map(|d| d.parse().unwrap());
    let xf = c1.next().unwrap();
    let yf = c1.next().unwrap();

    let mut c2 = s2.trim().split(',').map(|d| d.parse().unwrap());
    let xt = c2.next().unwrap();
    let yt = c2.next().unwrap();

    (xf,yf,xt,yt)
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|l| {
            let words: Vec<&str> = l.trim().split(' ').collect();
            match words[0] {
                "toggle" => {
                    Command::Toggle(parse_coords(words[1], words[3]))
                }
                "turn" => {
                    if words[1] == "on" {
                        Command::TurnOn(parse_coords(words[2], words[4]))
                    } else {
                        Command::TurnOff(parse_coords(words[2], words[4]))
                    }
                }
                _ => panic!("Oopsie!")
            }
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Vec<Command>) -> usize {
    let n = 1000;
    let mut map = BitVec::from_elem(n * n, false);
    for cmd in input {
        match cmd {
            Command::TurnOn((xf, yf, xt, yt)) => {
                for y in *yf..=*yt {
                    for x in *xf..=*xt {
                        map.set(y * n + x, true);
                    }
                }
            },
            Command::TurnOff((xf, yf, xt, yt)) => {
                for y in *yf..=*yt {
                    for x in *xf..=*xt {
                        map.set(y * n + x, false);
                    }
                }
            },
            Command::Toggle((xf, yf, xt, yt)) => {
                for y in *yf..=*yt {
                    for x in *xf..=*xt {
                        map.set(y * n + x, !map.get(y * n + x).unwrap());
                    }
                }
            },
        }
    }
    let mut i = 0;
    for y in 0..n {
        for x in 0..n {
            if map.get(y * n + x).unwrap() {
                i += 1;
            }
        }
    }
    i
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Vec<Command>) -> i32 {
    let n = 1000;
    let mut grid: Vec<Vec<i32>> = vec![vec![0; n]; n];
    for cmd in input {
        match cmd {
            Command::TurnOn((xf, yf, xt, yt)) => {
                for y in *yf..=*yt {
                    for x in *xf..=*xt {
                        grid[y][x] = grid[y][x] + 1;
                    }
                }
            },
            Command::TurnOff((xf, yf, xt, yt)) => {
                for y in *yf..=*yt {
                    for x in *xf..=*xt {
                        grid[y][x] = max(grid[y][x] - 1, 0);
                    }
                }
            },
            Command::Toggle((xf, yf, xt, yt)) => {
                for y in *yf..=*yt {
                    for x in *xf..=*xt {
                        grid[y][x] = grid[y][x] + 2;
                    }
                }
            },
        }
    }
    grid.iter().map(|row| row.iter().sum::<i32>()).sum()
}
