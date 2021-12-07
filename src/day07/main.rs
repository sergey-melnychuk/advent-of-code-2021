use advent_of_code_2021::util::lines;

type Num = isize;

fn total_fuel<F>(target: Num, positions: &[Num], cost: F) -> Num
    where F: Fn(Num, Num) -> Num
{
    positions.into_iter()
        .map(|pos| cost(target, *pos))
        .sum()
}

fn cost1(target: Num, from: Num) -> Num {
    (target - from).abs()
}

fn cost2(target: Num, from: Num) -> Num {
    let steps = (target - from).abs();
    (steps + 1) * steps / 2
}

fn main() {
    let positions = lines()[0]
        .split(',')
        .into_iter()
        .map(|num| num.parse::<Num>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", positions.len());

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let (_target, fuel) = (min..=max).into_iter()
        .map(|target| (target, total_fuel(target, &positions, cost1)))
        .min_by_key(|(_, fuel)| *fuel)
        .unwrap();
    println!("{}", fuel);

    let (_target, fuel) = (min..=max).into_iter()
        .map(|target| (target, total_fuel(target, &positions, cost2)))
        .min_by_key(|(_, fuel)| *fuel)
        .unwrap();
    println!("{}", fuel);
}
