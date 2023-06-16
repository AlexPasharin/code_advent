fn main() {
    let input = "3113322113";
    // let input = "1";

    let mut input_digits: Vec<u32> = input.chars().map(|ch| ch.to_digit(10).unwrap()).collect();

    for _ in 0..40 {
        input_digits = next_game_iteration(input_digits);
    }

    let result = input_digits.len();
    println!("Part 1 answer: {}", result); //329356

    for _ in 0..10 {
        input_digits = next_game_iteration(input_digits);
    }

    let result = input_digits.len();
    println!("Part 2 answer: {}", result); //4666278
}

fn next_game_iteration(digits: Vec<u32>) -> Vec<u32> {
    let mut result = Vec::new();

    let mut curr_digit = digits[0];
    let mut counter = 0;

    for digit in digits {
        if digit == curr_digit {
            counter += 1;
        } else {
            result.push(counter);
            result.push(curr_digit);

            curr_digit = digit;
            counter = 1;
        }
    }

    result.push(counter);
    result.push(curr_digit);

    result
}
