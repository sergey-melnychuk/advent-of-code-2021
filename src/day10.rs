use advent_of_code_2021::util::lines;

fn cost1(chr: &char) -> usize {
    match chr {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn cost2(chr: &char) -> usize {
    match chr {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}

fn score(seq: &[char]) -> usize {
    seq.iter().fold(0usize, |sum, c| sum * 5 + cost2(c))
}

fn mirror(chr: &char) -> char {
    match chr {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => unreachable!(),
    }
}

fn opens(chr: &char) -> bool {
    matches!(chr, '(' | '[' | '{' | '<')
}

fn closes(chr: &char) -> bool {
    opens(&mirror(chr))
}

enum Check {
    Corrupted(char),
    Incomplete(Vec<char>),
    Ok,
}

fn check(line: &str) -> Check {
    let mut stack = Vec::new();

    for c in line.chars() {
        if opens(&c) {
            stack.push(mirror(&c));
        } else if closes(&c) {
            if stack.is_empty() {
                return Check::Corrupted(c);
            }
            if stack.pop().unwrap() != c {
                return Check::Corrupted(c);
            }
        }
    }

    if stack.is_empty() {
        Check::Ok
    } else {
        Check::Incomplete(stack.into_iter().rev().collect())
    }
}

fn main() {
    let input = lines();

    let part1 = input
        .iter()
        .filter_map(|line| match check(line) {
            Check::Corrupted(c) => Some(cost1(&c)),
            _ => None,
        })
        .sum::<usize>();
    println!("{}", part1);

    let mut scores = input
        .iter()
        .flat_map(|line| match check(line) {
            Check::Incomplete(remaining) => Some(score(&remaining)),
            _ => None,
        })
        .collect::<Vec<_>>();
    scores.sort_unstable();

    let mid = scores[scores.len() / 2];
    println!("{}", mid);
}
