#![feature(iter_collect_into)]
#![feature(get_many_mut)]

use itertools::Itertools;

fn init_stacks(crates: &str) -> Vec<Vec<char>> {
    let mut it = crates.lines().rev();

    let mut stacks = vec![vec![]; it.next().unwrap().split_whitespace().count()];

    for l in it {
        for (i, c) in l.chars().enumerate().filter(|(_, c)| c.is_ascii_alphabetic()) {
            stacks[i / 4].push(c);
        }
    }

    stacks
}

fn execute_instructions(stacks: &Vec<Vec<char>>, instructions: &str, keep_order: bool) -> Vec<Vec<char>> {
    let mut stacks = stacks.clone();

    for insn in instructions.lines() {
        let Some(("move", n, "from", from, "to", to)) = insn.split_whitespace().collect_tuple() else { unreachable!() };
        let n = n.parse::<usize>().unwrap();
        let [from, to] = stacks.get_many_mut([
            from.parse::<usize>().unwrap() - 1,
            to.parse::<usize>().unwrap() - 1
        ]).unwrap();

        if keep_order {
            from.drain(from.len()-n..).collect_into(to);
        }else {
            from.drain(from.len()-n..).rev().collect_into(to);
        }
    }

    stacks
}

fn to_string(stacks: &Vec<Vec<char>>) -> String {
    stacks.iter().flat_map(|s| s.last()).collect::<String>()
}

fn main() {
    let input = include_str!("input");

    let (stacks, instructions) = input.split_once("\n\n").unwrap();
    let stacks = init_stacks(stacks);

    println!("P1: {}", to_string(&execute_instructions(&stacks, instructions, false)));
    println!("P2: {}", to_string(&execute_instructions(&stacks, instructions, true)));
}
