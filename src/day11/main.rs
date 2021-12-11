use advent_of_code_2021::util::lines;
use std::collections::HashSet;

type Grid = Vec<Vec<u8>>;

fn adj(grid: &[Vec<u8>], row: usize, col: usize) -> Vec<(usize, usize)> {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut adj = Vec::with_capacity(4);
    if row > 0 {
        adj.push((row - 1, col));
    }
    if row < rows - 1 {
        adj.push((row + 1, col));
    }
    if col > 0 {
        adj.push((row, col - 1));
    }
    if col < cols - 1 {
        adj.push((row, col + 1));
    }

    if row > 0 && col > 0 {
        adj.push((row - 1, col - 1));
    }
    if row > 0 && col < cols - 1 {
        adj.push((row - 1, col + 1));
    }
    if row < rows - 1 && col > 0 {
        adj.push((row + 1, col - 1));
    }
    if row < rows - 1 && col < cols - 1 {
        adj.push((row + 1, col + 1));
    }
    adj
}

fn done(grid: &[Vec<u8>]) -> bool {
    grid.iter().all(|row| row.iter().all(|cell| *cell <= 9))
}

fn cells(grid: &[Vec<u8>]) -> Vec<(usize, usize, u8)> {
    let (rows, cols) = (grid.len(), grid[0].len());
    (0..rows)
        .into_iter()
        .flat_map(move |row| {
            (0..cols)
                .into_iter()
                .map(move |col| (row, col, grid[row][col]))
        })
        .collect()
}

fn all(grid: &[Vec<u8>]) -> bool {
    cells(grid).iter().all(|(_, _, cell)| *cell == 0)
}

fn step(grid: &mut Grid) -> usize {
    let mut fired: HashSet<(usize, usize)> = HashSet::new();

    grid.iter_mut()
        .for_each(|row| row.iter_mut().for_each(|cell| *cell += 1));

    while !done(grid) {
        cells(grid)
            .into_iter()
            .filter_map(|(row, col, cell)| if cell > 9 { Some((row, col)) } else { None })
            .for_each(|cell @ (row, col)| {
                fired.insert(cell);
                grid[row][col] = 0;
                adj(grid, row, col).into_iter().for_each(|(row, col)| {
                    grid[row][col] += 1;
                })
            });
    }

    fired.iter().cloned().for_each(|(row, col)| {
        grid[row][col] = 0;
    });
    fired.len()
}

fn main() {
    let mut grid: Grid = lines()
        .into_iter()
        .map(|line| line.chars().into_iter().map(|c| c as u8 - b'0').collect())
        .collect();

    let n = 100;
    let mut part1 = 0usize;
    for _ in 0..n {
        let n = step(&mut grid);
        part1 += n;
    }
    println!("{}", part1);

    let mut part2 = n;
    while !all(&grid) {
        step(&mut grid);
        part2 += 1;
    }
    println!("{}", part2);
}
