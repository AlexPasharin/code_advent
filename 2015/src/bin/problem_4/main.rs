#![warn(clippy::all, clippy::pedantic)]
use md5::{compute, Digest};

// NOTE: run in release mode. much slower in debug mode.

fn main() {
    let input = "iwrupvqb";
    let mut counter = 1;

    let mut calc_hash_until_cond = |predicate: &(dyn Fn(Digest) -> bool)| loop {
        let hash = compute(format!("{}{}", input, counter));

        if predicate(hash) {
            return counter;
        }

        counter += 1;
    };

    println!(
        "The answer of part 1 is {}",
        calc_hash_until_cond(&starts_with_five_zeroes)
    ); // 346386

    println!(
        "The answer of part 2 is {}",
        calc_hash_until_cond(&starts_with_six_zeroes)
    ); // 9958218
}

// Digest is essentially an array [u8; 16]
fn starts_with_five_zeroes(digest: Digest) -> bool {
    let result = digest.0;

    result[0] == 0 && result[1] == 0 && result[2] < 16
}

fn starts_with_six_zeroes(digest: Digest) -> bool {
    let result = digest.0;

    result[0] == 0 && result[1] == 0 && result[2] == 0
}
