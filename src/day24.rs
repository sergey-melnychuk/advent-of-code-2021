use advent_of_code_2021::util::lines;
use std::collections::HashMap;

#[derive(Debug, Default, Eq, PartialEq, Hash, Copy, Clone)]
struct Alu {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    at: usize,
}

impl Alu {
    fn get(&mut self, r: &str) -> &mut i64 {
        match r {
            "w" => &mut self.w,
            "x" => &mut self.x,
            "y" => &mut self.y,
            "z" => &mut self.z,
            r => panic!("no such register: {}", r),
        }
    }

    fn arg(&mut self, s: &str) -> i64 {
        if s.chars().next().unwrap().is_alphabetic() {
            *self.get(s)
        } else {
            s.parse::<i64>().unwrap()
        }
    }

    fn end(&self, code: &[String]) -> bool {
        self.at >= code.len()
    }

    fn run(&mut self, code: &[String]) {
        loop {
            if self.end(code) {
                break;
            }
            let op = &code[self.at];
            if op.starts_with("inp") {
                break;
            }
            self.exec(op, 0);
        }
    }

    fn put(&mut self, code: &[String], inp: i64) {
        if self.end(code) {
            return;
        }
        let op = &code[self.at];
        assert!(op.starts_with("inp"));
        self.exec(op, inp);
    }

    fn exec(&mut self, op: &str, inp: i64) {
        let tokens = op.split_whitespace().collect::<Vec<_>>();
        assert!(tokens.len() >= 2);
        match tokens[0] {
            "inp" => {
                assert!(inp > 0 && inp < 10);
                *self.get(tokens[1]) = inp;
            }
            "add" => {
                let b = self.arg(tokens[2]);
                let a = self.get(tokens[1]);
                *a += b;
            }
            "mul" => {
                let b = self.arg(tokens[2]);
                let a = self.get(tokens[1]);
                *a *= b;
            }
            "div" => {
                let b = self.arg(tokens[2]);
                let a = self.get(tokens[1]);
                *a /= b;
            }
            "mod" => {
                let b = self.arg(tokens[2]);
                let a = self.get(tokens[1]);
                *a %= b;
            }
            "eql" => {
                let b = self.arg(tokens[2]);
                let a = self.get(tokens[1]);
                *a = if *a == b { 1 } else { 0 };
            }
            _ => unreachable!(),
        }
        self.at += 1;
    }
}

fn dfs(
    code: &[String],
    init: &Alu,
    seen: &mut HashMap<Alu, Option<usize>>,
    inv: bool,
) -> Option<usize> {
    if let Some(result) = seen.get(init).cloned() {
        return result;
    }

    let mut seq = (1..10).collect::<Vec<_>>();
    if inv {
        seq.reverse();
    }
    for x in seq {
        let mut alu = *init;
        alu.put(code, x as i64);
        alu.run(code);

        if alu.end(code) {
            if alu.z == 0 {
                seen.insert(alu, Some(x));
                return Some(x);
            } else {
                break;
            }
        }

        if let Some(hit) = dfs(code, &alu, seen, inv) {
            let hit = hit * 10 + x;
            seen.insert(alu, Some(hit));
            return Some(hit);
        } else {
            continue;
        }
    }

    seen.insert(*init, None);
    None
}

fn solve(code: &[String], inv: bool) -> usize {
    let mut seen = HashMap::new();
    let alu = Alu::default();
    format!("{}", dfs(code, &alu, &mut seen, inv).unwrap_or_default())
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap()
}

fn main() {
    let code = lines();

    let part1 = solve(&code, true);
    println!("{}", part1);
    // 29599469991739

    let part2 = solve(&code, false);
    println!("{}", part2);
    // 17153114691118
}
