fn get_score(s: (&str, &str)) -> i32 {
    let mut score = match s {
        (a, b) if a == b => 3,
        ("A", "B") | ("B", "C") | ("C", "A") => 6,
        _ => 0
    };

    score += match s.1 {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 0
    };

    score
}

fn part1(input: &str) -> i32 {
    input.lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(a, b)| (a, match b {
            "X" => "A",
            "Y" => "B",
            "Z" => "C",
            _ => unreachable!()
        }))
        .map(get_score)
        .sum()
}

fn part2(input: &str) -> i32 {
    input.lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(a, b)|
            (a, match b {
                "Y" => a,
                "X" => match a { "A" => "C", "B" => "A", "C" => "B", _ => "" },
                "Z" => match a { "A" => "B", "B" => "C", "C" => "A", _ => "" },
                _ => unreachable!()
            })
        )
        .map(get_score)
        .sum()
}

fn main() {
    let input = include_str!("input");

    println!("P1: {}", part1(input));
    println!("P2: {}", part2(input))
}
