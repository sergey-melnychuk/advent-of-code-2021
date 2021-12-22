use advent_of_code_2021::util::lines;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Dot {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Cube {
    x0: isize,
    x1: isize,
    y0: isize,
    y1: isize,
    z0: isize,
    z1: isize,
}

impl Cube {
    fn dots(&self) -> Vec<Dot> {
        let mut dots = Vec::new();
        for x in self.x0..=self.x1 {
            for y in self.y0..=self.y1 {
                for z in self.z0..=self.z1 {
                    let dot = Dot { x, y, z };
                    dots.push(dot);
                }
            }
        }
        dots
    }

    fn size(&self) -> usize {
        (self.x1 - self.x0 + 1).abs() as usize
            * (self.y1 - self.y0 + 1).abs() as usize
            * (self.z1 - self.z0 + 1).abs() as usize
    }

    fn and(&self, that: &Cube) -> Option<Cube> {
        let (x0, x1) = overlap(self.x0, self.x1, that.x0, that.x1)?;
        let (y0, y1) = overlap(self.y0, self.y1, that.y0, that.y1)?;
        let (z0, z1) = overlap(self.z0, self.z1, that.z0, that.z1)?;
        Some(Cube {
            x0,
            x1,
            y0,
            y1,
            z0,
            z1,
        })
    }
}

fn overlap(a: isize, b: isize, c: isize, d: isize) -> Option<(isize, isize)> {
    let a0 = a.min(b);
    let a1 = a.max(b);

    let b0 = c.min(d);
    let b1 = c.max(d);

    if a0 < b0 && a1 < b0 {
        return None;
    }
    if a0 > b1 && a1 > b1 {
        return None;
    }

    Some((a0.max(b0), a1.min(b1)))
}

fn parse(s: &str) -> (bool, Cube) {
    let mut it = s.split_whitespace();
    let flag_part = it.next().unwrap();

    let mut cube_part = it.next().unwrap().split(',');
    let mut x_range = cube_part
        .next()
        .unwrap()
        .split('=')
        .nth(1)
        .unwrap()
        .split("..");
    let mut y_range = cube_part
        .next()
        .unwrap()
        .split('=')
        .nth(1)
        .unwrap()
        .split("..");
    let mut z_range = cube_part
        .next()
        .unwrap()
        .split('=')
        .nth(1)
        .unwrap()
        .split("..");

    let cube = Cube {
        x0: x_range.next().unwrap().parse().unwrap(),
        x1: x_range.next().unwrap().parse().unwrap(),
        y0: y_range.next().unwrap().parse().unwrap(),
        y1: y_range.next().unwrap().parse().unwrap(),
        z0: z_range.next().unwrap().parse().unwrap(),
        z1: z_range.next().unwrap().parse().unwrap(),
    };

    match flag_part {
        "on" => (true, cube),
        "off" => (false, cube),
        _ => unreachable!(),
    }
}

fn part1(ops: &[(bool, Cube)], init: &Cube) -> usize {
    ops.iter()
        .flat_map(|(flag, cube)| {
            cube.and(init)
                .map(|c| c.dots())
                .unwrap_or_default()
                .into_iter()
                .map(|d| (flag, d))
                .collect::<Vec<_>>()
        })
        .fold(HashSet::new(), |mut set, (on, dot)| {
            if *on {
                set.insert(dot);
            } else {
                set.remove(&dot);
            }
            set
        })
        .len()
}

fn cut_one(a: isize, b: isize, c: isize, d: isize) -> Vec<(isize, isize)> {
    assert!(a <= c);
    assert!(b >= d);

    // match
    // a           b
    // |-----------|
    // c           d
    // |-----------|
    // a           b
    // |-----------|
    if a == c && b == d {
        return vec![(a, b)];
    }

    // lo end
    // a            b
    // |------------|
    // c        d
    // |--------|
    // c        d   b
    // |--------||--|
    if a == c && d < b {
        return vec![(a, d), (d + 1, b)];
    }

    // hi end
    // a            b
    // |------------|
    //     c        d
    // ----|--------|
    // a   c        d
    // |--||--------|
    if c > a && d == b {
        return vec![(a, c - 1), (c, d)];
    }

    // middle
    // a             b
    // |-------------|
    //    c      d
    // ---|------|----
    // a  c      d   b
    // |-||------||--|
    vec![(a, c - 1), (c, d), (d + 1, b)]
}

fn cut(this: &Cube, that: &Cube) -> Vec<Cube> {
    if let Some(that) = this.and(that) {
        let mut cubes = Vec::new();
        for (x0, x1) in cut_one(this.x0, this.x1, that.x0, that.x1) {
            for (y0, y1) in cut_one(this.y0, this.y1, that.y0, that.y1) {
                for (z0, z1) in cut_one(this.z0, this.z1, that.z0, that.z1) {
                    let cube = Cube {
                        x0,
                        x1,
                        y0,
                        y1,
                        z0,
                        z1,
                    };
                    if cube == that {
                        continue;
                    }
                    cubes.push(cube);
                }
            }
        }
        cubes
    } else {
        vec![*this]
    }
}

fn part2(ops: &[(bool, Cube)]) -> usize {
    let mut done: Vec<Cube> = Vec::new();
    for (flag, cube) in ops {
        done = done.into_iter().flat_map(|x| cut(&x, cube)).collect();
        if *flag {
            done.push(*cube);
        }
    }

    done.into_iter().map(|cube| cube.size()).sum()
}

fn main() {
    let ops = lines().into_iter().map(|s| parse(&s)).collect::<Vec<_>>();

    let init = Cube {
        x0: -50,
        x1: 50,
        y0: -50,
        y1: 50,
        z0: -50,
        z1: 50,
    };
    let part1 = part1(&ops, &init);
    println!("{}", part1);

    let part1_ops = ops
        .iter()
        .cloned()
        .flat_map(|(flag, cube)| cube.and(&init).map(|cube| (flag, cube)))
        .collect::<Vec<_>>();
    assert_eq!(part2(&part1_ops), part1);

    println!("{}", part2(&ops));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("on x=-20..26,y=-36..17,z=-47..7"),
            (
                true,
                (Cube {
                    x0: -20,
                    x1: 26,
                    y0: -36,
                    y1: 17,
                    z0: -47,
                    z1: 7
                })
            )
        );
    }

    #[test]
    fn test_size() {
        assert_eq!(
            Cube {
                x0: 0,
                x1: 0,
                y0: 0,
                y1: 0,
                z0: 0,
                z1: 0,
            }
            .size(),
            1
        );

        assert_eq!(
            Cube {
                x0: 0,
                x1: 6,
                y0: 0,
                y1: 4,
                z0: 0,
                z1: 0,
            }
            .size(),
            35
        );
    }
}
