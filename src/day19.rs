use advent_of_code_2021::util::lines;
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Default)]
struct V3(isize, isize, isize);

impl Ord for V3 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .cmp(&other.0)
            .then(self.1.cmp(&other.1))
            .then(self.2.cmp(&other.2))
    }
}

impl PartialOrd for V3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl V3 {
    fn sub(&self, that: &V3) -> V3 {
        V3(self.0 - that.0, self.1 - that.1, self.2 - that.2)
    }

    fn add(&self, that: &V3) -> V3 {
        V3(self.0 + that.0, self.1 + that.1, self.2 + that.2)
    }

    fn manhattan(&self, that: &V3) -> isize {
        (self.0 - that.0).abs() + (self.1 - that.1).abs() + (self.2 - that.2).abs()
    }
}

fn parse_vec(s: &str) -> V3 {
    let mut it = s.split(',');
    let x = it.next().unwrap().parse().unwrap();
    let y = it.next().unwrap().parse().unwrap();
    let z = it.next().unwrap().parse().unwrap();
    V3(x, y, z)
}

struct Op {
    map: [(isize, usize); 3],
}

fn lookup(src: &[(isize, usize)], idx: usize) -> (isize, usize) {
    for (i, (sign, pos)) in src.iter().cloned().enumerate() {
        if pos == idx {
            return (sign, i);
        }
    }
    unreachable!()
}

impl Op {
    const fn of(sx: isize, x: usize, sy: isize, y: usize, sz: isize, z: usize) -> Self {
        Self {
            map: [(sx, x), (sy, y), (sz, z)],
        }
    }

    fn undo(&self) -> Self {
        Self {
            map: [
                lookup(&self.map, 0),
                lookup(&self.map, 1),
                lookup(&self.map, 2),
            ],
        }
    }

    fn run(&self, dot: &V3) -> V3 {
        let [(sx, x), (sy, y), (sz, z)] = self.map;
        let dot = [dot.0, dot.1, dot.2];
        V3(dot[x] * sx, dot[y] * sy, dot[z] * sz)
    }
}

fn ops() -> Vec<Op> {
    vec![
        Op::of(1, 0, 1, 1, 1, 2),
        Op::of(1, 2, 1, 1, -1, 0),
        Op::of(-1, 0, 1, 1, -1, 2),
        Op::of(-1, 2, 1, 1, 1, 0),
        Op::of(1, 1, -1, 0, 1, 2),
        Op::of(1, 1, -1, 2, 1, 1),
        Op::of(1, 1, 1, 0, -1, 2),
        Op::of(1, 1, 1, 2, 1, 0),
        Op::of(-1, 1, 1, 0, 1, 2),
        Op::of(-1, 1, 1, 2, -1, 1),
        Op::of(-1, 1, -1, 0, -1, 2),
        Op::of(-1, 1, -1, 2, 1, 0),
        Op::of(-1, 0, -1, 1, 1, 2),
        Op::of(-1, 2, -1, 1, -1, 0),
        Op::of(1, 0, -1, 1, -1, 2),
        Op::of(1, 2, -1, 1, 1, 0),
        Op::of(1, 0, -1, 2, 1, 1),
        Op::of(1, 2, 1, 0, 1, 1),
        Op::of(-1, 0, 1, 2, 1, 1),
        Op::of(-1, 2, -1, 0, 1, 1),
        Op::of(1, 0, 1, 2, -1, 1),
        Op::of(1, 2, -1, 0, -1, 1),
        Op::of(-1, 0, -1, 2, -1, 1),
        Op::of(-1, 2, 1, 0, -1, 1),
    ]
}

#[allow(dead_code)] // for the reference
fn rotations(dot: &V3) -> Vec<V3> {
    vec![
        V3(dot.0, dot.1, dot.2),    // DEFAULT
        V3(dot.2, dot.1, -dot.0),   // DEFAULT + CCW
        V3(-dot.0, dot.1, -dot.2),  // DEFAULT + CCW + CCW
        V3(-dot.2, dot.1, dot.0),   // DEFAULT + CCW + CCW + CCW
        V3(dot.1, -dot.0, dot.2),   // LEFT
        V3(dot.1, -dot.2, -dot.1),  // LEFT + CCW
        V3(dot.1, dot.0, -dot.2),   // LEFT + CCW + CCW
        V3(dot.1, dot.2, dot.1),    // LEFT + CCW + CCW + CCW
        V3(-dot.1, dot.0, dot.2),   // RIGHT
        V3(-dot.1, dot.2, -dot.1),  // RIGHT + CCW
        V3(-dot.1, -dot.0, -dot.2), // RIGHT + CCW + CCW
        V3(-dot.1, -dot.2, dot.0),  // RIGHT + CCW + CCW + CCW
        V3(-dot.0, -dot.1, dot.2),  // BACK
        V3(-dot.2, -dot.1, -dot.0), // BACK + CCW
        V3(dot.0, -dot.1, -dot.2),  // BACK + CCW + CCW
        V3(dot.2, -dot.1, dot.0),   // BACK + CCW + CCW + CCW
        V3(dot.0, -dot.2, dot.1),   // UP
        V3(dot.2, dot.0, dot.1),    // UP + CCW
        V3(-dot.0, dot.2, dot.1),   // UP + CCW + CCW
        V3(-dot.2, -dot.0, dot.1),  // UP + CCW + CCW + CCW
        V3(dot.0, dot.2, -dot.1),   // DOWN
        V3(dot.2, -dot.0, -dot.1),  // DOWN + CCW
        V3(-dot.0, -dot.2, -dot.1), // DOWN + CCW + CCW
        V3(-dot.2, dot.0, -dot.1),  // DOWN + CCW + CCW + CCW
    ]
}

