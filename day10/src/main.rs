fn main() {
    let mut x = 1;
    let mut cycle: i32 = 0;
    let mut sum = 0;

    let mut crt = [false; 40*6];

    for insn in include_str!("input").lines() {
        let mut cycle = || {
            if (cycle % 40).abs_diff(x) <= 1 {
                crt[cycle as usize] = true;
            }

            cycle += 1;

            if (cycle - 20) % 40 == 0 {
                sum += cycle * x;
            }
        };

        match insn.split(' ').collect::<Vec<_>>()[..] {
            ["addx", n] => {
                cycle();
                cycle();

                x += n.parse::<i32>().unwrap();
            },
            ["noop"] => cycle(),
            _ => unreachable!()
        }
    }
    
    println!("P1: {sum}");

    crt.chunks(40).for_each(|row|{
        println!("{}", row.iter().map(|&b| if b { '#' } else { '.' }).collect::<String>())
    });
}
