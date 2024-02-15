#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashSet;

use utils::file_reader::FileReader;

fn main() {
    let mut numbers = Vec::new();
    let mut symbol_indices = Vec::new();
    let mut gear_numbers = Vec::new();

    struct Number {
        value: u32,
        start: usize,
        end: usize,
    }

    FileReader::process_lines("./input/problem3.txt", &mut |line| {
        let mut line_numbers = Vec::new();
        let mut line_symbol_indices = HashSet::new();
        let mut line_gear_indices = HashSet::new();

        let mut reading_number = false;
        let mut number = 0;
        let mut number_start = 0;

        for (idx, char) in line.chars().enumerate() {
            match char.to_digit(10) {
                Some(digit) => {
                    number = number * 10 + digit;

                    if !reading_number {
                        number_start = idx;
                        reading_number = true;
                    }
                }
                None => {
                    if reading_number {
                        reading_number = false;

                        line_numbers.push(Number {
                            value: number,
                            start: number_start,
                            end: idx - 1,
                        });

                        number = 0;
                    }

                    if char != '.' {
                        line_symbol_indices.insert(idx);

                        if char == '*' {
                            line_gear_indices.insert(idx);
                        }
                    }
                }
            }
        }

        if reading_number {
            line_numbers.push(Number {
                value: number,
                start: number_start,
                end: line.len() - 1,
            });
        }

        numbers.push(line_numbers);
        symbol_indices.push(line_symbol_indices);
        gear_numbers.push(line_gear_indices);
    });

    let amount_of_rows = numbers.len();

    let mut part_numbers_sum = 0;

    for (line_idx, line_numbers) in numbers.iter().enumerate() {
        for Number { value, start, end } in line_numbers {
            let min_row = if line_idx > 0 { line_idx - 1 } else { line_idx };
            let max_row = if line_idx < amount_of_rows - 1 {
                line_idx + 1
            } else {
                line_idx
            };

            let min_col = if *start > 0 { *start - 1 } else { *start };
            let max_col = end + 1;

            'outer: for x in min_row..=max_row {
                for y in min_col..=max_col {
                    if symbol_indices[x].contains(&y) {
                        part_numbers_sum += value;
                        break 'outer;
                    }
                }
            }
        }
    }

    println!("Sum of all part numbers: {}", part_numbers_sum);

    let mut gear_ratio_sum = 0;

    for (line_idx, line_gears) in gear_numbers.into_iter().enumerate() {
        for gear_column_idx in line_gears {
            let min_row = if line_idx > 0 { line_idx - 1 } else { line_idx };
            let max_row = if line_idx < amount_of_rows - 1 {
                line_idx + 1
            } else {
                line_idx
            };

            let min_col = if gear_column_idx > 0 {
                gear_column_idx - 1
            } else {
                gear_column_idx
            };
            let max_col = gear_column_idx + 1;

            let mut adjacent_numbers = HashSet::new();

            for x in min_row..=max_row {
                for y in min_col..=max_col {
                    adjacent_numbers.extend(
                        numbers[x]
                            .iter()
                            .filter(|n| n.start <= y && y <= n.end)
                            .map(|n| n.value),
                    );
                }
            }

            println!("{:?}", adjacent_numbers);

            if adjacent_numbers.len() == 2 {
                gear_ratio_sum += adjacent_numbers.iter().product::<u32>();
            }
        }
    }

    println!("Sum of all gear ratios: {}", gear_ratio_sum);
}
