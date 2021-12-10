use advent_of_code_2021::util::lines;
use std::collections::{HashSet, VecDeque};

type Grid = Vec<Vec<u8>>;

fn adj(grid: &[Vec<u8>], row: usize, col: usize) -> Vec<(usize, usize)> {
    let (total_rows, total_cols) = (grid.len(), grid[0].len());
    let mut adj = Vec::with_capacity(4);
    if row > 0 {
        adj.push((row - 1, col));
    }
    if row < total_rows - 1 {
        adj.push((row + 1, col));
    }
    if col > 0 {
        adj.push((row, col - 1));
    }
    if col < total_cols - 1 {
        adj.push((row, col + 1));
    }
    adj
}

fn basin(grid: &[Vec<u8>], row: usize, col: usize) -> usize {
    let mut seen = HashSet::new();
    let mut frontier = VecDeque::new();
    frontier.push_back((row, col));
    seen.insert((row, col));
    while !frontier.is_empty() {
        let (row, col) = frontier.pop_front().unwrap();
        seen.insert((row, col));
        let this = grid[row][col];
        for (i, j) in adj(grid, row, col) {
            let next = grid[i][j];
            if !seen.contains(&(i, j)) && next < 9 && this < next {
                frontier.push_back((i, j));
            }
        }
    }
    seen.len()
}

fn main() {
    let grid: Grid = lines()
        .into_iter()
        .map(|line| line.chars().into_iter().map(|c| c as u8 - b'0').collect())
        .collect();
    let (rows, cols) = (grid.len(), grid[0].len());

    let minimums = (0..rows)
        .into_iter()
        .flat_map(|row| (0..cols).into_iter().map(move |col| (row, col)))
        .filter(|(row, col)| {
            let cell = grid[*row][*col];
            adj(&grid, *row, *col)
                .into_iter()
                .all(|(i, j)| grid[i][j] > cell)
        })
        .collect::<Vec<(usize, usize)>>();

    let part1 = minimums
        .iter()
        .map(|(row, col)| grid[*row][*col] as usize + 1)
        .sum::<usize>();
    println!("{}", part1);

    let mut basin_sizes = minimums
        .iter()
        .map(|(row, col)| basin(&grid, *row, *col))
        .collect::<Vec<_>>();
    basin_sizes.sort_unstable();

    let part2 = basin_sizes.iter().rev().take(3).product::<usize>();
    println!("{}", part2);
}
