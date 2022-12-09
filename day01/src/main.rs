use std::{str::FromStr, cmp::Reverse};

fn main() {
    let mut calories: Vec<i32> = include_str!("input")
        .split("\n\n")
        .map(|elf|
            elf.lines().flat_map(i32::from_str).sum()
        )
        .collect();

    calories.sort_by_key(|v| Reverse(*v));

    println!("P1: {}", calories[0]);
    println!("P2: {}", calories[0..3].iter().sum::<i32>())
}
