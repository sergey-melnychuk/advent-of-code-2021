use advent_of_code_2021::util::lines;

type Num = usize;

fn day(bins: &mut [Num; 9]) {
    let ready = bins[0];
    for i in 0..bins.len()-1 {
        bins[i] = bins[i+1];
    }
    bins[6] += ready;
    bins[8] = ready;
}

fn main() {
    let numbers = lines()[0]
        .split(',')
        .into_iter()
        .map(|num| num.parse::<Num>().unwrap())
        .collect::<Vec<_>>();
    assert!(*numbers.iter().min().unwrap() >= 1);
    assert!(*numbers.iter().max().unwrap() <= 8);

    let mut bins: [Num; 9] = [0; 9];
    for num in numbers {
        bins[num] += 1;
    }

    for _ in 0..80 {
        day(&mut bins);
    }
    println!("{}", bins.iter().sum::<Num>());

    for _ in 0..256-80 {
        day(&mut bins);
    }
    println!("{}", bins.iter().sum::<Num>());
}
