#![feature(iter_array_chunks)]

fn get_priority(c: char) -> u8 {
    match c {
        'a'..='z' => (c as u8) - b'a' + 1,
        'A'..='Z' => (c as u8) - b'A' + 27,
        _ => unreachable!()
    }
}

fn part1(input: &str) -> i32 {
    input.lines()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(a, b)|
            a.chars().find(|c| b.contains(*c)).unwrap()
        )
        .map(|c| get_priority(c) as i32)
        .sum::<i32>()
}

fn part2(input: &str) -> i32 {
    input.lines()
        .array_chunks::<3>()
        .map(|a|
            a[0].chars()
                .find(|c|
                    a[1..].iter().all(|s| s.contains(*c))
                )
                .unwrap()
        )
        .map(|c| get_priority(c) as i32)
        .sum::<i32>()
}

fn main() {
    let input = include_str!("input");

    println!("P1: {}", part1(input));
    println!("P2: {}", part2(input));
}
