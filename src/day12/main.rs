use std::collections::HashMap;
use std::str::FromStr;
use advent_of_code_2021::util::input;

#[derive(Debug, Clone)]
struct Edge(String, String);

type Graph = HashMap<String, Vec<String>>;

impl FromStr for Edge {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split("-");
        let src = it.next().unwrap().to_string();
        let dst = it.next().unwrap().to_string();
        Ok(Edge(src, dst))
    }
}

fn graph(edges: &[Edge]) -> Graph {
    let mut graph: Graph = HashMap::new();
    for Edge(src, dst) in edges.iter().cloned() {
        graph.entry(src.clone()).or_default().push(dst.clone());
        graph.entry(dst).or_default().push(src);
    }
    graph
}

fn is_small(node: &str) -> bool {
    node.chars().all(|c| c.is_lowercase())
}

fn is_large(node: &str) -> bool {
    node.chars().all(|c| c.is_uppercase())
}

fn is_valid1(node: &str, path: &[String]) -> bool {
    is_large(node) || (node != "start" && path.iter().all(|n| n != node))
}

fn is_valid2(node: &str, path: &[String]) -> bool {
    is_large(node)
    || node != "start"
    && {
        let count = counts(path);
        // 1. path does not contain this small node
        !count.contains_key(node)
        // 2. path contains this small node only once AND any other small node only once
        || count.into_iter()
            .filter(|(n, _)| is_small(n))
            .all(|(_, k)| k == 1)
    }
}

fn counts(path: &[String]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for node in path.iter().cloned() {
        *counts.entry(node).or_default() += 1;
    }
    counts
}

fn dfs<F>(graph: &Graph, node: &str, path: Vec<String>, hits: &mut usize, is_valid: F)
    where
        F: Fn(&str, &[String]) -> bool + Copy
{
    let mut path = path.clone();
    path.push(node.to_string());
    for next in &graph.get(node).cloned().unwrap_or_default() {
        if next == "end" {
            *hits += 1;
        } else if is_valid(next, &path) {
            dfs(graph, next, path.clone(), hits, is_valid);
        }
    }
}

fn main() {
    let edges: Vec<Edge> = input();
    let graph = graph(&edges);

    let mut part1 = 0usize;
    dfs(&graph, "start", Vec::new(), &mut part1, is_valid1);
    println!("{}", part1);

    let mut part2 = 0usize;
    dfs(&graph, "start", Vec::new(), &mut part2, is_valid2);
    println!("{}", part2);
}