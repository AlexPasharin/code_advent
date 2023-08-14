#![warn(clippy::all, clippy::pedantic)]

fn main() {
    let input = "3113322113";

    let mut input_digits: Vec<u32> = input.chars().map(|ch| ch.to_digit(10).unwrap()).collect();

    let mut play_game = |number_of_rounds: usize| {
        for _ in 0..number_of_rounds {
            // note - we must pass input_digits by reference, even though we save their new value into input_digits at every round
            // this is because if it would be passed by value, this closure would own it, so would be possible to be called just once i.e. be FnOnce
            // we need to call it twice
            input_digits = next_game_iteration(&input_digits);
        }

        input_digits.len()
    };

    let result = play_game(40);
    println!("Part 1 answer: {}", result); //329356

    let result = play_game(10);
    println!("Part 2 answer: {}", result); //4666278
}

fn next_game_iteration(digits: &Vec<u32>) -> Vec<u32> {
    let mut result = Vec::new();

    let mut curr_digit = digits[0];
    let mut counter = 0;

    for digit in digits {
        if *digit == curr_digit {
            counter += 1;
        } else {
            result.push(counter);
            result.push(curr_digit);

            curr_digit = *digit;
            counter = 1;
        }
    }

    result.push(counter);
    result.push(curr_digit);

    result
}
