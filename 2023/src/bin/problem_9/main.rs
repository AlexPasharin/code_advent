#![warn(clippy::all, clippy::pedantic)]

// use regex::Regex;
use utils::file_reader::FileReader;

fn main() {
    let mut forward_history_extrapolated_values_sum = 0;
    let mut backwards_history_extrapolated_values_sum = 0;

    FileReader::process_lines("./input/problem9.txt", &mut |line| {
        let sequence = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut sequences = vec![sequence];

        loop {
            let last_sequence = sequences.last().unwrap();

            if last_sequence.iter().all(|&s| s == 0) {
                break;
            } else {
                let difference = last_sequence
                    .iter()
                    .zip(last_sequence.iter().skip(1))
                    .map(|(a, b)| b - a)
                    .collect::<Vec<_>>();

                sequences.push(difference);
            }
        }

        let mut forward_history_extrapolated_value = 0;
        let mut backwards_history_extrapolated_value = 0;

        for sequence in sequences.iter().rev().skip(1) {
            let first_sequence_value = sequence.iter().next().unwrap();
            let last_sequence_value = sequence.iter().last().unwrap();

            forward_history_extrapolated_value += last_sequence_value;
            backwards_history_extrapolated_value =
                first_sequence_value - backwards_history_extrapolated_value;
        }

        forward_history_extrapolated_values_sum += forward_history_extrapolated_value;
        backwards_history_extrapolated_values_sum += backwards_history_extrapolated_value;
    });

    println!(
        "Sum of forward history extrapolated values: {}",
        forward_history_extrapolated_values_sum
    ); // 1974913025

    println!(
        "Sum of backwards history extrapolated values: {}",
        backwards_history_extrapolated_values_sum
    );
}
