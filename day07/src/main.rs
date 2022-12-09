use std::{collections::HashMap};

fn main() {
    let mut curr = Vec::<&str>::new();
    let mut sizes = HashMap::<Vec<&str>, i32>::new();
    sizes.insert(vec![], 0);

    for line in include_str!("input").lines() {
        match line.split_whitespace().collect::<Vec<_>>()[..] {
            ["$", "cd", "/"] => curr.clear(),
            ["$", "cd", ".."] => { curr.pop().unwrap(); },
            ["$", "cd", dir] => {
                curr.push(dir);
                sizes.insert(curr.clone(), 0);
            },
            ["dir" | "$", ..] => (),
            [size, _] => {
                let size = size.parse::<i32>().unwrap();

                (0..=curr.len()).for_each(|l|{
                    *sizes.get_mut(&curr[0..l]).unwrap() += size;
                });
            },
            _ => ()
        };
    }

    println!("P1: {}", sizes.values().filter(|s| **s <= 100_000).sum::<i32>());

    let used_space = sizes[&vec![]];
    let need_to_delete = used_space - (70_000_000 - 30_000_000);
    println!("P2: {}", sizes.values().filter(|s| **s >= need_to_delete).min().unwrap());
}
