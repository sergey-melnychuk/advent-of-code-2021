use advent_of_code_2021::util::input;

fn main() {
    let input = input::<i64>();

    let mut n = 0;
    for (a, b) in input.iter().zip(input.iter().skip(1)) {
        if b > a {
            n += 1;
        }
    }
    println!("{}", n);

    n = 0;
    for (a, b) in input.windows(3).zip(input.windows(3).skip(1)) {
        if b.iter().sum::<i64>() > a.iter().sum::<i64>() {
            n += 1;
        }
    }
    println!("{}", n);
}
