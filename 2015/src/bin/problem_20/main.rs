use std::collections::HashMap;

const INPUT: u64 = 34000000;
const LIMIT_1: u64 = INPUT / 10;
const LIMIT_2: u64 = INPUT / 11;

fn main() {
    let mut prev_sums = HashMap::new();
    prev_sums.insert(1, 1);

    for r in 2.. {
        let result1 = divisors_sum(r, &mut prev_sums);

        if result1 >= LIMIT_1 {
            println!("Part 1 answer: {r}"); // 665280
            break;
        }
    }

    for r in 2.. {
        let result2 = partial_divisors_sum(r);

        if result2 >= LIMIT_2 {
            println!("Part 2 answer: {r}"); // 706500
            break;
        }
    }
}

fn divisors_sum(n: u64, prev_sums: &mut HashMap<u64, u64>) -> u64 {
    match prev_sums.get(&n) {
        Some(s) => *s,
        None => {
            let limit = (n as f64).sqrt() as u64;
            let smallest_divisor = match (2..=limit).find(|i| n % i == 0) {
                Some(i) => i,
                None => n,
            };

            let mut m = n / smallest_divisor;
            let mut power = smallest_divisor * smallest_divisor;

            while m % smallest_divisor == 0 {
                power *= smallest_divisor;
                m /= smallest_divisor;
            }

            let prev_sum = divisors_sum(m, prev_sums);
            let sum = (power - 1) / (smallest_divisor - 1) * prev_sum;

            prev_sums.insert(n, sum);

            sum
        }
    }
}

fn partial_divisors_sum(n: u64) -> u64 {
    (1..=50).filter(|d| n % d == 0).map(|d| n / d).sum()
}
