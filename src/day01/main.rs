use advent_of_code_2021::util::lines;

fn input() -> Vec<i64> {
    lines()
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn main() {
    let input = input();

    let mut inc = 0;
    for (a, b) in input.iter().zip(input.iter().skip(1)) {
        if b > a {
            inc += 1;
        }
    }

    println!("{}", inc);

    inc = 0;
    for (a, b) in input.windows(3).zip(input.windows(3).skip(1)) {
        if b.iter().sum::<i64>() > a.iter().sum::<i64>() {
            inc += 1;
        }
    }

    println!("{}", inc);
}
