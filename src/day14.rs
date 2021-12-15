use advent_of_code_2021::util::lines;
use std::collections::HashMap;

fn parse(lines: &[String]) -> (String, HashMap<(char, char), char>) {
    let mut it = lines.split(|line| line.is_empty());
    let template = it.next().unwrap()[0].to_owned();

    let rules = it
        .next()
        .unwrap()
        .iter()
        .map(|line| {
            let mut it = line.split(" -> ");
            let mut key = it.next().unwrap().chars();
            let a = key.next().unwrap();
            let b = key.next().unwrap();
            let val = it.next().unwrap().chars().next().unwrap();
            ((a, b), val)
        })
        .collect();

    (template, rules)
}

fn expand(input: String, rules: &HashMap<(char, char), char>) -> String {
    let mut result = input.clone();

    let len = input.len();
    let mut k = 0usize;
    for i in 0..len - 1 {
        let a = input.chars().nth(i).unwrap();
        let b = input.chars().nth(i + 1).unwrap();

        rules.get(&(a, b)).into_iter().for_each(|r| {
            result.insert(i + 1 + k, *r);
            k += 1;
        });
    }

    result
}

fn count(line: &str) -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new();
    for c in line.chars() {
        *result.entry(c).or_default() += 1;
    }
    result
}

#[allow(dead_code)] // brute-force solution for part 1
fn solve1(steps: usize, template: &str, rules: &HashMap<(char, char), char>) -> usize {
    let result = (0..steps).fold(template.to_string(), |t, _i| expand(t, rules));

    let counts = count(&result);
    let mut counts = counts.into_iter().collect::<Vec<_>>();
    counts.sort_by_key(|(_, k)| *k);
    counts.reverse();

    let max = counts[0].1;
    let min = counts[counts.len() - 1].1;
    max - min
}

fn solve2(steps: usize, template: &str, rules: &HashMap<(char, char), char>) -> usize {
    let mut pairs_count: HashMap<(char, char), usize> = HashMap::new();
    let len = template.len();
    for i in 0..len - 1 {
        let a = template.chars().nth(i).unwrap();
        let b = template.chars().nth(i + 1).unwrap();
        *pairs_count.entry((a, b)).or_default() += 1;
    }

    let mut chars_count: HashMap<char, usize> = HashMap::new();
    for c in template.chars() {
        *chars_count.entry(c).or_default() += 1;
    }

    for _ in 0..steps {
        let pairs = pairs_count
            .iter()
            .filter(|(_, count)| count > &&0)
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<_>>();
        for (pair, k) in pairs {
            rules.get(&pair).into_iter().cloned().for_each(|r| {
                *chars_count.entry(r).or_default() += k;
                let (a, b) = pair;
                *pairs_count.entry((a, r)).or_default() += k;
                *pairs_count.entry((r, b)).or_default() += k;
                *pairs_count.entry(pair).or_default() -= k;
            });
        }
    }

    let mut counts = chars_count.into_iter().collect::<Vec<_>>();
    counts.sort_by_key(|(_, k)| *k);
    counts.reverse();

    let max = counts[0].1;
    let min = counts[counts.len() - 1].1;
    max - min
}

fn main() {
    let (template, rules) = parse(&lines());

    println!("{}", solve2(10, &template, &rules));
    println!("{}", solve2(40, &template, &rules));
}
