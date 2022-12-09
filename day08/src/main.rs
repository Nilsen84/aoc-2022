use take_until::TakeUntilExt;

fn part1(grid: &Vec<Vec<u8>>) -> i32 {
    let mut count = 0;

    for (y, r) in grid.iter().enumerate() {
        for(x, height) in r.iter().enumerate() {
            let is_visible = (0..y).all(|i| grid[i][x] < *height)
                || (y+1..grid.len()).all(|i| grid[i][x] < *height)
                || (0..x).all(|i| grid[y][i] < *height)
                || (x+1..grid[0].len()).all(|i| grid[y][i] < *height);

            if is_visible {
                count += 1;
            }
        }
    }

    count
}

fn part2(grid: &Vec<Vec<u8>>) -> i32 {
    let mut max_score = 0;

    for (y, r) in grid.iter().enumerate() {
        for(x, height) in r.iter().enumerate() {
            let score = (0..y).rev().take_until(|i| grid[*i][x] >= *height).count() *
                (y+1..grid.len()).take_until(|i| grid[*i][x] >= *height).count() *
                (0..x).rev().take_until(|i| grid[y][*i] >= *height).count() *
                (x+1..grid[0].len()).take_until(|i| grid[y][*i] >= *height).count();

            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score as i32
}


fn main() {
    let input = include_str!("input");

    let grid: Vec<Vec<u8>> = input.lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect();
        
    println!("P1: {}", part1(&grid));
    println!("P2: {}", part2(&grid));
}
