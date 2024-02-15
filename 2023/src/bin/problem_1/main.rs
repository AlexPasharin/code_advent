#![warn(clippy::all, clippy::pedantic)]

// use regex::Regex;
use utils::file_reader::FileReader;

fn main() {
    let mut sum_1 = 0;
    let mut sum_2 = 0;

    FileReader::process_lines("./input/problem1.txt", &mut |line| {
        let (first_as_digit, general_first_digit) = find_first_digits(line).unwrap();
        let (last_as_digit, general_last_digit) = find_last_digits(line).unwrap();

        sum_1 += first_as_digit * 10 + last_as_digit;
        sum_2 += general_first_digit * 10 + general_last_digit;
    });

    println!("sum_1 of all calibration values: {}", sum_1); // 56397
    println!("sum_2 of all calibration values: {}", sum_2); // 55701
}

static DIGIT_STRINGS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn find_first_digits(line: &str) -> Option<(u32, u32)> {
    let len = line.len();

    let mut first_digit_as_string = None;

    for (idx, ch) in line.chars().enumerate() {
        if ch.is_ascii_digit() {
            let digit = ch.to_digit(10).unwrap();
            let str_digit = first_digit_as_string.unwrap_or(digit);

            return Some((digit, str_digit));
        }

        if first_digit_as_string.is_none() {
            for (s, value) in DIGIT_STRINGS {
                let jx = idx + s.len();

                if jx <= len && &line[idx..jx] == s {
                    first_digit_as_string = Some(value)
                }
            }
        }
    }

    None
}

fn find_last_digits(line: &str) -> Option<(u32, u32)> {
    let len = line.len();

    let mut last_digit_as_string = None;

    for (ch, idx) in line.chars().rev().zip((0..len).rev()) {
        if ch.is_ascii_digit() {
            let digit = ch.to_digit(10).unwrap();
            let str_digit = last_digit_as_string.unwrap_or(digit);

            return Some((digit, str_digit));
        }

        if last_digit_as_string.is_none() {
            for (s, value) in DIGIT_STRINGS {
                let start_idx = idx.checked_sub(s.len() - 1);

                if let Some(start) = start_idx {
                    if &line[start..=idx] == s {
                        last_digit_as_string = Some(value);
                    }
                }
            }
        }
    }

    None
}
