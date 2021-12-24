use std::collections::HashMap;
use advent_of_code_2021::util::lines;

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
            r => panic!("no such register: {}", r)
        }
    }

    fn arg(&mut self, s: &str) -> i64 {
        if let Ok(x) = s.parse::<i64>() {
            x
        } else {
            *self.get(s)
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
            },
            "add"  => {
                let b = self.arg(tokens[2]);
                let a = self.get(tokens[1]);
                *a += b;
            },
            "mul" => {
                let b = self.arg(tokens[2]);
                let a = self.get(tokens[1]);
                *a *= b;
            },
            "div" => {
                let b = self.arg(tokens[2]);
                let a = self.get(tokens[1]);
                *a /= b;
            },
            "mod" => {
                let b = self.arg(tokens[2]);
                let a = self.get(tokens[1]);
                *a %= b;
            },
            "eql" => {
                let b = self.arg(tokens[2]);
                let a = self.get(tokens[1]);
                *a = if *a == b {1} else {0};
            },
            _ => unreachable!()
        }
        self.at += 1;
    }
}

// inspiration: https://github.com/emilyskidsister/aoc/blob/main/p2021_24/src/lib.rs
fn dfs(code: &[String], mut alu: Alu, seen: &mut HashMap<Alu, Option<usize>>) -> Option<usize> {
    println!("{:?}", alu);
    if let Some(result) = seen.get(&alu).cloned() {
        return result;
    }

    for inp in (1..10).rev() {
        while !alu.end(code) {
            alu.put(code, inp);
            alu.run(code);

            if let Some(best) = dfs(code, alu, seen) {
                let next = best * 10 + inp as usize;
                seen.insert(alu, Some(next));
            } else {
                break;
            }
        }

        if alu.z == 0 {
            seen.insert(alu, Some(inp as usize));
            println!("{}", inp);
            return Some(inp as usize);
        }
    }

    seen.insert(alu, None);
    None
}

fn main() {
    let code = lines();

    let mut seen = HashMap::new();
    let part1 = dfs(&code, Alu::default(), &mut seen).unwrap_or_default();
    println!("{}", part1);

    // 29599469991739
    // 17153114691118
}
