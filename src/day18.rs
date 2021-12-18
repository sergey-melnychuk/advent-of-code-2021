use advent_of_code_2021::util::lines;
use std::str::FromStr;

type Num = isize;

#[derive(Debug, Eq, PartialEq)]
enum Tree {
    Leaf(Num),
    Node(Box<Tree>, Box<Tree>),
}

impl FromStr for Tree {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<_>>();
        let mut pos = 0;
        Ok(match_tree(&chars, &mut pos).unwrap())
    }
}

fn match_tree(buf: &[char], pos: &mut usize) -> Option<Tree> {
    if buf[*pos] != '[' {
        return None;
    }
    *pos += 1;

    let lhs: Tree = if buf[*pos].is_ascii_digit() {
        let val = buf[*pos] as Num - '0' as Num;
        Tree::Leaf(val)
    } else {
        match_tree(buf, pos)?
    };

    *pos += 1;
    if buf[*pos] != ',' {
        return None;
    }
    *pos += 1;

    let rhs: Tree = if buf[*pos].is_ascii_digit() {
        let val = buf[*pos] as Num - '0' as Num;
        Tree::Leaf(val)
    } else {
        match_tree(buf, pos)?
    };

    *pos += 1;
    if buf[*pos] != ']' {
        return None;
    }

    Some(Tree::Node(Box::new(lhs), Box::new(rhs)))
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Entry {
    depth: usize,
    value: Num,
}

fn parse(s: &str) -> Vec<Entry> {
    let mut entries = Vec::new();
    let mut depth = 0;
    for char in s.chars() {
        match char {
            '[' => depth += 1,
            ']' => depth -= 1,
            x if x.is_ascii_digit() => {
                let value = x as Num - b'0' as Num;
                entries.push(Entry { depth, value });
            }
            ',' => (),
            _ => unreachable!(),
        }
    }
    entries
}

fn find_to_explode(entries: &[Entry]) -> Option<usize> {
    for i in 0..entries.len() - 1 {
        if entries[i].depth == 5 && entries[i + 1].depth == 5 {
            return Some(i);
        }
    }
    None
}

fn find_to_split(entries: &[Entry]) -> Option<usize> {
    entries
        .iter()
        .enumerate()
        .find(|(_, entry)| entry.value > 9)
        .map(|(i, _)| i)
}

fn can_reduce(entries: &[Entry]) -> bool {
    find_to_explode(entries)
        .or_else(|| find_to_split(entries))
        .is_some()
}

fn explode(mut entries: Vec<Entry>, i: usize) -> Vec<Entry> {
    let a = entries.remove(i);
    let b = entries.remove(i);

    if i > 0 {
        entries[i - 1].value += a.value;
    }
    if i < entries.len() {
        entries[i].value += b.value;
    }

    let depth = a.depth - 1;
    entries.insert(i, Entry { depth, value: 0 });
    entries
}

fn split(mut entries: Vec<Entry>, i: usize) -> Vec<Entry> {
    let entry = entries.remove(i);
    let depth = entry.depth + 1;
    let lhs = entry.value / 2;
    let rhs = entry.value - lhs;
    entries.insert(i, Entry { depth, value: rhs });
    entries.insert(i, Entry { depth, value: lhs });
    entries
}

fn reduce_once(entries: Vec<Entry>) -> Vec<Entry> {
    if let Some(i) = find_to_explode(&entries) {
        explode(entries, i)
    } else if let Some(i) = find_to_split(&entries) {
        split(entries, i)
    } else {
        entries
    }
}

fn reduce(mut entries: Vec<Entry>) -> Vec<Entry> {
    while can_reduce(&entries) {
        entries = reduce_once(entries);
    }
    entries
}

fn add(lhs: Vec<Entry>, rhs: Vec<Entry>) -> Vec<Entry> {
    let mut sum = Vec::with_capacity(lhs.len() + rhs.len());
    for mut entry in lhs {
        entry.depth += 1;
        sum.push(entry);
    }
    for mut entry in rhs {
        entry.depth += 1;
        sum.push(entry);
    }
    sum
}

fn reduce_magnitude(entries: Vec<Entry>) -> Vec<Entry> {
    if entries.len() == 1 {
        return entries;
    }
    let n = entries.len();
    let max_depth = entries.iter().map(|e| e.depth).max().unwrap();

    let mut reduced = Vec::new();
    let mut i = 0;
    while i < n {
        if entries[i].depth == max_depth && entries[i + 1].depth == max_depth {
            reduced.push(Entry {
                depth: max_depth - 1,
                value: 3 * entries[i].value + 2 * entries[i + 1].value,
            });
            i += 1;
        } else {
            reduced.push(entries[i].clone());
        }
        i += 1;
    }
    reduced
}

fn magnitude(mut entries: Vec<Entry>) -> Num {
    while entries.len() > 1 {
        let len = entries.len();
        entries = reduce_magnitude(entries);
        assert!(entries.len() < len);
    }
    entries[0].value
}

fn main() {
    let numbers = lines()
        .into_iter()
        .map(|line| parse(&line))
        .collect::<Vec<_>>();

    let part1 = numbers
        .iter()
        .cloned()
        .reduce(|lhs, rhs| reduce(add(lhs, rhs)))
        .map(magnitude)
        .unwrap();
    println!("{}", part1);

    let mut part2 = Num::MIN;
    let n = numbers.len();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let a = numbers[i].clone();
            let b = numbers[j].clone();
            let mag = magnitude(reduce(add(a, b)));
            part2 = part2.max(mag);
        }
    }
    println!("{}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_tree() {
        let line = "[[[1,2],[3,4]],5]";
        let tree: Tree = line.parse().unwrap();
        assert_eq!(
            tree,
            Tree::Node(
                Box::new(Tree::Node(
                    Box::new(Tree::Node(Box::new(Tree::Leaf(1)), Box::new(Tree::Leaf(2)),)),
                    Box::new(Tree::Node(Box::new(Tree::Leaf(3)), Box::new(Tree::Leaf(4)),)),
                )),
                Box::new(Tree::Leaf(5)),
            )
        );
    }

    #[test]
    fn test_parse() {
        let cases = vec![
            (
                "[[[1,2],[3,4]],5]",
                vec![
                    Entry { depth: 3, value: 1 },
                    Entry { depth: 3, value: 2 },
                    Entry { depth: 3, value: 3 },
                    Entry { depth: 3, value: 4 },
                    Entry { depth: 1, value: 5 },
                ],
            ),
            (
                "[[6,[5,[4,[3,2]]]],1]",
                vec![
                    Entry { depth: 2, value: 6 },
                    Entry { depth: 3, value: 5 },
                    Entry { depth: 4, value: 4 },
                    Entry { depth: 5, value: 3 },
                    Entry { depth: 5, value: 2 },
                    Entry { depth: 1, value: 1 },
                ],
            ),
        ];

        for (src, vec) in cases {
            assert_eq!(parse(src), vec);
        }
    }

    #[test]
    fn test_reduce() {
        let cases = vec![
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ];

        for (src, dst) in cases {
            let expected = parse(dst);
            let actual = reduce_once(parse(src));
            assert_eq!(actual, expected, "'{}' -> '{}", src, dst);
        }
    }

    #[test]
    fn test_magnitude() {
        let cases = vec![
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ];

        for (src, expected) in cases {
            let actual = magnitude(parse(src));
            assert_eq!(actual, expected, "{}", src);
        }
    }
}
