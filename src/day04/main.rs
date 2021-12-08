use advent_of_code_2021::util::lines;
use std::collections::HashSet;

type Num = u8;

type Score = u64;

#[derive(Debug)]
struct Board {
    rows: Vec<Vec<Num>>
}

impl Board {
    fn flatten(&self) -> impl Iterator<Item=Num> + '_ {
        self.rows.iter()
            .flatten()
            .cloned()
    }

    fn columns(&self) -> usize {
        self.rows.iter().map(|r| r.len()).max().unwrap()
    }

    fn transpose(&self) -> Board {
        let columns = self.columns();

        let rows = (0..columns).into_iter()
            .map(|i| self.rows.iter()
                .map(|r| r.get(i).unwrap())
                .cloned()
                .collect())
            .collect();

        Board { rows }
    }

    fn sets(&self) -> Vec<HashSet<Num>> {
        let mut result = Vec::with_capacity(self.rows.len() + self.columns());

        let rows: Vec<HashSet<Num>> = self.rows.iter()
            .map(|row| row.iter().cloned().collect())
            .collect();

        let columns: Vec<HashSet<Num>> = self.transpose().rows.iter()
            .map(|row| row.iter().cloned().collect())
            .collect();

        result.extend_from_slice(&rows);
        result.extend_from_slice(&columns);
        result
    }

    fn wins(&self, seq: &[Num]) -> Option<Score> {
        let seen: HashSet<Num> = seq.iter().cloned().collect();

        for set in self.sets() {
            if seen.is_superset(&set) {
                let last = seq.last().unwrap();
                return Some(self.score(seen, *last));
            }
        }

        None
    }

    fn score(&self, seen: HashSet<Num>, last: Num) -> Score {
        let sum = self.flatten().into_iter()
            .filter(|n| !seen.contains(n))
            .map(|n| n as Score)
            .sum::<Score>();

        sum * last as Score
    }
}

fn parse(lines: &[String]) -> (Vec<Num>, Vec<Board>) {
    let mut split = lines.split(|line| line.is_empty());
    let numbers: Vec<Num> = split.next().unwrap().iter().next().unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let boards: Vec<Board> = split.into_iter()
        .map(|rows| rows.iter()
            .map(|row| row
                .split_whitespace()
                .into_iter()
                .map(|n| n.parse().unwrap())
                .collect())
            .collect())
        .map(|rows| Board { rows })
        .collect();

    (numbers, boards)
}

fn wins<'a>(numbers: &'a [Num], boards: &'a [Board]) -> impl Iterator<Item=Score> + 'a {
    (1..numbers.len()).into_iter()
        .flat_map(|turn| {
            let seen = &numbers[0..turn];
            boards.iter()
                .filter(|board| {
                    let prev = &seen[0..seen.len()-1];
                    board.wins(prev).is_none()
                })
                .filter_map(move |board| {
                    board.wins(seen)
                })
        })
}

fn main() {
    let lines = lines();
    let (numbers, boards) = parse(&lines);

    let mut it = wins(&numbers, &boards);
    let score = it.next().unwrap_or_default();
    println!("{}", score);

    let score = it.last().unwrap_or_default();
    println!("{}", score);
}
