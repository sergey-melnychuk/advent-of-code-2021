use advent_of_code_2021::util::lines;

type Dot = (isize, isize);

#[derive(Clone)]
struct Grid {
    map: String,
    dots: Vec<Vec<char>>,
    around: char,
}

impl Grid {
    #[allow(dead_code)]
    fn dump(&self) -> String {
        let (rows, cols) = self.size();
        let lines = (0..rows)
            .map(|row| {
                (0..cols)
                    .map(|col| self.get(&(row as isize, col as isize)).unwrap())
                    .collect::<String>()
            })
            .collect::<Vec<_>>();
        lines.join("\n")
    }

    fn len(&self) -> usize {
        let (rows, cols) = self.size();
        (0..rows)
            .flat_map(|row| {
                (0..cols).map(move |col| self.get(&(row as isize, col as isize)).unwrap())
            })
            .filter(|c| *c == '#')
            .count()
    }

    fn get(&self, dot: &Dot) -> Option<char> {
        let (row, col) = (dot.0 as usize, dot.1 as usize);
        self.dots.get(row).and_then(|vec| vec.get(col).cloned())
    }

    fn size(&self) -> (usize, usize) {
        (self.dots.len(), self.dots[0].len())
    }

    fn idx(&self, dot: &Dot) -> usize {
        adj(dot)
            .into_iter()
            .map(|dot| self.get(&dot).unwrap_or(self.around))
            .map(|c| if c == '#' { 1 } else { 0 })
            .fold(0usize, |acc, x| (acc << 1) + x)
    }

    fn step(self) -> Self {
        let (rows, cols) = self.size();

        let dots = (0..rows + 2)
            .map(|row| {
                (0..cols + 2)
                    .map(|col| {
                        let dot = ((row - 1) as isize, (col - 1) as isize);
                        let idx = self.idx(&dot);
                        let c = self.map.chars().nth(idx).unwrap();
                        c
                    })
                    .collect()
            })
            .collect();

        Self {
            map: self.map,
            dots,
            around: flip(self.around),
            // around: self.around, // use this line for the example to work
        }
    }
}

fn parse(lines: &[String]) -> Grid {
    let mut it = lines.split(|line| line.is_empty());
    let map = it.next().unwrap()[0].to_owned();
    assert_eq!(map.len(), 512);

    let dots = it
        .next()
        .unwrap()
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Grid {
        map,
        dots,
        around: '.',
    }
}

fn flip(c: char) -> char {
    if c == '#' {
        '.'
    } else {
        '#'
    }
}

fn adj(dot: &Dot) -> Vec<Dot> {
    (-1..=1)
        .flat_map(|drow| (-1..=1).map(move |dcol| (dot.0 + drow, dot.1 + dcol)))
        .collect()
}

fn main() {
    let grid = parse(&lines());

    let part1 = (0..2).fold(grid.clone(), |grid, _| grid.step()).len();
    println!("{}", part1);

    let part2 = (0..50).fold(grid, |grid, _| grid.step()).len();
    println!("{}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adj() {
        assert_eq!(
            adj(&(3, 0)),
            vec![
                (2, -1),
                (2, 0),
                (2, 1),
                (3, -1),
                (3, 0),
                (3, 1),
                (4, -1),
                (4, 0),
                (4, 1)
            ]
        );
    }
}