fn align(this: &[V3], that: &[V3]) -> Option<(V3, HashSet<V3>)> {
    for a in this {
        for op1 in ops() {
            let this_set: HashSet<V3> = this
                .iter()
                .cloned()
                .map(|x| x.sub(a))
                .map(|x| op1.run(&x))
                .collect();

            for op2 in ops() {
                for b in that {
                    let that_set: HashSet<V3> = that
                        .iter()
                        .cloned()
                        .map(|x| x.sub(b))
                        .map(|x| op2.run(&x))
                        .collect();

                    let overlap = this_set.intersection(&that_set).count();
                    if overlap >= 12 {
                        let points = that_set
                            .into_iter()
                            .map(|x| op1.undo().run(&x))
                            .map(|x| x.add(a))
                            .collect();
                        let center = {
                            let a = op1.undo().run(a);
                            let b = op2.undo().run(b);
                            a.sub(&b)
                        };
                        return Some((center, points));
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let scanners: Vec<Vec<V3>> = lines()
        .split(|line| line.is_empty())
        .map(|chunk| chunk.iter().skip(1).map(|line| parse_vec(line)).collect())
        .collect();

    let mut done: HashSet<V3> = scanners.get(0).unwrap().iter().cloned().collect();
    let mut todo = scanners.into_iter().skip(1).collect::<Vec<_>>();

    let mut centers = Vec::new();

    while !todo.is_empty() {
        let size = todo.len();
        println!("size={}", size);
        let mut remaining = vec![];
        for scanner in todo {
            let base = done.iter().cloned().collect::<Vec<_>>();
            // base.sort(); // this gets rid of the non-determinism, but gets the wrong answer :)

            if let Some((center, points)) = align(&base, &scanner) {
                println!("\tmatch");
                //println!("\tmatch:\n\t\tscanner={:?}\n\t\tcenter={:?}\n\t\tpoints={:?})", scanner, center, points);
                for p in points {
                    done.insert(p);
                }
                centers.push(center);
            } else {
                remaining.push(scanner);
                println!("\tnope");
            }
        }
        assert!(remaining.len() < size);
        todo = remaining;
    }
    println!("{}", done.len());

    let mut max = isize::MIN;
    for i in 0..centers.len() - 1 {
        for j in i + 1..centers.len() {
            let d = (&centers[i]).manhattan(&centers[j]);
            if d > max {
                println!("i={} j={} d={}", i, j, d);
                max = d;
            }
        }
    }
    println!("{}", max); // expected: 12201 for input dat/19.txt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align() {
        let uno = vec![
            V3(-618, -824, -621),
            V3(-537, -823, -458),
            V3(-447, -329, 318),
            V3(404, -588, -901),
            V3(544, -627, -890),
            V3(528, -643, 409),
            V3(-661, -816, -575),
            V3(390, -675, -793),
            V3(423, -701, 434),
            V3(-345, -311, 381),
            V3(459, -707, 401),
            V3(-485, -357, 347),
        ];

        let dos = vec![
            V3(686, 422, 578),
            V3(605, 423, 415),
            V3(515, 917, -361),
            V3(-336, 658, 858),
            V3(-476, 619, 847),
            V3(-460, 603, -452),
            V3(729, 430, 532),
            V3(-322, 571, 750),
            V3(-355, 545, -477),
            V3(413, 935, -424),
            V3(-391, 539, -444),
            V3(553, 889, -390),
        ];

        assert_eq!(
            align(&uno, &dos).unwrap(),
            (
                V3(68, -1246, -43),
                uno.iter().cloned().collect::<HashSet<_>>(),
            )
        );
    }
}
