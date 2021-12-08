use advent_of_code_2021::util::input;
use std::collections::HashSet;
use std::str::FromStr;

type Digit = HashSet<char>;
type Num = u64;

#[derive(Debug, Eq, PartialEq)]
struct Observation {
    patterns: Vec<Digit>,
    outputs: Vec<Digit>,
}

impl FromStr for Observation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(" | ");
        let before = it.next().unwrap();
        let after = it.next().unwrap();

        let patterns = before
            .split_whitespace()
            .into_iter()
            .map(|chars| chars.chars().collect())
            .collect();

        let outputs = after
            .split_whitespace()
            .into_iter()
            .map(|chars| chars.chars().collect())
            .collect();

        Ok(Self { patterns, outputs })
    }
}

const TEN: usize = 10;

fn overlap(a: &Digit, b: &Digit) -> usize {
    a.intersection(b).count()
}

fn solve(observation: &Observation) -> Num {
    let mut mapping: [Digit; TEN] = Default::default();

    mapping[1] = observation
        .patterns
        .iter()
        .find(|digit| digit.len() == 2)
        .cloned()
        .unwrap();
    mapping[4] = observation
        .patterns
        .iter()
        .find(|digit| digit.len() == 4)
        .cloned()
        .unwrap();
    mapping[7] = observation
        .patterns
        .iter()
        .find(|digit| digit.len() == 3)
        .cloned()
        .unwrap();
    mapping[8] = observation
        .patterns
        .iter()
        .find(|digit| digit.len() == 7)
        .cloned()
        .unwrap();

    let pattern235 = observation
        .patterns
        .iter()
        .filter(|digit| digit.len() == 5)
        .cloned();
    let pattern069 = observation
        .patterns
        .iter()
        .filter(|digit| digit.len() == 6)
        .cloned();

    let one = mapping[1].clone();
    let four = mapping[4].clone();
    pattern235.map(
            |digit| match (overlap(&digit, &one), overlap(&digit, &four)) {
                (2, _) => (digit, 3),
                (_, 3) => (digit, 5),
                (_, 2) => (digit, 2),
                _ => unreachable!(),
            },
        )
        .for_each(|(digit, number)| {
            mapping[number] = digit;
        });

    pattern069.map(
            |digit| match (overlap(&digit, &one), overlap(&digit, &four)) {
                (_, 4) => (digit, 9),
                (1, 3) => (digit, 6),
                (2, 3) => (digit, 0),
                _ => unreachable!(),
            },
        )
        .for_each(|(digit, number)| {
            mapping[number] = digit;
        });

    observation
        .outputs
        .iter()
        .map(|digit| {
            mapping
                .iter()
                .enumerate()
                .find(|(_, d)| d == &digit)
                .map(|(i, _)| i as Num)
                .unwrap()
        })
        .fold(0, |sum, x| sum * 10 + x)
}

fn main() {
    let input: Vec<Observation> = input();

    let filter_1478 = |len: &usize| -> bool {
        *len == 2    // 1
        || *len == 4 // 4
        || *len == 3 // 7
        || *len == 7 // 8
    };

    let part1 = input
        .iter()
        .flat_map(|obs| obs.outputs.iter().map(|digit| digit.len()))
        .filter(filter_1478)
        .count();
    println!("{}", part1);

    let part2 = input.iter().map(solve).sum::<Num>();
    println!("{}", part2);
}

#[cfg(test)]
mod day08_tests {
    use super::*;

    const INPUT: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    fn observation() -> Observation {
        Observation {
            patterns: vec![
                "acedgfb".chars().collect::<_>(),
                "cdfbe".chars().collect::<_>(),
                "gcdfa".chars().collect::<_>(),
                "fbcad".chars().collect::<_>(),
                "dab".chars().collect::<_>(),
                "cefabd".chars().collect::<_>(),
                "cdfgeb".chars().collect::<_>(),
                "eafb".chars().collect::<_>(),
                "cagedb".chars().collect::<_>(),
                "ab".chars().collect::<_>(),
            ],
            outputs: vec![
                "cdfeb".chars().collect::<_>(),
                "fcadb".chars().collect::<_>(),
                "cdfeb".chars().collect::<_>(),
                "cdbaf".chars().collect::<_>(),
            ],
        }
    }

    #[test]
    fn test_from_str() {
        let actual: Observation = INPUT.parse().unwrap();
        assert_eq!(actual, observation());
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(&observation()), 5353);
    }
}
