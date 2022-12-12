use pathfinding::prelude::{Matrix, bfs};

fn main() {
    let mut grid: Matrix<u8> = include_str!("input").lines().map(|l| l.bytes()).collect();
    let start = grid.indices().find(|&i| grid[i] == b'S').unwrap();
    let end = grid.indices().find(|&i| grid[i] == b'E').unwrap();
    grid[start] = b'a';
    grid[end] = b'z';

    let successors = |&i: &(usize, usize)|{
        let grid = &grid;
        grid.neighbours(i, false).filter(move |&n| grid[n] >= grid[i] - 1)
    };

    println!(
        "P1: {}",
        bfs(&end, successors, |&i| i == start).unwrap().len() - 1
    );


    println!(
        "P2: {}",
        bfs(&end, successors, |&i| grid[i] == b'a').unwrap().len() - 1
    );
}
