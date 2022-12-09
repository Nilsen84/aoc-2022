fn parse_line(l: &str) -> ((i32, i32), (i32, i32)) {
    let (a, b) = l.split_once(',').unwrap();
    let (a, b) = (
        a.split_once('-').unwrap(),
        b.split_once('-').unwrap()
    );

    (
        (a.0.parse().unwrap(), a.1.parse().unwrap()),
        (b.0.parse().unwrap(), b.1.parse().unwrap())
    )
}

fn main() {
    let parsed: Vec<_> = include_str!("input")
        .lines()
        .map(parse_line)
        .collect();

    println!(
        "P1: {}",
        parsed.iter().filter(|((a1, a2), (b1, b2))|
            (a1 >= b1 && a2 <= b2) || (b1 >= a1 && b2 <= a2)
        ).count()
    );

    println!(
        "P2: {}",
        parsed.iter().filter(|((a1, a2), (b1, b2))|
            (a1..=a2).contains(&b1) || (a1..=a2).contains(&b2)
                || (b1..=b2).contains(&a1)
        ).count()
    )
}
