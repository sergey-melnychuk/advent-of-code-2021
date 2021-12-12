use advent_of_code_2021::util::lines;

fn main() {
    let lines = lines();
    let count = lines.len();
    let bits = lines.iter().map(|line| line.len()).max().unwrap();

    let mut ones = vec![0usize; bits];
    for line in lines.iter() {
        for (i, bit) in line.as_bytes().iter().enumerate() {
            if *bit == b'1' {
                ones[i] += 1;
            }
        }
    }

    let gamma = ones
        .iter()
        .map(|n| if *n > count / 2 { 1 } else { 0 })
        .fold(0u64, |acc, x| (acc << 1) | x);
    let epsilon = ones
        .iter()
        .map(|n| if *n < count / 2 { 1 } else { 0 })
        .fold(0u64, |acc, x| (acc << 1) | x);
    println!("{}", gamma * epsilon);

    let oxygen = search(
        &lines,
        |ones, zeros| if ones >= zeros { b'1' } else { b'0' },
    );
    let carbon = search(
        &lines,
        |ones, zeros| if ones >= zeros { b'0' } else { b'1' },
    );
    println!("{}", oxygen * carbon);
}

fn search<F>(lines: &[String], f: F) -> u64
where
    F: Fn(usize, usize) -> u8 + Copy,
{
    let mut all = lines.to_vec();
    let mut prefix: usize = 0;
    while all.len() > 1 {
        let remaining = filter(&all, prefix, f);
        all = remaining;
        prefix += 1;
    }
    assert_eq!(all.len(), 1);
    u64::from_str_radix(&all[0], 2).unwrap()
}

fn filter<F>(lines: &[String], prefix: usize, f: F) -> Vec<String>
where
    F: Fn(usize, usize) -> u8,
{
    let mut ones = 0usize;
    for line in lines {
        let bit = line.as_bytes().iter().nth(prefix).unwrap();
        if *bit == b'1' {
            ones += 1;
        }
    }

    let zeros = lines.len() - ones;
    let target = f(ones, zeros);

    lines
        .iter()
        .filter(|line| line.as_bytes().iter().nth(prefix).unwrap() == &target)
        .cloned()
        .collect()
}
