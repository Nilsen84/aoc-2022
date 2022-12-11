use std::{mem, cmp::Reverse};

#[derive(Debug, Clone)]
enum Op {
    AddOld,
    MulOld,
    Add(i64),
    Mul(i64)
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    op: Op,
    divisible_test: i64,
    if_true: usize,
    if_false: usize
}

impl Monkey {
    fn parse(s: &str) -> Option<Self> {
        let mut lines = s.lines().skip(1);

        let items: Vec<_> = lines.next()?
            .strip_prefix("  Starting items: ")?
            .split(", ")
            .map(|s| s.parse().ok())
            .collect::<Option<_>>()?;
        
        let op = match lines.next()?.strip_prefix("  Operation: new = old ")?.split_once(' ')? {
            ("+", "old") => Op::AddOld,
            ("*", "old") => Op::MulOld,
            ("+", n) => Op::Add(n.parse().ok()?),
            ("*", n) => Op::Mul(n.parse().ok()?),
            _ => return None
        };

        let divisible_by = lines.next()?.strip_prefix("  Test: divisible by ")?.parse().ok()?;
        let if_true = lines.next()?.strip_prefix("    If true: throw to monkey ")?.parse().ok()?;
        let if_false = lines.next()?.strip_prefix("    If false: throw to monkey ")?.parse().ok()?;

        Some(
            Monkey {
                items,
                op,
                divisible_test: divisible_by,
                if_true,
                if_false
            }
        )
    }
}

fn gcd(a: i64, b: i64) -> i64 {
   return if b == 0 { a } else { gcd(b, a%b) };
}

fn monkey_buisness(monkeys: &Vec<Monkey>, rounds: usize, relief: bool) -> u64 {
    let mut monkeys: Vec<_> = monkeys.iter()
        .cloned()
        .map(|m| (m, 0u64))
        .collect();

    let lcm = if !relief {
        monkeys.iter().fold(1, |lcm, (m, _)| {
            lcm * m.divisible_test / gcd(lcm, m.divisible_test)
        })
    } else { i64::MAX };

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for mut item in mem::take(&mut monkeys[i].0.items) {
                monkeys[i].1 += 1;

                match monkeys[i].0.op {
                    Op::AddOld => item += item,
                    Op::MulOld => item *= item,
                    Op::Add(n) => item += n,
                    Op::Mul(n) => item *= n,
                };

                item %= lcm;

                if relief { item /= 3 };

                let to = if item % monkeys[i].0.divisible_test == 0 {
                    monkeys[i].0.if_true
                }else {
                    monkeys[i].0.if_false
                };

                monkeys[to].0.items.push(item);
            }
        }
    }

    monkeys.sort_by_key(|(_, i)| Reverse(*i));
    monkeys[0].1 * monkeys[1].1
}

fn main() {
    let monkeys: Vec<_> = include_str!("input")
        .split("\n\n")
        .map(|s|{
            Monkey::parse(s).unwrap()
        })
        .collect();

    println!("P1: {}", monkey_buisness(&monkeys, 20, true));
    println!("P2: {}", monkey_buisness(&monkeys, 10000, false));
}
