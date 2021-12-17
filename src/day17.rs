use advent_of_code_2021::util::lines;
use regex::Regex;

#[derive(Debug, Default, Copy, Clone)]
struct Pos(isize, isize);

#[derive(Debug, Copy, Clone)]
struct Velocity(isize, isize);

impl Pos {
    fn step(&mut self, velocity: &mut Velocity) {
        self.0 += velocity.0;
        self.1 += velocity.1;

        if velocity.0 < 0 {
            velocity.0 += 1;
        }
        if velocity.0 > 0 {
            velocity.0 -= 1;
        }
        velocity.1 -= 1;
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Rect(isize, isize, isize, isize);

impl Rect {
    fn hit(&self, pos: &Pos) -> bool {
        let Rect(x0, x1, y0, y1) = self;
        let Pos(x, y) = pos;
        x0 <= x && x <= x1 && y0 <= y && y <= y1
    }
}

fn parse(line: &str) -> Option<Rect> {
    let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    if let Some(cap) = re.captures_iter(line).next() {
        let a = cap[1].parse().unwrap();
        let b = cap[2].parse().unwrap();
        let c = cap[3].parse().unwrap();
        let d = cap[4].parse().unwrap();
        return Some(Rect(a, b, c, d));
    }
    None
}

#[allow(clippy::many_single_char_names)]
fn main() {
    let target = parse(&lines()[0]).unwrap();

    let Rect(_, b, c, d) = target;
    let t = c.min(d).abs() - 1;
    let h = (t / 2 + t % 2) * t;
    println!("{}", h);

    let mut counter = 0usize;
    for x in 1..=b {
        for y in d * 2..=-d * 2 {
            let mut p = Pos(0, 0);
            let mut v = Velocity(x, y);

            loop {
                p.step(&mut v);
                if target.hit(&p) {
                    counter += 1;
                    break;
                }
                let Rect(a, b, c, d) = target;
                if p.0 > a.max(b) || p.1 < c.min(d) {
                    break;
                }
            }
        }
    }
    println!("{}", counter);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let line = "target area: x=277..318, y=-92..-53";
        assert_eq!(parse(&line), Some(Rect(277, 318, -92, -53)));
    }
}
