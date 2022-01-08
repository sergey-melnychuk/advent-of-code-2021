use advent_of_code_2021::util::lines;

struct Grid {
    rows: usize,
    cols: usize,
    chars: Vec<Vec<char>>,
}

impl Grid {
    fn right(&mut self, row: usize, mut col: usize) -> &mut char {
        if col >= self.cols - 1 {
            col = 0;
        } else {
            col += 1;
        }
        self.get_mut(row, col)
    }

    fn below(&mut self, mut row: usize, col: usize) -> &mut char {
        if row >= self.rows - 1 {
            row = 0;
        } else {
            row += 1;
        }
        self.get_mut(row, col)
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut char {
        assert!(row < self.rows);
        assert!(col < self.cols);
        self.chars.get_mut(row).unwrap().get_mut(col).unwrap()
    }

    fn get(&self, row: usize, col: usize) -> char {
        assert!(row < self.rows);
        assert!(col < self.cols);
        *self.chars.get(row).unwrap().get(col).unwrap()
    }

    fn find(&self, target: char) -> Vec<(usize, usize)> {
        (0..self.rows)
            .into_iter()
            .flat_map(|row| {
                (0..self.cols)
                    .into_iter()
                    .map(move |col| (row, col))
                    .collect::<Vec<_>>()
            })
            .filter(|(row, col)| self.get(*row, *col) == target)
            .collect()
    }

    fn dump(&self) -> String {
        let lines = self
            .chars
            .iter()
            .map(|vec| vec.iter().cloned().collect::<String>())
            .collect::<Vec<_>>();

        lines.join("\n")
    }
}

fn parse(lines: &[String]) -> Grid {
    let rows = lines.len();
    let cols = lines.iter().map(|line| line.len()).max().unwrap();

    let chars = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    Grid { rows, cols, chars }
}

fn step(grid: &mut Grid) -> usize {
    let mut moves = 0;

    let more = grid
        .find('>')
        .into_iter()
        .filter(|(row, col)| *grid.right(*row, *col) == '.')
        .collect::<Vec<_>>();
    for (row, col) in more {
        let next = grid.right(row, col);
        if *next == '.' {
            *next = '>';
            *grid.get_mut(row, col) = '.';
            moves += 1;
        }
    }

    let down = grid
        .find('v')
        .into_iter()
        .filter(|(row, col)| *grid.below(*row, *col) == '.')
        .collect::<Vec<_>>();
    for (row, col) in down {
        let next = grid.below(row, col);
        if *next == '.' {
            *next = 'v';
            *grid.get_mut(row, col) = '.';
            moves += 1;
        }
    }

    moves
}

fn main() {
    let lines = lines();
    let mut grid = parse(&lines);
    println!("\n{}", grid.dump());

    let mut rounds = 0;
    loop {
        let n = step(&mut grid);
        rounds += 1;
        //println!("\n{}", grid.dump());
        if n == 0 {
            break;
        }
        // if rounds > 10 {
        //     break;
        // }
    }

    println!("{}", rounds);
}
