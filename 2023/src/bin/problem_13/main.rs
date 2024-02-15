#![warn(clippy::all, clippy::pedantic)]

// use regex::Regex;
use utils::file_reader::FileReader;

fn main() {
    let mut sum_of_reflection_values = 0;
    let mut sum_of_smudge_point_reflection_values = 0;
    let mut current_pattern = Vec::new();

    FileReader::process_lines("./input/problem13.txt", &mut |line| {
        if line.is_empty() {
            let (original_pattern_number, smudge_point_pattern_number) =
                smudge_point_pattern_number(&current_pattern);

            sum_of_reflection_values += original_pattern_number;
            sum_of_smudge_point_reflection_values += smudge_point_pattern_number;

            current_pattern.clear();
        } else {
            current_pattern.push(line.chars().collect::<Vec<_>>())
        }
    });

    let (original_pattern_number, smudge_point_pattern_number) =
        smudge_point_pattern_number(&current_pattern);

    sum_of_reflection_values += original_pattern_number;
    sum_of_smudge_point_reflection_values += smudge_point_pattern_number;

    println!("Sum of notes: {:?}", sum_of_reflection_values); //34993
    println!(
        "Sum of notes with smudge points corrected: {:?}",
        sum_of_smudge_point_reflection_values
    )
}

fn calculate_pattern_number(pattern: &Vec<Vec<char>>, original_number: Option<usize>) -> usize {
    let pattern_width = pattern[0].len(); // we assume all patterns are squares
    let pattern_height = pattern.len();

    for column_idx in 1..pattern_width {
        if let Some(original_number) = original_number {
            if original_number == column_idx {
                continue;
            }
        }

        let is_vertical_reflection_point = (0..pattern_height).all(|row_idx| {
            let row = &pattern[row_idx];

            let smaller_part_length = column_idx.min(pattern_width - column_idx);

            let mut left_idx = column_idx - smaller_part_length;
            let mut right_idx = column_idx + smaller_part_length - 1;

            let mut row_is_mirrored = true;

            while left_idx < right_idx {
                if row[left_idx] != row[right_idx] {
                    row_is_mirrored = false;
                    break;
                }

                left_idx += 1;
                right_idx -= 1;
            }

            row_is_mirrored
        });

        if is_vertical_reflection_point {
            return column_idx;
        }
    }

    for row_idx in 1..pattern_height {
        if let Some(original_number) = original_number {
            if original_number == row_idx * 100 {
                continue;
            }
        }
        let smaller_part_length = row_idx.min(pattern_height - row_idx);

        let mut upper_idx = row_idx - smaller_part_length;
        let mut lower_idx = row_idx + smaller_part_length - 1;

        let mut is_horizontal_reflection_point = true;

        while upper_idx < lower_idx {
            if pattern[upper_idx] != pattern[lower_idx] {
                is_horizontal_reflection_point = false;
                break;
            }

            upper_idx += 1;
            lower_idx -= 1;
        }

        if is_horizontal_reflection_point {
            return row_idx * 100;
        }
    }

    0
}

fn smudge_point_pattern_number(pattern: &Vec<Vec<char>>) -> (usize, usize) {
    let original_pattern_number = calculate_pattern_number(&pattern, None);

    let pattern_width = pattern[0].len(); // we assume all patterns are squares
    let pattern_height = pattern.len();

    for row_idx in 0..pattern_height {
        for column_idx in 0..pattern_width {
            let mut new_pattern = pattern.clone();
            let old_symbol = new_pattern[row_idx][column_idx];
            new_pattern[row_idx][column_idx] = if old_symbol == '.' { '#' } else { '.' };

            let new_pattern_number =
                calculate_pattern_number(&new_pattern, Some(original_pattern_number));
            if new_pattern_number > 0 {
                return (original_pattern_number, new_pattern_number);
            }
        }
    }

    (original_pattern_number, 0)
}
