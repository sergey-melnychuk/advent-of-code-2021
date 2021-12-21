fn part1(mut tile: [usize; 2], limit: usize) -> (usize, usize) {
    let mut turn = 0usize;
    let mut dice = 0usize;
    let mut score = [0usize; 2];

    while score.iter().all(|s| *s < limit) {
        let mut thrown = 0;
        for _ in 0..3 {
            let n = (dice % 100) + 1;
            thrown += n;
            dice += 1;
        }

        let next = tile[turn % 2] + thrown;
        tile[turn % 2] = next - (next - 1) / 10 * 10;
        score[turn % 2] += tile[turn % 2];

        turn += 1;
    }

    let idx = if score[0] < 1000 { 0 } else { 1 };
    (score[idx], dice)
}

#[allow(clippy::too_many_arguments)]
fn f(
    lim: usize,
    step: usize,
    mut p1: usize,
    mut p2: usize,
    mut s1: usize,
    mut s2: usize,
    next: usize,
    acc: usize,
    end: &mut [usize],
) {
    if step % 2 == 0 {
        p1 += next;
        p1 = p1 - (p1 - 1) / 10 * 10;
        s1 += p1;
    } else {
        p2 += next;
        p2 = p2 - (p2 - 1) / 10 * 10;
        s2 += p2;
    }

    if s1 >= lim {
        end[1] += acc;
        return;
    }
    if s2 >= lim {
        end[0] += acc;
        return;
    }

    for (n, w) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        f(lim, step + 1, p1, p2, s1, s2, n, acc * w, end)
    }
}

fn part2(lim: usize, p1: usize, p2: usize) -> usize {
    let mut end = [0usize; 2];
    for (n, w) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        f(lim, 0, p1, p2, 0, 0, n, w, &mut end);
    }
    end[0].max(end[1])
}

fn main() {
    let (score, throws) = part1([8, 2], 1000);
    println!("{}", score * throws);

    let x = part2(21, 8, 2);
    println!("{}", x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1([4, 8], 1000), (745, 993));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(21, 4, 8), 444356092776315);
    }
}
