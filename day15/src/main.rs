use itertools::Itertools;
use regex::Regex;

fn parse_input(input: &str) -> Vec<((i32, i32), (i32, i32), u32)> {
    let r = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();

    input.lines()
        .map(|l|{
            let captures = r.captures(l).unwrap();
            let sensor: (i32, i32) = (captures[1].parse().unwrap(), captures[2].parse().unwrap());
            let beacon = (captures[3].parse().unwrap(), captures[4].parse().unwrap());

            (sensor, beacon, sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1))
        })
        .collect()
}

fn merge_ranges(v: &mut Vec<(i32, i32)>) {
    'outer: loop {
        for i in 0..v.len() {
            for j in i+1..v.len() {
                if (v[j].0..=v[j].1).contains(&v[i].0) {
                    v[j].1 = v[j].1.max(v[i].1);
                    v.swap_remove(i);
                    continue 'outer;
                }else if (v[j].0..=v[j].1).contains(&v[i].1) {
                    v[j].0 = v[j].0.min(v[i].0);
                    v.swap_remove(i);
                    continue 'outer;
                }
            }
        }

        return;
    }
}

fn part1(parsed: &Vec<((i32, i32), (i32, i32), u32)>, y: i32) -> i32 {
    let mut ranges = parsed.iter()
        .filter_map(|((sx, sy), _, d)|
            d.checked_sub(sy.abs_diff(y)).map(|len| (sx-len as i32, sx+len as i32))
        )
        .collect::<Vec<_>>();

    merge_ranges(&mut ranges);

    let total_len: i32 = ranges.iter().map(|&(s, e)| (e - s) + 1).sum();

    total_len - parsed.iter()
        .filter(|(_, (_, by), _)| *by == y)
        .unique_by(|(_, (bx, _), _)| *bx)
        .count() as i32
}

fn part2(parsed: &Vec<((i32, i32), (i32, i32), u32)>, search_space: i32) -> Option<i64> {
    for ((sx, sy), _, d) in parsed {
        let d = *d as i32 + 1;

        // go through points on diamond perimeter

        let points = (0..d).map(|y| (d - y, y))
            .flat_map(|(x, y)|
                [(x, y), (-x, -y), (y, x), (-y, -x)]
            )
            .map(|(x, y)| (x + sx, y + sy))
            .filter(|&(x, y)| x >= 0 && y >= 0 && x <= search_space && y <= search_space);
        
        for (x, y) in points {
            if parsed.iter().any(|((sx, sy), _, d)| sx.abs_diff(x) + sy.abs_diff(y) <= *d) {
                continue;
            }

            return Some(x as i64 * 4_000_000 + y as i64);
        }
    }

    None
}

fn main() {
    let parsed = parse_input(include_str!("input"));

    println!("P1: {}", part1(&parsed, 2000000));
    println!("P2: {:?}", part2(&parsed, 4000000).unwrap());
}
