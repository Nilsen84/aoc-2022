use rustc_hash::FxHashSet as HashSet;

fn parse_rocks(input: &str) -> HashSet<(i32, i32)> {
    let mut rocks = HashSet::<(i32, i32)>::default();

    input.lines()
        .map(|l|
            l.split(" -> ").map(|c|{
                let (a, b) = c.split_once(',').unwrap();
                (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
            })
        )
        .for_each(|mut line|{
            let mut tracer = line.next().unwrap();
            rocks.insert(tracer);

            for point in line {
                while tracer != point {
                    tracer.0 += (point.0 - tracer.0).signum();
                    tracer.1 += (point.1 - tracer.1).signum();

                    rocks.insert(tracer);
                }
            }
        });

    rocks
}

fn part1(rocks: &HashSet<(i32, i32)>) -> usize {
    let mut occupied = rocks.clone();

    let max_y = occupied.iter().max_by_key(|c| c.1).unwrap().1;

    'spawn_loop: loop  {
        let mut sand = (500, 0);

        'move_loop: loop {
            for p in [(sand.0, sand.1 + 1), (sand.0 - 1, sand.1 + 1), (sand.0 + 1, sand.1 + 1)] {
                if occupied.contains(&p) { continue; };
                if p.1 >= max_y {
                    break 'spawn_loop;
                }
                sand = p;
                continue 'move_loop;
            }
            break;
        }

        occupied.insert(sand);
    }

    occupied.len() - rocks.len()
}

fn part2(rocks: &HashSet<(i32, i32)>) -> usize {
    let mut occupied = rocks.clone();

    let floor = occupied.iter().max_by_key(|c| c.1).unwrap().1 + 2;

    loop  {
        let mut sand = (500, 0);

        'move_loop: loop {
            for p in [(sand.0, sand.1 + 1), (sand.0 - 1, sand.1 + 1), (sand.0 + 1, sand.1 + 1)] {
                if occupied.contains(&p) || p.1 == floor { continue; };
                sand = p;
                continue 'move_loop;
            }
            break;
        }

        occupied.insert(sand);
        if sand == (500, 0) {
            break;
        }
    }

    occupied.len() - rocks.len()
}

fn main() {
    let rocks = parse_rocks(include_str!("input"));

    println!("P1: {}", part1(&rocks));
    println!("P2: {}", part2(&rocks));
}
