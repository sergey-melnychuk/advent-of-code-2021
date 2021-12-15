use advent_of_code_2021::util::lines;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

type Num = usize;

type Grid = Vec<Vec<Num>>;

fn adj(grid: &[Vec<Num>], cell: &(usize, usize)) -> Vec<(usize, usize)> {
    let (rows, cols) = (grid.len(), grid[0].len());
    let (row, col) = cell.to_owned();
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
    adj
}

fn adj_n(grid: &[Vec<Num>], scale: usize, cell: &(usize, usize)) -> Vec<(usize, usize)> {
    let (rows, cols) = (grid.len(), grid[0].len());
    let (row, col) = cell.to_owned();
    let mut adj = Vec::with_capacity(4);
    if row > 0 {
        adj.push((row - 1, col));
    }
    if row < rows * scale - 1 {
        adj.push((row + 1, col));
    }
    if col > 0 {
        adj.push((row, col - 1));
    }
    if col < cols * scale - 1 {
        adj.push((row, col + 1));
    }
    adj
}

#[derive(Debug, Eq, PartialEq)]
struct Entry {
    cell: (usize, usize),
    cost: Num,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.cell.cmp(&other.cell))
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra<F, G>(start: (usize, usize), stop: (usize, usize), risk: F, adj: G) -> Option<Num>
where
    F: Fn((usize, usize)) -> Num,
    G: Fn((usize, usize)) -> Vec<(usize, usize)>,
{
    let mut dist: HashMap<(usize, usize), Num> = HashMap::new();
    let mut queue: BinaryHeap<Entry> = BinaryHeap::new();

    dist.insert(start, 0);
    queue.push(Entry {
        cell: start,
        cost: 0,
    });
    while let Some(Entry { cell, cost }) = queue.pop() {
        if cell == stop {
            return Some(cost);
        }
        if cost > dist[&cell] {
            continue;
        }

        for edge in adj(cell) {
            let step = risk(edge);
            let next = Entry {
                cell: edge,
                cost: cost + step,
            };

            if next.cost < dist.get(&next.cell).cloned().unwrap_or(Num::MAX) {
                dist.insert(next.cell, next.cost);
                queue.push(next);
            }
        }
    }

    None
}

fn main() {
    let grid: Grid = lines()
        .into_iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| c as Num - b'0' as Num)
                .collect()
        })
        .collect();

    let (rows, cols) = (grid.len(), grid[0].len());
    let start = (0, 0);
    let stop = (rows - 1, cols - 1);

    let part1 = dijkstra(
        start,
        stop,
        |(row, col)| grid[row][col],
        |pos| adj(&grid, &pos),
    )
    .unwrap();
    println!("{}", part1);

    let scale = 5;
    let stop = (rows * scale - 1, cols * scale - 1);
    let part2 = dijkstra(
        start,
        stop,
        |(row, col)| {
            let (r, c) = (row % rows, col % cols);
            let extra = row / rows + col / cols;
            let cell = grid[r][c] + extra;
            if cell < 10 {
                cell
            } else {
                cell % 10 + 1
            }
        },
        |pos| adj_n(&grid, scale, &pos),
    )
    .unwrap();
    println!("{}", part2);
}
