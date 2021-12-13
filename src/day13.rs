use advent_of_code_2021::util::lines;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Dot(usize, usize);

#[derive(Debug, Copy, Clone)]
enum Fold {
    H(usize),
    V(usize),
}

fn parse(lines: &[String]) -> (Vec<Dot>, Vec<Fold>) {
    let mut top = lines.split(|line| line.is_empty());
    let dots = top
        .next()
        .unwrap()
        .iter()
        .map(|line| {
            let mut xy = line.split(',');
            let row = xy.next().unwrap().parse::<usize>().unwrap();
            let col = xy.next().unwrap().parse::<usize>().unwrap();
            Dot(row, col)
        })
        .collect();

    let folds = top
        .next()
        .unwrap()
        .iter()
        .map(|line| {
            let mut chunks = line.split('=');
            let axis = chunks.next().unwrap().chars().last().unwrap();
            let num = chunks.next().unwrap().parse::<usize>().unwrap();
            if axis == 'x' {
                Fold::H(num)
            } else {
                Fold::V(num)
            }
        })
        .collect();

    (dots, folds)
}

fn fold(dots: &mut HashSet<Dot>, fold: &Fold) {
    let moved: HashSet<Dot> = dots
        .iter()
        .cloned()
        .filter(|dot| match fold {
            Fold::H(x) => dot.0 > *x,
            Fold::V(y) => dot.1 > *y,
        })
        .collect();

    for dot in moved.iter() {
        dots.remove(dot);
    }

    moved
        .into_iter()
        .map(|dot| match fold {
            Fold::H(x) => Dot(x - (dot.0 - x), dot.1),
            Fold::V(y) => Dot(dot.0, y - (dot.1 - y)),
        })
        .for_each(|dot| {
            dots.insert(dot);
        });

    match fold {
        Fold::H(x) => assert!(dots.iter().all(|dot| dot.0 < *x)),
        Fold::V(y) => assert!(dots.iter().all(|dot| dot.1 < *y)),
    };
}

fn print(dots: &HashSet<Dot>) -> Vec<Vec<char>> {
    let cols = dots.iter().map(|d| d.0).max().unwrap();
    let rows = dots.iter().map(|d| d.1).max().unwrap();
    (0..=rows)
        .into_iter()
        .map(move |row| {
            (0..=cols)
                .into_iter()
                .map(move |col| {
                    if dots.contains(&Dot(col, row)) {
                        '#'
                    } else {
                        ' '
                    }
                })
                .collect()
        })
        .collect()
}

fn main() {
    let (dots, folds) = parse(&lines());

    let f = folds[0];
    let mut part1: HashSet<Dot> = dots.iter().cloned().collect();
    fold(&mut part1, &f);
    println!("{}", part1.len());

    let mut part2: HashSet<Dot> = dots.iter().cloned().collect();
    for f in folds {
        fold(&mut part2, &f);
    }

    for row in print(&part2) {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}
