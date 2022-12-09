use std::collections::HashSet;

fn simulate_snake<const N: usize>(movement: &str) -> i32 {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut snake = [(0i32, 0i32); N];

    for line in movement.lines() {
        let (dir, steps) = line.split_once(' ').unwrap();
        let steps: i32 = steps.parse().unwrap();

        for _ in 0..steps {
            match dir {
                "U" => snake[0].0 += 1,
                "D" => snake[0].0 -= 1,
                "R" => snake[0].1 += 1,
                "L" => snake[0].1 -= 1,
                _ => unreachable!()
            };

            for i in 1..N {
                let curr = snake[i];
                let next = snake[i-1];

                if next.0.abs_diff(curr.0) > 1 || next.1.abs_diff(curr.1) > 1 {
                    snake[i].0 += (next.0-curr.0).signum();
                    snake[i].1 += (next.1-curr.1).signum();
                } else {
                    break;
                }
            }

            visited.insert(snake[N - 1]);
        }
    }

    visited.len() as i32
}

fn main() {
    let input = include_str!("input");

    println!("P1: {}", simulate_snake::<2>(input));
    println!("P2: {}", simulate_snake::<10>(input));
}
