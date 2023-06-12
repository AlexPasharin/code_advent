use md5::{compute, Digest};

fn main() {
    let input = "iwrupvqb";
    let mut n = 1;

    loop {
        let hash = compute(format!("{}{}", input, n));

        if starts_with_five_zeroes(hash) {
            println!("The answer of part 1 is {}", n); // 346386
            break;
        }

        n += 1;
    }

    loop {
        let hash = compute(format!("{}{}", input, n));

        if starts_with_six_zeroes(hash) {
            println!("The answer of part 2 is {}", n); // 9958218
            break;
        }

        n += 1;
    }
}

fn starts_with_five_zeroes(digest: Digest) -> bool {
    let result = digest.0;

    return result[0] == 0 && result[1] == 0 && result[2] < 16;
}

fn starts_with_six_zeroes(digest: Digest) -> bool {
    let result = digest.0;

    return result[0] == 0 && result[1] == 0 && result[2] == 0;
}
