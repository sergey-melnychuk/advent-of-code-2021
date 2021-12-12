use advent_of_code_2021::util::input;
use std::collections::HashMap;
use std::str::FromStr;

type Num = i32;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Dot {
    x: Num,
    y: Num,
}

impl Dot {
    fn of(x: Num, y: Num) -> Self {
        Dot { x, y }
    }
}

#[derive(Debug)]
struct Line {
    at: Dot,
    to: Dot,
}

impl Line {
    fn of(at: Dot, to: Dot) -> Self {
        Line { at, to }
    }

    fn is_vertical(&self) -> bool {
        self.at.x == self.to.x
    }

    fn is_horizontal(&self) -> bool {
        self.at.y == self.to.y
    }

    fn dots(&self) -> Vec<Dot> {
        if self.is_horizontal() {
            assert_eq!(self.at.y, self.to.y);
            let lo = self.at.x.min(self.to.x);
            let hi = self.at.x.max(self.to.x);
            let y = self.at.y;
            return (lo..=hi).into_iter().map(|x| Dot::of(x, y)).collect();
        }

        if self.is_vertical() {
            assert_eq!(self.at.x, self.to.x);
            let lo = self.at.y.min(self.to.y);
            let hi = self.at.y.max(self.to.y);
            let x = self.at.x;
            return (lo..=hi).into_iter().map(|y| Dot::of(x, y)).collect();
        }

        let dx = self.to.x - self.at.x;
        let dy = self.to.y - self.at.y;
        assert_eq!(dx.abs(), dy.abs());

        (0..=dx.abs())
            .into_iter()
            .map(|d| Dot::of(self.at.x + d * dx.signum(), self.at.y + d * dy.signum()))
            .collect()
    }
}

#[allow(dead_code)] // unnecessary for diagonal lines only
fn gcd(a: Num, b: Num) -> Num {
    assert!(a >= b);
    let r = a % b;
    if r == 0 {
        b
    } else {
        gcd(b, r)
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" -> ");
        let mut fst = split.next().unwrap().split(',');
        let x1 = fst.next().unwrap().parse().unwrap();
        let y1 = fst.next().unwrap().parse().unwrap();
        let mut snd = split.next().unwrap().split(',');
        let x2 = snd.next().unwrap().parse().unwrap();
        let y2 = snd.next().unwrap().parse().unwrap();
        Ok(Line::of(Dot::of(x1, y1), Dot::of(x2, y2)))
    }
}

fn solve<'a>(lines: impl Iterator<Item = &'a Line>) -> usize {
    lines
        .flat_map(|line| line.dots())
        .fold(HashMap::new(), |mut map, dot| {
            *map.entry(dot).or_insert(0) += 1;
            map
        })
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .count()
}

fn main() {
    let lines: Vec<Line> = input();

    let n = solve(
        lines
            .iter()
            .filter(|line| line.is_horizontal() || line.is_vertical()),
    );
    println!("{}", n);

    let n = solve(lines.iter());
    println!("{}", n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        let cases = vec![(54, 24, 6)];

        for (a, b, expected) in cases {
            let actual = gcd(a, b);
            assert_eq!(actual, expected, "a={} b={} gcd={}", a, b, expected);
        }
    }

    #[test]
    fn test_dots() {
        let cases = vec![
            (
                Line::of(Dot::of(1, 1), Dot::of(3, 3)),
                vec![Dot::of(1, 1), Dot::of(2, 2), Dot::of(3, 3)],
            ),
            (
                Line::of(Dot::of(9, 7), Dot::of(7, 9)),
                vec![Dot::of(9, 7), Dot::of(8, 8), Dot::of(7, 9)],
            ),
        ];

        for (line, expected) in cases {
            let actual = line.dots();
            assert_eq!(actual, expected);
        }
    }
}
