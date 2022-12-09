fn contains_duplicates(s: &[u8]) -> bool {
    s.iter().enumerate().any(|(i, e)|
        s[i+1..].contains(e)
    )
}

fn find_start(input: &[u8], distinct: usize) -> usize {
    input.windows(distinct)
        .enumerate()
        .find(|(_, s)| !contains_duplicates(s))
        .map(|(i, w)| i+w.len())
        .unwrap()
}

fn main() {
    let input = include_bytes!("input");

    println!("P1: {}", find_start(input, 4));
    println!("P2: {}", find_start(input, 14));
}
