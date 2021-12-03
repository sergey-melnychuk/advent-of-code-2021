use advent_of_code_2021::util::input;
use std::str::FromStr;

enum Move {
    Fwd(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace();
        let name = it.next().unwrap();
        let num: i32 = it.next().unwrap().parse().unwrap();
        match name {
            "up" => Ok(Move::Up(num)),
            "down" => Ok(Move::Down(num)),
            "forward" => Ok(Move::Fwd(num)),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
struct Pos {
    depth: i32,
    length: i32,
}

impl Pos {
    fn step(&mut self, m: &Move) {
        match m {
            Move::Up(x) => self.depth -= x,
            Move::Down(x) => self.depth += x,
            Move::Fwd(x) => self.length += x,
        }
    }
}

#[derive(Default)]
struct Sub {
    aim: i32,
    pos: Pos,
}

impl Sub {
    fn step(&mut self, m: &Move) {
        match m {
            Move::Up(x) => self.aim -= x,
            Move::Down(x) => self.aim += x,
            Move::Fwd(x) => {
                self.pos.length += x;
                self.pos.depth += self.aim * x;
            }
        }
    }
}

fn main() {
    let moves: Vec<Move> = input();

    let mut pos = Pos::default();
    for m in moves.iter() {
        pos.step(m);
    }
    println!("{}", pos.length * pos.depth);

    let mut sub = Sub::default();
    for m in moves.iter() {
        sub.step(m);
    }
    println!("{}", sub.pos.length * sub.pos.depth);
}
