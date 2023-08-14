#![warn(clippy::all, clippy::pedantic)]

use serde_json::{self, de, Value};
use std::fs::File;
use std::io::BufReader;
use utils::file_reader::FileReader;

fn main() {
    // First solution for part 1: ignore json aspect, parse input as text char by char low-level style
    let mut sum: i32 = 0;
    let mut current_number: i32 = 0;
    let mut current_number_is_negative = false;

    let mut reading_number = false;

    let file_path = "./input/problem_12.txt";

    FileReader::process_file(file_path, &mut |ch| {
        if ch == '-' {
            current_number_is_negative = true;

            return;
        }

        if let Some(digit) = ch.to_digit(10) {
            reading_number = true;

            current_number = current_number * 10 + (digit as i32);
        } else if reading_number {
            reading_number = false;

            if current_number_is_negative {
                current_number = -current_number;
                current_number_is_negative = false;
            }

            sum += current_number;
            current_number = 0;
        }
    });

    println!("Sum of numbers in given json file, solution 1: {sum}"); // 119433

    let file_reader = BufReader::new(File::open(file_path).unwrap());

    let input_as_json = de::from_reader(file_reader).unwrap();

    let mut part1_result = 0f64;
    let mut part2_result = 0f64;

    update_both_results(
        &input_as_json,
        &mut part1_result,
        &mut Some(&mut part2_result), // a convoluted silly way to avoid repetitions in the code
    );

    println!("Sum of numbers in given json file, part 1, solution 2: {part1_result}"); // 119433
    println!("Sum of numbers in given json file, part 2: {}", {
        part2_result
    }); // 68644
}

fn update_both_results(val: &Value, result1: &mut f64, result2: &mut Option<&mut f64>) {
    match val {
        Value::Number(n) => {
            let number = n.as_f64().unwrap();

            *result1 += number;

            if let Some(result_2_val) = result2 {
                **result_2_val += number;
            }
        }
        Value::Array(arr) => {
            for el in arr {
                update_both_results(el, result1, result2);
            }
        }
        Value::Object(map) => {
            let contains_red = map.values().into_iter().any(|k| k == "red");
            let mut none = None;

            for (_, value) in map {
                update_both_results(
                    value,
                    result1,
                    if contains_red { &mut none } else { result2 },
                );
            }
        }
        _ => (),
    }
}
