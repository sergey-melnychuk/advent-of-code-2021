use std::fmt::Debug;
use std::io::{self, BufRead};
use std::str::FromStr;

pub fn lines() -> Vec<String> {
    let stdin = io::stdin();
    stdin.lock().lines().map(|line| line.unwrap()).collect()
}

pub fn input<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    lines()
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect()
}
