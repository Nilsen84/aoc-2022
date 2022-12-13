use std::{cmp::Ordering, vec};

#[derive(PartialEq, Eq, Debug, Clone)]
enum PacketData {
    Int(i32),
    List(Vec<PacketData>),
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        use PacketData::*;

        match (self, other) {
            (Int(a), Int(b)) => a.cmp(b),
            (List(a), List(b)) => a.cmp(b),
            (Int(a), List(b)) => [Int(*a)][..].cmp(b),
            (List(a), Int(b)) => a[..].cmp(&[Int(*b)]),
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_packet(s: &mut &str) -> PacketData {
    *s = s.strip_prefix('[').unwrap();

    let mut res = vec![];

    loop {
        match &s[..1] {
            "[" => {
                res.push(parse_packet(s));
            }
            "," => *s = &s[1..],
            "]" => {
                *s = &s[1..];
                break;
            }
            _ => {
                let (i, _) = s.split_once([',', ']']).unwrap();
                *s = &s[i.len()..];
                res.push(PacketData::Int(i.parse().unwrap()))
            }
        }
    }

    PacketData::List(res)
}

fn part1(input: &str) -> i32 {
    input
        .split("\n\n")
        .zip(1..)
        .filter_map(|(p, i)| {
            let (mut a, mut b) = p.split_once('\n').unwrap();
            (parse_packet(&mut a) <= parse_packet(&mut b)).then_some(i)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut v: Vec<_> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|mut l| parse_packet(&mut l))
        .collect();

    let div1 = parse_packet(&mut "[[2]]");
    let div2 = parse_packet(&mut "[[6]]");

    v.push(div1.clone());
    v.push(div2.clone());

    v.sort_unstable();

    (v.iter().position(|p| *p == div1).unwrap() + 1)
        * (v.iter().position(|p| *p == div2).unwrap() + 1)
}

fn main() {
    let input = include_str!("input");

    println!("P1: {}", part1(input));
    println!("P2: {}", part2(input));
}
